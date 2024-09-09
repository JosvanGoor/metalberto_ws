use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::Arc;

use jb::common::bytes_to_string;
use jb::http::{HttpMethod, HttpParserState, HttpRequest, HttpResponseParser};
use jb::net::{SslTcpStream, Uri};

fn main() {

    let stream = SslTcpStream::new(&"https://www.google.nl".try_into().unwrap());

    // rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
    // let root_store = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    // let config = Arc::new(rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth());
    
    // let uri = Uri::from("https://google.com").unwrap();
    // println!("addr: {}", format!("{}", uri.host()));
    // let addr = format!("{}", uri.host()).try_into().unwrap();
    // let mut client = rustls::ClientConnection::new(config, addr).unwrap();
    // let mut connection = rustls::Stream::new(&mut client, )

    // let mut request = HttpRequest::new();
    // let payload = request.generate(HttpMethod::Get, &uri, None);

    // connection.writer().write(payload.as_slice()).unwrap();
    // // let mut connection = TcpStream::connect(format!("{}:{}", uri.host(), uri.determine_port().unwrap())).unwrap();
    // // connection.write(payload.as_slice()).unwrap();

    // let mut buffer = [0u8; 1024];
    // let mut parser = HttpResponseParser::new();

    // let response = loop {
    //     let read = match connection.reader().read(buffer.as_mut_slice()) {
    //         Ok(num) => num,
    //         Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => continue,
    //         Err(err) => Err(err).unwrap()
    //     };

    //     if parser.update(&buffer[0..read]).unwrap() == HttpParserState::ParsingDone {
    //         break parser.take_response().unwrap();
    //     }
    // };

    // println!("Status: {:?}", response.status);
    // println!("Reason: {:?}", response.reason);
    // println!("Version: {:?}\n", response.version);

    // for (k, v) in response.fields.iter() {
    //     println!("{}: {}", k, v);
    // }

    // println!("\ncontent type: {:?}", response.content.content_type());
    // println!("content: {}", bytes_to_string(response.content.as_slice()).unwrap());
}
