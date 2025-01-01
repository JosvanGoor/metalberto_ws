use core::str;

use super::{CalculatorError, CalculatorErrorType, CalculatorResult, Token, TokenType};

pub struct Tokenizer<'src> {
    caret: usize,
    expr:  &'src [u8],
}

impl<'src> Tokenizer<'src> {
    pub fn new(expr: &'src str) -> Self {
        Self { caret: 0,
               expr:  expr.as_bytes(), }
    }

    pub fn tokenize(&mut self) -> CalculatorResult<Vec<Token>> {
        let mut tokens = Vec::new();

        self.skip_whitespace();

        while !self.eof() {
            match self.peek() {
                b'+' => {
                    tokens.push(self.simple_token(TokenType::Plus));
                }
                b'-' => tokens.push(self.simple_token(TokenType::Minus)),
                b'*' => tokens.push(self.simple_token(TokenType::Star)),
                b'/' => tokens.push(self.simple_token(TokenType::Slash)),
                b'^' => tokens.push(self.simple_token(TokenType::Hat)),
                b'(' => tokens.push(self.simple_token(TokenType::LeftParen)),
                b')' => tokens.push(self.simple_token(TokenType::RightParen)),
                b',' => tokens.push(self.simple_token(TokenType::Comma)),
                b'0'..=b'9' => tokens.push(self.number()?),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => tokens.push(self.identifier()?),

                _ => return Err(CalculatorError::new(self.caret, CalculatorErrorType::UnexpectedCharacter(self.peek() as char))),
            }

            self.skip_whitespace();
        }

        tokens.push(self.simple_token(TokenType::EndOfLine));
        Ok(tokens)
    }

    fn eof(&self) -> bool {
        self.caret >= self.expr.len()
    }

    fn advance(&mut self) -> u8 {
        if self.eof() {
            return b'\0';
        }

        let ch = self.expr[self.caret];
        self.caret += 1;
        ch
    }

    fn peek(&self) -> u8 {
        if self.eof() {
            return b'\0';
        }
        self.expr[self.caret]
    }

    fn simple_token(&mut self, token: TokenType) -> Token {
        self.advance();
        Token { token,
                literal: None }
    }

    fn complex_token(&self, token: TokenType, start: usize) -> CalculatorResult<Token> {
        let Ok(literal) = str::from_utf8(&self.expr[start..self.caret]) else {
            return Err(CalculatorError::new(self.caret, CalculatorErrorType::UtfParseError));
        };

        Ok(Token { token,
                   literal: Some(String::from(literal)) })
    }

    fn number(&mut self) -> CalculatorResult<Token> {
        let start = self.caret;

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' {
            self.advance();

            if !self.peek().is_ascii_digit() {
                return Err(CalculatorError::new(self.caret, CalculatorErrorType::ExpectedNumber));
            }

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        if self.peek() == b'e' || self.peek() == b'E' {
            self.advance();

            if self.peek() == b'-' {
                self.advance(); // we allow for negative exponent
            }

            if !self.peek().is_ascii_digit() {
                return Err(CalculatorError::new(self.caret, CalculatorErrorType::ExpectedMinusOrNumber));
            }

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.complex_token(TokenType::Number, start)
    }

    fn identifier(&mut self) -> CalculatorResult<Token> {
        let start = self.caret;

        while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
            self.advance();
        }

        self.complex_token(TokenType::identifier(&self.expr[start..self.caret]), start)
    }

    fn skip_whitespace(&mut self) {
        while let b' ' | b'\r' | b'\n' | b'\t' = self.peek() {
            self.advance();
        }
    }
}
