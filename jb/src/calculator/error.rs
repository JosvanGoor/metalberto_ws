use super::TokenType;

#[derive(Clone, Copy, Debug)]
pub enum CalculatorErrorType {
    ExpectedNumber,
    ExpectedMinusOrNumber,
    UnexpectedCharacter(char),
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
    pub error_type: CalculatorErrorType,
}

impl CalculatorError {
    pub fn new(caret: usize, error_type: CalculatorErrorType) -> Self {
        CalculatorError {
            caret,
            error_type,
        }
    }
}

pub type CalculatorResult<T> = Result<T, CalculatorError>;
