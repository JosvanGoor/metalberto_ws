use jb::http::HttpError;
use jb::json::{JsonError, JsonMappingError};
use jb::net::UriParseError;


#[derive(Debug)]
pub enum TelegramError {
    DnsError(String),
    IoError(std::io::Error),
    RustlsError(rustls::Error),
    UriError(UriParseError),
    HttpError(HttpError),
    JsonError(JsonError),
    MappingError(JsonMappingError)
}

pub type TelegramResult<T> = std::result::Result<T, TelegramError>;

// MARK: from
impl From<std::io::Error> for TelegramError {
    fn from(value: std::io::Error) -> Self {
        TelegramError::IoError(value)
    }
}

impl From<rustls::Error> for TelegramError {
    fn from(value: rustls::Error) -> Self {
        TelegramError::RustlsError(value)
    }
}

impl From<UriParseError> for TelegramError {
    fn from(value: UriParseError) -> Self {
        TelegramError::UriError(value)
    }
}

impl From<HttpError> for TelegramError {
    fn from(value: HttpError) -> Self {
        TelegramError::HttpError(value)
    }
}

impl From<JsonError> for TelegramError {
    fn from(value: JsonError) -> Self {
        TelegramError::JsonError(value)
    }
}

impl From<JsonMappingError> for TelegramError {
    fn from(value: JsonMappingError) -> Self {
        TelegramError::MappingError(value)
    }
}