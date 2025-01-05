use std::collections::HashMap;
use std::str;

use super::error::JsonErrorType;
use super::{JsonError, JsonResult, Value};

struct Parser<'a> {
    line:     usize,
    caret:    usize,
    document: &'a [u8],
}

impl Parser<'_> {
    // constructor
    fn new(document: &'_ str) -> Parser<'_> {
        Parser { line:     0,
                 caret:    0,
                 document: document.as_bytes(), }
    }

    // entry function
    fn parse(&mut self) -> JsonResult<Value> {
        self.skip_whitespace()?;

        // println!("entering parse, seeing: '{}'", char::from(self.peek()?));
        match self.peek()? {
            b'{' => self.dict(),
            b'[' => self.array(),
            b'"' => Ok(Value::String(self.string()?)),
            b't' => {
                self.word(b"true")?;
                Ok(Value::Boolean(true))
            }
            b'f' => {
                self.word(b"false")?;
                Ok(Value::Boolean(false))
            }
            b'n' => {
                self.word(b"null")?;
                Ok(Value::Null)
            }
            _ => self.number(),
        }
    }

    // specific parsers
    fn array(&mut self) -> JsonResult<Value> {
        self.advance()?;
        let mut array: Vec<Value> = Vec::new();

        loop {
            self.skip_whitespace()?;

            if self.check(b']')? {
                return Ok(Value::Array(array));
            }

            array.push(self.parse()?);

            self.skip_whitespace()?;
            if self.peek()? != b']' && !self.check(b',')? {
                return Err(self.error(JsonErrorType::ExpectedArrayCloseOrComma));
            }
        }
    }

    fn number(&mut self) -> JsonResult<Value> {
        let start = self.caret;
        self.check(b'-')?;

        if self.peek()? == b'0' {
            self.advance()?;
        } else {
            while self.peek()?.is_ascii_digit() {
                self.advance()?;
            }
        }

        if !self.check(b'.')? {
            // no dot so integer
            let as_str = str::from_utf8(&self.document[start..self.caret]).unwrap();
            return Ok(Value::Integer(as_str.parse().unwrap()));
        }

        while self.peek()?.is_ascii_digit() {
            self.advance()?;
        }

        if self.check(b'e')? || self.check(b'E')? {
            while self.peek()?.is_ascii_digit() {
                self.advance()?;
            }
        }

        let as_str = str::from_utf8(&self.document[start..self.caret]).unwrap();
        Ok(Value::Float(as_str.parse().unwrap()))
    }

    fn dict(&mut self) -> JsonResult<Value> {
        self.advance()?; // skip '{'
        let mut dict: HashMap<String, Value> = HashMap::new();

        loop {
            self.skip_whitespace()?;

            if self.check(b'}')? {
                return Ok(Value::Dict(dict));
            }

            if !self.peek()? == b'"' {
                return Err(self.error(JsonErrorType::ExpectedDictKey));
            }

            let key = self.string()?;

            self.skip_whitespace()?;

            if !self.check(b':')? {
                return Err(self.error(JsonErrorType::ExpectedDictColonAfterKey(key)));
            }

            self.skip_whitespace()?;
            dict.insert(key, self.parse()?);
            self.skip_whitespace()?;

            if self.peek()? != b'}' && !self.check(b',')? {
                return Err(self.error(JsonErrorType::ExpectedDictCloseOrComma));
            }
        }
    }

    fn string(&mut self) -> JsonResult<String> {
        self.advance()?;
        let start = self.caret;

        loop {
            if self.check(b'"')? {
                // this can probably be from_utf8_unchecked but what do I know, lets leave unsafe for what it
                // is for now
                let string = String::from_utf8(self.document[start..(self.caret - 1)].to_vec()).unwrap();
                // println!("Parsed string: '{}'", string);
                return Ok(string);
            }
            self.check(b'\\')?;
            self.caret += 1;
        }
    }

    fn word(&mut self, characters: &[u8]) -> JsonResult<()> {
        for char in characters.iter() {
            if self.advance()? != *char {
                return Err(self.error(JsonErrorType::UnknownKeyword(String::from_utf8(characters.to_vec()).unwrap())));
            }
        }

        // println!("parsed keyword!");
        Ok(())
    }

    // utility
    fn advance(&mut self) -> JsonResult<u8> {
        let ch = self.peek()?;
        self.caret += 1;
        Ok(ch)
    }

    fn peek(&self) -> JsonResult<u8> {
        if self.caret >= self.document.len() {
            return Err(self.error(JsonErrorType::UnexpectedEndOfFile));
        }
        // println!(" peek: i: {:03}, {}", self.caret, char::from(self.document[self.caret]));
        Ok(self.document[self.caret])
    }

    fn check(&mut self, expected: u8) -> JsonResult<bool> {
        // println!("check: i: {}, '{}' (?: '{}')", self.caret, char::from(self.document[self.caret]), char::from(expected));
        if self.peek()? != expected {
            return Ok(false);
        }

        self.advance()?;
        Ok(true)
    }

    fn error(&self, error: JsonErrorType) -> JsonError {
        JsonError { line: self.line,
                error }
    }

    fn skip_whitespace(&mut self) -> JsonResult<()> {
        loop {
            match self.peek()? {
                b' ' => self.caret += 1,
                b'\t' => self.caret += 1,
                b'\n' => {
                    self.caret += 1;
                    self.line += 1
                }
                _ => break,
            }
        }
        Ok(())
    }
}

//
//  Public interface
//
#[allow(dead_code)]
pub fn json_from_string(document: &'_ str) -> JsonResult<Value> {
    let mut parser = Parser::new(document);
    parser.parse()
}
