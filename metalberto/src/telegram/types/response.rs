use jb::json::FromJson;

use super::Update;

#[derive(Default, Debug, Clone, FromJson)]
pub struct Response {
    ok: bool,
    result: Vec<Update>,
}