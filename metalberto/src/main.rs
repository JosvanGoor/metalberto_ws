#![allow(dead_code, unused)]
use std::sync::Arc;

use jb::json::FromJson;
use jb::utility::StopToken;
use rustls::crypto::CryptoProvider;
use telegram::{feed, MessageType, Update, UpdateType};

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
    let (tx, rx) = std::sync::mpsc::channel::<Update>();
    let stop_token = StopToken::default();

    let join_handle = {
        let stop_token = stop_token.clone();
        let config = config.clone();

        std::thread::spawn(move || {
            feed(config, tx, api_url, stop_token).expect("Error in feed: ");
            println!("!! - Exiting Thread - !!");
        })
    };

    // join_handle.join();

    while let Ok(update) = rx.recv() {
        println!("update_id: {}", update.update_id);
        println!("{:?}", update);

        if let UpdateType::Message(message) = update.update_type {
            if let MessageType::Text { body } = message.payload {
                if body == "/stop" {
                    stop_token.request_stop();
                }
            }
        }
    }
}
