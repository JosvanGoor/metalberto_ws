use core::str;

use super::{CalculatorError, CalculatorErrorType, CalculatorResult};


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    Error,

    Number,
    Identifier,
    EndOfLine,

    Plus,
    Minus,
    Star,
    Slash,
    Hat,
    LeftParen,
    RightParen,
    Comma,

    KwSqrt,
    KwPow,
    KwSin,
    KwCos,
    KwTan,
    KwLog,
    KwLn,
    KwDeg,
    KwRad,
    KwExp,

    ConstE,
    ConstPi
}

impl TokenType {
    pub fn from_identifier(identifier: &[u8]) -> Self {
        match identifier {
            b"sqrt" => TokenType::KwSqrt,
            b"pow" => TokenType::KwPow,
            b"sin" => TokenType::KwSin,
            b"cos" => TokenType::KwCos,
            b"tan" => TokenType::KwTan,
            b"log" => TokenType::KwLog,
            b"ln" => TokenType::KwLn,
            b"deg" => TokenType::KwDeg,
            b"rad" => TokenType::KwRad,
            b"exp" => TokenType::KwExp,

            b"e" => TokenType::ConstE,
            b"pi" => TokenType::ConstPi,
            _=> TokenType::Identifier
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token: TokenType,
    pub literal: Option<String>
}




