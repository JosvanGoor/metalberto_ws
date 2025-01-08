use std::sync::Arc;

mod telegram;
mod utility;

fn main() {
    let root_store = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = Arc::new(rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth());

    
}
