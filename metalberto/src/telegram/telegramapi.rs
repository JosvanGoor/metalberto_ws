use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
use std::time::Duration;

use jb::common::AnyCase;
use jb::http::{ApplicationJson, HttpContent, HttpMethod, HttpParserState, HttpRequest, HttpResponse, HttpResponseParser, HttpResponseStatusCode};
use jb::json::{json_from_string, json_to_string, FromJson, Value};
use jb::net::Uri;
use jb::utility::StopToken;
use rustls::{ClientConfig, ClientConnection, Stream};

use super::{ChatMember, Message, TelegramResponse, Update, User};
use crate::telegram::{helpers, Response};
use crate::utility::{TelegramError, TelegramResult};

pub struct TelegramApi {
    // feed thread
    stop_token:       StopToken,
    feed_join_handle: JoinHandle<TelegramResult<()>>,
    feed_input:       mpsc::Receiver<Update>,

    // connection config
    api_base_url: String,
    tls_config:   Arc<ClientConfig>,

    // runtime information
    channel_rights: HashMap<i64, ChatMember>,
    bot_user_info: User,
}

impl TelegramApi {
    pub fn new(tls_config: Arc<ClientConfig>, bot_token: String) -> TelegramResult<Self> {
        let api_base_url = format!("https://api.telegram.org/bot{}", bot_token);
        let stop_token = StopToken::default();
        let (feed_output, feed_input) = mpsc::channel::<Update>();

        let feed_join_handle = {
            let api_url = api_base_url.clone();
            let stop_token = stop_token.clone();
            let tls_config = tls_config.clone();

            std::thread::spawn(move || {
                let result = feed(tls_config, feed_output, &api_url, stop_token);
                println!("thread result: {:?}", result);
                result
            })
        };

        let mut api = Self { stop_token,
            feed_join_handle,
            feed_input,
            api_base_url,
            tls_config,
            channel_rights: HashMap::new(),
            bot_user_info: Default::default() };
        api.load_bot_user_info()?;
        println!("Bot info: {:?}", api.bot_user_info);
        Ok(api)
    }

    // MARK: Spin
    pub fn spin(&self) -> TelegramResult<()> {
        while let Ok(update) = self.feed_input.recv() {
            match update.update_type {
                crate::telegram::UpdateType::Message(message) => {
                    if let crate::telegram::MessageType::Text { body, } = message.payload {
                        let sent_message = self.send_message(message.chat.id, body, false)?;
                        println!("{:?}", sent_message);
                    };
                }
                crate::telegram::UpdateType::MessageEdit(message) => todo!(),
                _ => (),
            }
        }

        println!("Input channel closed, exiting...");
        std::thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    // MARK: utility
    pub fn transact<T: FromJson>(&self, endpoint: &str, payload: &Value) -> TelegramResult<T> {
        let (mut tls_client, mut tcp_connection) = helpers::connect(self.tls_config.clone(), Some(Duration::from_secs(1)))?;
        let mut stream = Stream::new(&mut tls_client, &mut tcp_connection);
        let uri = helpers::endpoint_url(&self.api_base_url, endpoint)?;
        let response = helpers::transact::<T>(&mut stream, &uri, payload, self.stop_token.clone())?.payload;
        if !response.ok {
            return Err(TelegramError::TelegramResponse(response.description.unwrap_or("No description".into())));
        }
        Ok(response.result)
    }

    // this is a special case, since it requires mutable but also since it performs get and all other methods
    // perform post
    fn load_bot_user_info(&mut self) -> TelegramResult<()> {
        let (mut tls_client, mut tcp_connection) = helpers::connect(self.tls_config.clone(), Some(Duration::from_secs(1)))?;
        let mut stream = Stream::new(&mut tls_client, &mut tcp_connection);
        let uri = helpers::endpoint_url(&self.api_base_url, "getMe")?;

        let mut http_request = HttpRequest::new();
        http_request.set_field("Accept", ApplicationJson);
        http_request.set_field("Connection", "close");
        stream.write_all(&http_request.generate(HttpMethod::Get, &uri, None))?;
        let response = helpers::read_http_response(&mut stream, self.stop_token.clone())?;
        let response = helpers::parse_response::<User>(response)?.payload;

        if !response.ok {
            return Err(TelegramError::TelegramResponse(response.description.unwrap_or("No description".into())));
        }

        self.bot_user_info = response.result;
        Ok(())
    }

    // MARK: Api calls
    pub fn send_message(&self, chat_id: i64, text: String, markdown_formatting: bool) -> TelegramResult<Message> {
        let mut object = HashMap::<String, Value>::new();
        object.insert("chat_id".into(), chat_id.into());
        object.insert("text".into(), text.into());
        if markdown_formatting {
            object.insert("parse_mode".into(), "MarkdownV2".to_string().into());
        }

        self.transact::<Message>("sendMessage", &Value::Dict(object))
    }
}

// MARK: Feed
fn feed(tls_config: Arc<ClientConfig>, output: mpsc::Sender<Update>, api_url: &str, stop_token: StopToken) -> TelegramResult<()> {
    println!("Feed thread starting...");
    let uri = helpers::endpoint_url(api_url, "getUpdates")?;
    let mut update_id = 0i64;

    while !stop_token.stop_requested() {
        println!("[feed] opening connection");
        let (mut tls_client, mut tcp_connection) = helpers::connect(tls_config.clone(), Some(Duration::from_secs(45)))?;
        let mut stream = Stream::new(&mut tls_client, &mut tcp_connection);

        while !stop_token.stop_requested() {
            let raw_request = format!("{{\"offset\": {}, \"timeout\": {}}}", update_id, 40);
            let request = helpers::http_request_from_string(&uri, &raw_request, true);
            stream.write_all(&request)?;

            let response = helpers::read_http_response(&mut stream, stop_token.clone())?;
            let result = helpers::parse_response::<Vec<Update>>(response)?;

            if !result.payload.ok {
                return Err(TelegramError::TelegramResponse(result.payload.description.unwrap_or("No description".into())));
            }

            for update in result.payload.result.into_iter() {
                update_id = update_id.max(update.update_id + 1);
                output.send(update);
            }

            if !result.keep_alive {
                println!("Server asking to close connection");
                break;
            }
        }
    }

    println!("ending feed thread");
    Ok(())
}
