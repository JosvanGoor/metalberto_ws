use std::str::Utf8Error;

use super::TokenType;

#[derive(Clone, Copy, Debug)]
pub enum CalculatorErrorType {
    ExpectedNumber,
    ExpectedMinusOrNumber,
    UnexpectedCharacter(u8),
    UtfParseError,
    InvalidBinaryOp,
    ArityError(usize, usize), // params present, expected
    InvalidCallOp,
    ExpectedPrimary(TokenType), // instead
    UnclosedParenthesis,
    NoParenthesisAfterIdentifier,
    ExpectedCommaOrClosingParenthesis(TokenType), // instead
    TokenWithoutDisplay,
}

#[derive(Copy, Clone, Debug)]
pub struct CalculatorError {
    pub caret: usize,
    pub error_type: CalculatorErrorType
}

pub type CalculatorResult<T> = Result<T, CalculatorError>;

impl CalculatorError {

    pub fn new(caret: usize, error_type: CalculatorErrorType) -> Self {
        CalculatorError { caret, error_type }
    }

}