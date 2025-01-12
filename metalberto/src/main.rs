#![allow(dead_code, unused)]
use std::sync::Arc;

use jb::json::FromJson;
use jb::utility::StopToken;
use rustls::crypto::CryptoProvider;
use telegram::{feed, MessageType, TelegramApi, Update, UpdateType};

mod telegram;
mod utility;

#[derive(FromJson)]
struct Secret {
    bot_token: String,
}

fn main() {
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install crypto provider");
    let root_store = rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = Arc::new(rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth());
    let secret = Secret::from_json(jb::json::json_from_string(&std::fs::read_to_string("secret.json").unwrap()).unwrap()).unwrap();

    let api_url = format!("https://api.telegram.org:443/bot{}/", secret.bot_token);
    let telegram = TelegramApi::new(config.clone(), secret.bot_token).unwrap();
    telegram.spin().unwrap();
}
