use std::net::TcpStream;
use std::sync::Arc;

use rustls::{ClientConfig, ClientConnection, Stream};

use super::{TelegramError, TelegramResult};


pub struct Connection {
    host: String,
    connection: ClientConnection,
    socket: TcpStream,
    // stream: Stream<'a, ClientConnection, TcpStream>,
}

impl Connection {
    pub fn new(config: Arc<ClientConfig>, url: &str) -> TelegramResult<Self> {
        let host: String = url.into();
        let uri = host.clone().try_into().map_err(|err| TelegramError::Dns(format!("{:?}", err)))?;
        let connection = ClientConnection::new(config, uri)?;
        let socket = TcpStream::connect(url)?;
        Ok(Self{host, connection, socket})
    }
}