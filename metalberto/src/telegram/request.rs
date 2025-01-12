use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

use jb::http::{HttpContent, HttpRequest, HttpResponse, HttpResponseParser};
use jb::json::{json_to_string, Value};
use rustls::{ClientConfig, ClientConnection, Stream};

use crate::utility::{TelegramError, TelegramResult};

pub struct Request {
    pub endpoint: String,
    pub payload:  Value,
}

impl Request {
    pub fn new(endpoint: &str, payload: Value) -> Self {
        Self { endpoint: endpoint.into(),
               payload }
    }
}

pub fn do_request(tls_config: Arc<ClientConfig>, request: Request) -> TelegramResult<Value> {
    let mut server_name = "api.telegram.org".try_into().map_err(|err| TelegramError::Dns(format!("{:?}", err)))?;
    let mut tls_client = ClientConnection::new(tls_config.clone(), server_name)?;
    let mut tcp_connection = TcpStream::connect("api.telegram.org:443")?;
    tcp_connection.set_read_timeout(Some(Duration::from_secs(1)))?;

    


    todo!()
}
