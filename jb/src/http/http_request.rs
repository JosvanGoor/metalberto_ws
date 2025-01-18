use crate::common::AnyCase;
use crate::http::{HttpContent, HttpMethod};
use crate::net::Uri;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    fields: HashMap<AnyCase, String>,
}

impl HttpRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_field<T: Into<AnyCase>>(&mut self, key: T, value: &str) -> Option<String> {
        self.fields.insert(key.into(), value.into())
    }

    pub fn get_field<T: Into<AnyCase>>(&mut self, key: T) -> Option<&String> {
        self.fields.get(&key.into())
    }

    pub fn generate(&mut self, method: HttpMethod, uri: &Uri, content: Option<&HttpContent>) -> Vec<u8> {
        let mut buffer = Vec::new();
        write_head_line(&mut buffer, method, uri);

        if let Some(content) = content {
            self.set_field("Content-Length", &format!("{}", content.content_length()));
            self.set_field("Content-Type", content.content_type());
        } else {
            self.fields.remove(&"Content-Length".into());
            self.fields.remove(&"Content-Type".into());
        }

        self.fields.entry("Connection".into()).or_insert_with(|| "Close".into());
        self.fields.entry("Accept".into()).or_insert_with(|| "*/*".into());
        self.fields.insert("Host".into(), uri.host().into());

        for (key, val) in self.fields.iter() {
            buffer.extend_from_slice(format!("{}: {}\r\n", key, val).as_bytes());
        }

        // extra empty line to end header
        buffer.extend_from_slice("\r\n".as_bytes());
        if let Some(content) = content {
            buffer.extend_from_slice(content.as_slice());
        }

        buffer
    }
}

// Boilerplate heavy helper functions
fn write_head_line(buffer: &mut Vec<u8>, method: HttpMethod, uri: &Uri) {
    let method_str: &str = method.into();

    buffer.extend_from_slice(method_str.as_bytes());
    buffer.push(b' ');
    if uri.path().is_empty() {
        buffer.push(b'/');
    } else {
        buffer.extend_from_slice(uri.path().as_bytes());
    }

    if !uri.query().is_empty() {
        buffer.push(b'?');
        buffer.extend_from_slice(uri.query().as_bytes());
    }

    buffer.extend_from_slice(" HTTP/1.1\r\n".as_bytes());
}
