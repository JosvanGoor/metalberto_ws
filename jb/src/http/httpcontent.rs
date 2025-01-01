use core::str;

use crate::random::Lcg;

pub const ApplicationJson: &str = "application/json";

const MultipartFormData: &str = "multipart/form-data; boundary=";
const MultipartDelimiterSize: usize = 20;
const MultipartCharacterSelection: &str = "0123456789abcdefghijklmnopqrstuvwABCDEFGHIJKLMNOPQRSTUVW";
const MultipartPrefix: &str = "Content-Disposition: form-data";
const MultipartNamePrefix: &str = "; name=";
const MultipartFilenamePrefix: &str = "; filename=";

#[derive(Clone, Debug, Default)]
pub struct HttpContent {
    content_type: String,
    content: Vec<u8>,
    multipart_delimiter: Option<String>,
}

impl HttpContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(content_type: String, content: Vec<u8>) -> Self {
        Self {
            content_type: content_type,
            content: content,
            multipart_delimiter: None,
        }
    }

    pub fn content_length(&self) -> usize {
        self.content.len()
    }

    pub fn content_type(&self) -> &String {
        &self.content_type
    }

    pub fn as_slice(&self) -> &[u8] {
        self.content.as_slice()
    }

    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.content.extend_from_slice(data);
    }

    pub fn update_content_type(&mut self, content_type: Option<&String>) {
        match content_type {
            Some(content_type) => self.content_type = content_type.clone(),
            None => (),
        }
    }

    pub fn set_content(&mut self, content: Vec<u8>) {
        self.content = content;
    }

    pub fn write_multipart_data(&mut self, name: Option<String>, filename: Option<String>, content_type: Option<String>, data: Vec<u8>, close_body: bool) {
        self.write_multipart_header(name, filename, content_type);
        self.content.extend_from_slice(data.as_slice());

        if close_body {
            self.content.extend_from_slice(self.multipart_delimiter.as_ref().unwrap().as_bytes());
            self.content.extend_from_slice("--\r\n".as_bytes());
        }
    }

    fn generate_delimiter(&mut self) {
        if self.multipart_delimiter.is_some() {
            return;
        }

        let mut random = Lcg::default();
        let selection = MultipartCharacterSelection.as_bytes();
        let max_idx = selection.len() as u32;
        let mut buffer: [u8; 2 + MultipartDelimiterSize] = [0; 2 + MultipartDelimiterSize];

        for it in buffer.iter_mut() {
            *it = selection[(random.generate() % max_idx) as usize];
        }
        buffer[0] = b'-';
        buffer[1] = b'-';

        self.multipart_delimiter = Some(String::from(str::from_utf8(&buffer).unwrap()));
        self.content_type = format!("{}{}", MultipartFormData, String::from(str::from_utf8(&buffer[2..]).unwrap()));
        println!("Delimiter: {:?}", self.multipart_delimiter);
    }

    fn write_multipart_header(&mut self, name: Option<String>, filename: Option<String>, content_type: Option<String>) {
        self.generate_delimiter();
        self.content.extend_from_slice(MultipartFilenamePrefix.as_bytes());

        if let Some(name) = name {
            self.content.extend_from_slice(format!("{}\"{}\"", MultipartNamePrefix, name).as_bytes());
        };
        if let Some(filename) = filename {
            self.content.extend_from_slice(format!("{}\"{}\"", MultipartFilenamePrefix, filename).as_bytes());
        }
        self.content.extend_from_slice("\r\n".as_bytes()); // close line

        if let Some(content_type) = content_type {
            self.content.extend_from_slice(format!("Content-Type: {}\r\n", content_type).as_bytes());
        }

        // end of header, write extra empty line
        self.content.extend_from_slice("\r\n".as_bytes());
    }
}
