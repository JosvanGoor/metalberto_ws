use std::net::TcpStream;
use std::sync::Arc;

use rustls::client::ClientConnection;
use rustls::pki_types::ServerName;

use super::Uri;

pub struct SslTcpStream {
    stream: TcpStream,
    tls:    ClientConnection,
}

impl SslTcpStream {
    pub fn new(uri: &Uri) -> Self {
        rustls::crypto::ring::default_provider().install_default().unwrap();
        let root_store = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let config = Arc::new(rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth());

        let server_name = ServerName::try_from(uri.host().clone()).expect("Invalid DNS Name");
        let mut stream = TcpStream::connect(format!("{}:{}", uri.host(), uri.determine_port().unwrap())).unwrap();
        let mut client_connection = ClientConnection::new(config, server_name).unwrap();
        client_connection.complete_io(&mut stream).unwrap();

        println!("is_handshaking: {}", client_connection.is_handshaking());

        Self { stream,
               tls: client_connection }
    }
}
