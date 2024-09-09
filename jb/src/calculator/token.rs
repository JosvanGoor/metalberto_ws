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
    ConstPi,
}

impl TokenType {
    pub fn identifier(identifier: &[u8]) -> Self {
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
            _ => TokenType::Identifier,
        }
    }

    pub fn describe(&self) -> CalculatorResult<&'static str> {
        match *self {
            TokenType::Plus => Ok("+"),
            TokenType::Minus => Ok("-"),
            TokenType::Star => Ok("*"),
            TokenType::Slash => Ok("/"),
            TokenType::Hat => Ok("^"),
            TokenType::KwSqrt => Ok("sqrt"),
            TokenType::KwPow => Ok("pow"),
            TokenType::KwSin => Ok("sin"),
            TokenType::KwCos => Ok("cos"),
            TokenType::KwTan => Ok("tan"),
            TokenType::KwLog => Ok("log"),
            TokenType::KwLn => Ok("ln"),
            TokenType::KwDeg => Ok("deg"),
            TokenType::KwRad => Ok("rad"),
            TokenType::KwExp => Ok("exp"),

            _ => Err(CalculatorError::new(0, CalculatorErrorType::TokenWithoutDisplay)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token: TokenType,
    pub literal: Option<String>,
}
