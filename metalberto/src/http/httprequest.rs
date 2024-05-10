use std::HashMap;
use crate::common::anycase::AnyCase;

pub type HttpRequestFields = HashMap<AnyCase, String>;

// fn http_get_request(fields: &HttpRequestFields, )