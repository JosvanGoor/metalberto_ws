use crate::common::traits::I32Enum;
use crate::common::BytesToI32Error;
use jb_derived::I32Enum;
use std::fmt;
use std::num::ParseIntError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum HttpError {
    ParsingNotDone,
    InvalidHeadLine,
    Utf8Error(Utf8Error),
    StatusParseError(BytesToI32Error),
    StatusUnknown,
    InvalidFieldLine,
    ExpectedIntInHeader(String, ParseIntError), // field, error
    InvalidTransferEncoding(String),
    TooMuchDataInResponse,
    ChunkSizeError(ParseIntError),
}

impl From<BytesToI32Error> for HttpError {
    fn from(value: BytesToI32Error) -> Self {
        HttpError::StatusParseError(value)
    }
}

impl From<Utf8Error> for HttpError {
    fn from(value: Utf8Error) -> Self {
        HttpError::Utf8Error(value)
    }
}

pub type HttpResult<T> = Result<T, HttpError>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
       write!(f, "{}", std::convert::Into::<&str>::into(*self))
    }
}

impl From<HttpMethod> for &str {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => "GET",
            HttpMethod::Head => "HEAD",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Connect => "CONNECT",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Patch => "PATCH",
        }
    }
}

#[derive(Copy, Clone, Debug, I32Enum, Default, PartialEq, Eq)]
pub enum HttpResponseStatusCode {
    #[default]
    Uninitialized = 0,

    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}