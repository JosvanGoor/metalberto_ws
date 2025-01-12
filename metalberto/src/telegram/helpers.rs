use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;
use std::str;

use jb::http::{ApplicationJson, HttpContent, HttpMethod, HttpParserState, HttpRequest, HttpResponse, HttpResponseParser, HttpResponseStatusCode};
use jb::json::{json_from_string, json_to_string, FromJson, Value};
use jb::net::Uri;
use jb::utility::StopToken;
use rustls::{ClientConfig, ClientConnection, Stream};

use crate::utility::{TelegramError, TelegramResult};

use super::Response;

const SERVER_NAME: &str = "api.telegram.org";
const SERVER_URL: &str = "api.telegram.org:443";

pub struct TelegramResponse<T: FromJson> {
    pub keep_alive: bool,
    pub payload:    Response<T>,
}

pub fn endpoint_url(api_base_url: &str, endpoint: &str) -> TelegramResult<Uri> {
    Ok(Uri::parse(format!("{}/{}", api_base_url, endpoint))?)
}

pub fn connect(tls_config: Arc<ClientConfig>, timeout: Option<Duration>) -> TelegramResult<(ClientConnection, TcpStream)> {
    let server_name = SERVER_NAME.to_string().try_into().map_err(|err| TelegramError::Dns(format!("{:?}", err)))?;
    let tls_client = ClientConnection::new(tls_config.clone(), server_name)?;
    let tcp_connection = TcpStream::connect(SERVER_URL)?;
    tcp_connection.set_read_timeout(timeout)?;

    Ok((tls_client, tcp_connection))
}

pub fn http_request_from_string(uri: &Uri, payload: &str, keep_open: bool) -> Vec<u8> {
    let content = HttpContent::with_content(ApplicationJson, payload.into());

    let mut http_request = HttpRequest::new();
    http_request.set_field("Accept", ApplicationJson);

    if keep_open {
        http_request.set_field("Connection", "keep-alive");
    } else {
        http_request.set_field("Connection", "close");
    }

    http_request.generate(HttpMethod::Post, uri, Some(&content))
}

pub fn http_request_from_value(uri: &Uri, payload: &Value, keep_open: bool) -> Vec<u8> {
    http_request_from_string(uri, &json_to_string(payload), keep_open)
}

pub fn read_http_response(stream: &mut Stream<ClientConnection, TcpStream>, stop_token: StopToken) -> TelegramResult<HttpResponse> {
    let mut buffer = [0u8; 4096];
    let mut response_parser = HttpResponseParser::new();

    while !stop_token.stop_requested() {
        let read = stream.read(&mut buffer)?;
        if response_parser.update(&buffer[..read])? == HttpParserState::ParsingDone {
            return Ok(response_parser.take_response()?);
        }
    }

    Err(TelegramError::ConnectionClosed) // is this the right error?
}

pub fn parse_response<T: FromJson>(response: HttpResponse) -> TelegramResult<TelegramResponse<T>> {
    let keep_alive = response.fields.get(&("Connection".into())).map_or(false, |value| value == "keep-alive");
    if !matches!(response.status, HttpResponseStatusCode::Ok) {
        Err(response.status)?;
    }

    let document = str::from_utf8(response.content.as_slice())?;
    Ok(TelegramResponse::<T> { keep_alive,
                               payload: Response::<T>::from_json(json_from_string(document)?)? })
}

pub fn transact_raw<T: FromJson>(stream: &mut Stream<ClientConnection, TcpStream>, endpoint: &Uri, payload: &str, stop_token: StopToken) -> TelegramResult<TelegramResponse<T>> {
    let request = http_request_from_string(endpoint, payload, false);
    stream.write_all(&request);
    
    let response = read_http_response(stream, stop_token)?;
    parse_response(response)
}

pub fn transact<T: FromJson>(stream: &mut Stream<ClientConnection, TcpStream>, endpoint: &Uri, payload: &Value, stop_token: StopToken) -> TelegramResult<TelegramResponse<T>> {
    transact_raw(stream, endpoint, &json_to_string(payload), stop_token)
}