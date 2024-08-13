
const ApplicationJson: &str = "application/json";

#[derive(Clone, Debug)]
pub struct HttpContentInfo {
    content_type: String,
    content_length: usize
}

impl HttpContentInfo {

    pub fn new(content_type: String, content_length: usize) -> Self {
        Self { content_type: content_type, content_length: content_length }
    }

}