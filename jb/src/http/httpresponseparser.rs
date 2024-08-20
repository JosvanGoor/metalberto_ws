use crate::common::{bytes_to_i32, bytes_to_string, subsequence_index, I32Enum};
use std::usize;
use super::{HttpError, HttpResponse, HttpResponseStatusCode, HttpResult};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum HttpParserState {
    #[default] Idle,
    ParsingHead,
    ParsingFields,
    ParsingBodyChunked,
    ParsingBodySized,
    ParsingBodyClosed,
    ParsingDone,
}

#[derive(Debug, Default)]
pub struct HttpResponseParser {
    state: HttpParserState,
    buffer: Vec<u8>,
    size: usize,
    response: HttpResponse,
}

impl HttpResponseParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.state = HttpParserState::Idle;
        self.buffer.clear();
        self.size = 0;
        self.response = HttpResponse::default();
    }

    pub fn state(&self) -> HttpParserState {
        self.state
    }

    pub fn update(&mut self, data: &[u8]) -> HttpResult<HttpParserState> {
        self.buffer.extend_from_slice(data);

        let mut pending = true;
        while pending {
            match self.state {
                HttpParserState::Idle => self.state = HttpParserState::ParsingHead,
                HttpParserState::ParsingHead => pending = self.parse_head()?,
                HttpParserState::ParsingFields => pending = self.parse_fields()?,
                HttpParserState::ParsingBodyChunked => pending = self.parse_body_chunked()?,
                HttpParserState::ParsingBodySized => pending = self.parse_body_sized()?,
                HttpParserState::ParsingBodyClosed => pending = self.parse_body_closed()?,
                HttpParserState::ParsingDone => return Ok(HttpParserState::ParsingDone),
            }
        }
        Ok(self.state)
    }

    pub fn take_response(&mut self) -> HttpResult<HttpResponse> {
        if self.state == HttpParserState::ParsingDone || self.state == HttpParserState::ParsingBodyClosed {
            return Ok(std::mem::take(&mut self.response));
        }
        Err(HttpError::ParsingNotDone)
    }

    /*
        Private interface
    */

    fn parse_head(&mut self) -> HttpResult<bool> {
        // needs more data
        let Some(idx) = subsequence_index(0, &self.buffer, b"\r\n") else {
            return Ok(false);
        };

        // find first space
        let Some(first_space) = self.buffer.iter().position(|ch| *ch == b' ') else {
            return Err(HttpError::InvalidHeadLine);
        };

        let Some(mut second_space) = self.buffer[(first_space + 1)..].iter().position(|ch| *ch == b' ') else {
            return Err(HttpError::InvalidHeadLine);
        };
        second_space += first_space + 1; // since we take the index starting from the first space

        
        self.response.version = bytes_to_string(&self.buffer[0..first_space]).map_err(|e| e.into())?;
        self.response.reason = bytes_to_string(&self.buffer[(first_space + 1)..second_space]).map_err(|e| e.into())?;
        self.response.status = HttpResponseStatusCode::from_i32(bytes_to_i32(&self.buffer[(first_space + 1)..second_space]).map_err(|e| e.into())?)
            .ok_or(HttpError::StatusUnknown)?;
        self.buffer.drain(..(idx + 2));
        self.state = HttpParserState::ParsingFields;

        Ok(true)
    }

    fn parse_fields(&mut self) -> HttpResult<bool> {
        let Some(idx) = subsequence_index(0, &self.buffer, b"\r\n") else {
            return Ok(false);
        };

        let line = &self.buffer[0..idx];
        
        // empty line signifies end of fields
        if line.len() != 0 {
            let Some(colon) = line.iter().position(|ch| *ch == b':') else {
                return Err(HttpError::InvalidFieldLine);
            };

            self.response.fields.insert(
                bytes_to_string(&line[0..colon]).map_err(|err| err.into())?.into(),
                bytes_to_string(&line[(colon + 2)..]).map_err(|err| err.into())?
            );
            self.buffer.drain(..(idx + 2));
            return Ok(true);
        }

        self.buffer.drain(..2); // we pop the '\r\n'
        self.determine_body_type_from_fields()?;
        Ok(self.state != HttpParserState::ParsingDone)
    }

    fn determine_body_type_from_fields(&mut self) -> HttpResult<()> {
        self.response.content.update_content_type(self.response.fields.get(&"Content-Type".into()));
        if let Some(content_length) = self.response.fields.get(&"Content-Length".into()) {
            let number = content_length.parse::<usize>().map_err(|err| HttpError::ExpectedIntInHeader("Content-Length".into(), err))?;
            self.size = number;
            self.state = HttpParserState::ParsingBodySized;
            return Ok(());
        } else if let Some(chunked) = self.response.fields.get(&"Transfer-Encoding".into()) {
            if chunked.to_lowercase() == "chunked" {
                self.state = HttpParserState::ParsingBodyChunked;
                self.size = usize::MAX;
                return Ok(());
            } else {
                return Err(HttpError::InvalidTransferEncoding(chunked.clone()));
            }
        } else if let Some(close) = self.response.fields.get(&"Connection".into()) {
            if close == "close" {
                self.state = HttpParserState::ParsingBodyClosed;
                return Ok(())
            }
        }

        self.state = HttpParserState::ParsingDone;
        Ok(())
    }

    fn parse_body_chunked(&mut self) -> HttpResult<bool> {
        if self.size == usize::MAX {
            // chunk size unknown
            if let Some(line_end) = subsequence_index(0, &self.buffer, b"\r\n") {
                let size_str = bytes_to_string(&self.buffer[0..line_end]).map_err(|err| err.into())?;
                self.size = usize::from_str_radix(&size_str, 16).map_err(|err| HttpError::ChunkSizeError(err))?;
                // self.size = bytes_to_i32(&self.buffer[0..line_end]).map_err(|err| err.into())? as usize;
                self.buffer.drain(0..(line_end + 2));

                if self.size == 0 {
                    self.state = HttpParserState::ParsingDone;
                    self.size = usize::MAX;
                    return Ok(false);
                }
            }
        }

        if self.buffer.len() < self.size {
            return Ok(false);
        }

        self.response.content.extend_from_slice(&self.buffer[0..self.size]);
        self.buffer.drain(0..(self.size + 2));
        self.size = usize::MAX;
        Ok(true)
    }

    fn parse_body_sized(&mut self) -> HttpResult<bool> {
        if self.buffer.len() > self.size {
            return Err(HttpError::TooMuchDataInResponse);
        }

        if self.buffer.len() != self.size {
            return Ok(false);
        }

        self.response.content.set_content(std::mem::take(&mut self.buffer));
        self.state = HttpParserState::ParsingDone;
        Ok(false)
    }

    fn parse_body_closed(&mut self) -> HttpResult<bool> {
        self.response.content.extend_from_slice(self.buffer.as_slice());
        self.buffer.clear();
        Ok(false)
    }
}