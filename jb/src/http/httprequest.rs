use crate::common::AnyCase;
use crate::http::{HttpContentInfo, HttpMethod, HttpResult};
use crate::net::Uri;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    fields: HashMap<AnyCase, String>
}

impl HttpRequest {

    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, key: AnyCase, value: String) -> Option<String> {
        self.fields.insert(key, value)
    }

    

    pub fn generate(&self, method: HttpMethod, uri: &Uri, _content: Option<HttpContentInfo>) -> HttpResult<Vec<u8>> {
        let mut buffer = Vec::new();
        write_head_line(&mut buffer, method, uri);

        Ok(buffer)
    }

}

impl Index<String> for HttpRequest {
    type Output = String;

    fn index(&self, index: String) -> &Self::Output {
        &self.fields[&index.into()]
    }
}

impl Index<&AnyCase> for HttpRequest {
    type Output = String;

    fn index(&self, index: &AnyCase) -> &Self::Output {
        &self.fields[&index]
    }
}

// Boilerplate heavy helper functions
fn write_head_line(buffer: &mut Vec<u8>, method: HttpMethod, uri: &Uri) {
    let method_str: &str = method.into();

    buffer.extend_from_slice(method_str.as_bytes());
    buffer.push(b' ');
    buffer.extend_from_slice(uri.path().as_bytes());

    if !uri.query().is_empty() {
        buffer.push(b'?');
        buffer.extend_from_slice(uri.query().as_bytes());
    }

    buffer.extend_from_slice(" HTTP/1.1\r\n".as_bytes());
}


