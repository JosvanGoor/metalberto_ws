use std::net::TcpStream;
use std::io::prelude::*;

use jb::common::bytes_to_string;
use jb::http::{HttpMethod, HttpParserState, HttpRequest, HttpResponseParser};
use jb::net::Uri;



fn main() {
    let uri = Uri::from("http://anglesharp.azurewebsites.net/Chunked").unwrap();
    let mut request = HttpRequest::new();
    let payload = request.generate(HttpMethod::Get, &uri, None);

    let mut connection = TcpStream::connect(format!("{}:{}", uri.host(), uri.determine_port().unwrap())).unwrap();
    connection.write(payload.as_slice()).unwrap();

    let mut buffer = [0u8; 1024];
    let mut parser = HttpResponseParser::new();

    let response = loop {
        let read = connection.read(buffer.as_mut_slice()).unwrap();
        
        if parser.update(&buffer[0..read]).unwrap() == HttpParserState::ParsingDone {
            break parser.take_response().unwrap();
        }
    };

    println!("Status: {:?}", response.status);
    println!("Reason: {:?}", response.reason);
    println!("Version: {:?}\n", response.version);
    
    for (k, v) in response.fields.iter() {
        println!("{}: {}", k, v);
    }

    println!("\ncontent type: {:?}", response.content.content_type());
    println!("content: {}", bytes_to_string(response.content.as_slice()).unwrap());
}