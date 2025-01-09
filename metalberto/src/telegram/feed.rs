use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::{mpsc, Arc};
use std::time::Duration;

use jb::common::AnyCase;
use jb::http::{HttpContent, HttpMethod, HttpParserState, HttpRequest, HttpResponseParser, HttpResponseStatusCode};
use jb::json::{json_from_string, FromJson};
use jb::net::Uri;
use jb::utility::StopToken;
use rustls::{ClientConfig, ClientConnection, Stream};

use super::{Response, Update};
use crate::utility::{TelegramError, TelegramResult};

struct FeedState {
    tls_config:           Arc<ClientConfig>,
    output:               mpsc::Sender<Update>,
    api_url:              String,
    stop_token:           StopToken,
    http_request:         HttpRequest,
    http_response_parser: HttpResponseParser,
    update_id:            i64,
}

fn communicate(state: &mut FeedState,
               stream: &mut Stream<ClientConnection, TcpStream>,
               uri: &Uri)
               -> TelegramResult<()> {
    state.http_response_parser.reset();
    let payload = format!("{{\"offset\": {}, \"timeout\": {}", state.update_id, 40);

    state.http_request.set_field("Connection", "keep-alive");
    state.http_request.set_field("Accept", "application/json");
    let content = HttpContent::with_content("application/json", payload.into_bytes());

    stream.write_all(&state.http_request.generate(HttpMethod::Post, uri, Some(&content)))?;
    let mut buffer: [u8; 1024] = [0; 1024];

    while !state.stop_token.stop_requested() {
        let read = stream.read(&mut buffer)?;
        state.http_response_parser.update(&buffer[..read])?;

        if state.http_response_parser.state() == HttpParserState::ParsingDone {
            let response = state.http_response_parser.take_response()?;
            let must_close = response.fields.get(&("Connection".into())).cloned().unwrap_or("close".into()) == "close";
            println!("Telegram header response - Connection: {}",
                     response.fields.get(&("Connection".into())).cloned().unwrap_or("-".into()));

            if !matches!(response.status, HttpResponseStatusCode::Ok) {
                Err(response.status)?
            };

            let document = str::from_utf8(response.content.as_slice()).expect("[feed] Utf8!");
            let parsed = json_from_string(document)?;
            let response = Response::from_json(parsed)?;

            if !response.ok {
                return Err(TelegramError::TelegramResponse(response.description.unwrap_or("No reason given".into())));
            }

            for update in response.result.into_iter() {
                if update.update_id >= state.update_id {
                    state.update_id = update.update_id + 1;
                }
                state.output.send(update);
            }

            if must_close {
                return Err(TelegramError::ConnectionClosed);
            }
            break;
        }
    }

    Ok(())
}

pub fn feed(tls_config: Arc<ClientConfig>,
            output: mpsc::Sender<Update>,
            api_url: String,
            stop_token: StopToken)
            -> TelegramResult<()> {
    let mut state = FeedState { tls_config,
                                output,
                                api_url,
                                stop_token,
                                http_request: HttpRequest::new(),
                                http_response_parser: HttpResponseParser::new(),
                                update_id: 0 };

    while !state.stop_token.stop_requested() {
        println!("[feed] Establishing connection...");
        let mut server_name = "api.telegram.org".try_into().unwrap();
        let mut tls_client = ClientConnection::new(state.tls_config.clone(), server_name)?;
        let mut tcp_connection = TcpStream::connect("149.154.167.220:443")?;
        tcp_connection.set_read_timeout(Some(Duration::from_secs(45)))?;

        let mut stream = Stream::new(&mut tls_client, &mut tcp_connection);
        let string_uri = format!("{}{}", state.api_url, "getUpdates");
        let uri = Uri::parse(string_uri)?;

        while !state.stop_token.stop_requested() {
            println!("[feed] Communicate");
            match communicate(&mut state, &mut stream, &uri) {
                Ok(()) => {}
                Err(error) => match error {
                    TelegramError::Io(error) => match error.kind() {
                        std::io::ErrorKind::NotConnected | std::io::ErrorKind::TimedOut => {
                            println!("NotConnected / TimedOut, sleeping for 60 seconds");
                            std::thread::sleep(Duration::from_secs(60));
                            break;
                        }
                        _ => return Err(TelegramError::Io(error)),
                    },
                    TelegramError::HttpResponseCode(http_response_status_code) => match http_response_status_code {
                        HttpResponseStatusCode::Ok => continue,
                        HttpResponseStatusCode::Conflict |
                        HttpResponseStatusCode::TooManyRequests |
                        HttpResponseStatusCode::InternalServerError |
                        HttpResponseStatusCode::BadGateway => {
                            println!("{:?}, sleeping for 60 seconds", http_response_status_code);
                            std::thread::sleep(Duration::from_secs(60));
                            break;
                        }
                        _ => panic!("Unacceptable http response: {:?}", http_response_status_code),
                    },
                    TelegramError::TelegramResponse(response) => {
                        println!("Telegram response not ok: {}", response);
                        println!("Sleeping for 60 seconds");
                        std::thread::sleep(Duration::from_secs(60));
                        break;
                    }
                    TelegramError::ConnectionClosed => {
                        println!("Telegram requested connection close");
                        break;
                    }
                    _ => panic!("Unacceptable error: {:?}", error),
                },
            }
        }
    }

    Ok(())
}
