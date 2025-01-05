use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum JsonErrorType {
    ExpectedArrayCloseOrComma,
    ExpectedDictKey,
    ExpectedDictColonAfterKey(String), // key
    ExpectedDictCloseOrComma,
    UnexpectedEndOfFile,
    UnknownKeyword(String), // keyword
    InvalidTypeCoercion,
}

#[derive(Debug)]
pub struct JsonError {
    pub line:  usize,
    pub error: JsonErrorType,
}

pub type JsonResult<T> = std::result::Result<T, JsonError>;

impl std::error::Error for JsonError {}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?} on line {}", self.error, self.line)
    }
}

#[derive(Debug)]
pub enum JsonMappingError {
    TypeMismatch,
    FieldError(String),
}