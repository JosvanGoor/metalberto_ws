use crate::common::AnyCase;
use std::collections::HashMap;
use super::{HttpResponseStatusCode, HttpContent};

#[derive(Clone, Debug, Default)]
pub struct HttpResponse {
    pub status: HttpResponseStatusCode,
    pub reason: String,
    pub version: String,
    pub content: HttpContent,
    pub fields: HashMap<AnyCase, String>
}

