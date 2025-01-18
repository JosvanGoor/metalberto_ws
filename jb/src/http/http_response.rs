use super::{HttpContent, HttpResponseStatusCode};
use crate::common::AnyCase;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct HttpResponse {
    pub status: HttpResponseStatusCode,
    pub reason: String,
    pub version: String,
    pub content: HttpContent,
    pub fields: HashMap<AnyCase, String>,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self::default()
    }
}