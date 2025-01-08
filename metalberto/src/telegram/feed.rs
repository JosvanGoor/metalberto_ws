use std::str;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{mpsc, Arc};
use std::time::Duration;

use jb::http::{HttpContent, HttpMethod, HttpParserState, HttpRequest, HttpResponseParser, HttpResponseStatusCode};
use jb::json::{json_from_string, FromJson};
use jb::net::Uri;
use jb::utility::StopToken;
use rustls::{ClientConfig, ClientConnection, Stream};

use crate::utility::{TelegramError, TelegramResult};

use super::Update;

pub fn feed(tls_config: Arc<ClientConfig>, output: mpsc::Sender<Update>, api_url: String, stop_token: StopToken) -> TelegramResult<()> {
    let mut http_request = HttpRequest::new();
    let mut http_response_parser = HttpResponseParser::new();
    
    'connection: while !stop_token.stop_requested() {
        // first we create the connection
        let server_name = api_url.clone().try_into().map_err(|err| TelegramError::DnsError(format!("{:?}", err)))?;
        let mut tls_client = ClientConnection::new(tls_config.clone(), server_name)?;
        let mut tcp_connection = TcpStream::connect(api_url.clone())?;
        tcp_connection.set_read_timeout(Some(Duration::from_secs(45)))?;
        
        let mut stream = Stream::new(&mut tls_client, &mut tcp_connection);
        let mut update_id = 0u64;

        let uri = Uri::parse(format!("{}{}", api_url, "getUpdates"))?;

        // now we perform requests as long as we need / are allowed to
        while !stop_token.stop_requested() {
            http_response_parser.reset();
            let payload = format!("{{\"offset\": {}, \"timeout\": {}", update_id, 40);
            
            http_request.set_field("Connection", "keep");
            http_request.set_field("Accept", "application/json");
            let content = HttpContent::with_content("application/json", payload.into_bytes());

            stream.write_all(&http_request.generate(HttpMethod::Post, &uri, Some(&content)))?;
            let mut buffer: [u8; 1024] = [0; 1024];

            while !stop_token.stop_requested() {
                match stream.read(&mut buffer) {
                    Ok(read) => { http_response_parser.update(&buffer[..read])?; },
                    Err(error) => {
                        match error.kind() {
                            std::io::ErrorKind::TimedOut => {
                                println!("[feed] Connection timed out!");
                                break 'connection;
                            }
                            _ => return Err(TelegramError::IoError(error))
                        }
                    },
                }

                if http_response_parser.state() == HttpParserState::ParsingDone {
                    let response = http_response_parser.take_response()?;
                    
                    match response.status {
                        HttpResponseStatusCode::Ok => { /* do nothing! */ },
                        HttpResponseStatusCode::BadGateway | HttpResponseStatusCode::TooManyRequests => {
                            println!("[feed] {:?}, timeout 1 minute", response.status);
                            std::thread::sleep(Duration::from_secs(60));
                            break 'connection;
                        },
                        _ => {
                            println!("[feed] Http error: {:?}", response.status);
                            break 'connection;
                        }
                    }

                    let document = str::from_utf8(response.content.as_slice()).expect("[feed] Utf8!");
                    let parsed = json_from_string(document)?;

                    let ok: bool = *parsed.borrow_dict()?.get("ok").expect("Invalid document").borrow_boolean()?;
                    // let updates: Vec<Update> = Vec::<Update>::from_json(parsed.get("results").unwrap().borrow_dict());
                    
                }
            }

        }
    }
    
    Ok(())
}