use jb::http::{HttpError, HttpResponseStatusCode};
use jb::json::{JsonError, JsonMappingError};
use jb::net::UriParseError;


#[derive(Debug)]
pub enum TelegramError {
    Dns(String),
    Io(std::io::Error),
    Rustls(rustls::Error),
    Uri(UriParseError),
    Http(HttpError),
    Json(JsonError),
    Mapping(JsonMappingError),
    HttpResponseCode(HttpResponseStatusCode),
    TelegramResponse(String), // error description
    ConnectionClosed,
}

pub type TelegramResult<T> = std::result::Result<T, TelegramError>;

// MARK: from
impl From<std::io::Error> for TelegramError {
    fn from(value: std::io::Error) -> Self {
        TelegramError::Io(value)
    }
}

impl From<rustls::Error> for TelegramError {
    fn from(value: rustls::Error) -> Self {
        TelegramError::Rustls(value)
    }
}

impl From<UriParseError> for TelegramError {
    fn from(value: UriParseError) -> Self {
        TelegramError::Uri(value)
    }
}

impl From<HttpError> for TelegramError {
    fn from(value: HttpError) -> Self {
        TelegramError::Http(value)
    }
}

impl From<JsonError> for TelegramError {
    fn from(value: JsonError) -> Self {
        TelegramError::Json(value)
    }
}

impl From<JsonMappingError> for TelegramError {
    fn from(value: JsonMappingError) -> Self {
        TelegramError::Mapping(value)
    }
}

impl From<HttpResponseStatusCode> for TelegramError {
    fn from(value: HttpResponseStatusCode) -> Self {
        TelegramError::HttpResponseCode(value)
    }
}