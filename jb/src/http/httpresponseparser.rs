use crate::common::{bytes_to_i32, bytes_to_string, subsequence_index, I32Enum};

use super::{HttpError, HttpResponse, HttpResponseStatusCode, HttpResult};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum HttpParserState {
    #[default] Idle,
    ParsingHead,
    ParsingFields,
    ParsingBodyChunked,
    ParsingBodyLength,
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

    pub fn update(&mut self, _data: &[u8]) -> HttpParserState {
        todo!()
    }

    pub fn take_response(&mut self) -> HttpResult<HttpResponse> {
        if self.state != HttpParserState::ParsingDone || self.state != HttpParserState::ParsingBodyClosed {
            return Err(HttpError::ParsingNotDone);
        }

        Ok(std::mem::take(&mut self.response))
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
            return Err(HttpError::InvalidHeadline);
        };

        let Some(second_space) = self.buffer[(first_space + 1)..].iter().position(|ch| *ch == b' ') else {
            return Err(HttpError::InvalidHeadline);
        };

        
        self.response.version = bytes_to_string(&self.buffer[0..first_space]).map_err(|e| e.into())?;
        self.response.reason = bytes_to_string(&self.buffer[(first_space + 1)..second_space]).map_err(|e| e.into())?;
        self.response.status = HttpResponseStatusCode::from_i32(bytes_to_i32(&self.buffer[(first_space + 1)..second_space]).map_err(|e| e.into())?)
            .ok_or(HttpError::StatusUnknown)?;
        self.buffer.drain(..(idx + 2));
        self.state = HttpParserState::ParsingFields;

        Ok(true)
    }

    fn parse_fields(&mut self) -> bool {
        todo!()
    }

    fn parse_body_chunked(&mut self) -> bool {
        todo!()
    }

    fn parse_body_sized(&mut self) -> bool {
        todo!()
    }

    fn parse_body_closed(&mut self) -> bool {
        todo!()
    }
}