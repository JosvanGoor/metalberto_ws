use jb::json::FromJson;

use super::Update;

#[derive(Default, Debug, Clone, FromJson)]
pub struct Response {
    pub ok: bool,
    pub result: Vec<Update>,
    pub description: Option<String>,
}