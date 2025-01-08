use jb::json::helpers::{get_or_error, get_or_none};
use jb::json::{FromJson, IntoJson, JsonMappingError, Value};

#[derive(Default, Debug, Clone)]
pub struct Chat {
    pub id:         i64,
    pub chat_type:  String,
    pub title:      Option<String>,
    pub username:   Option<String>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
}

impl FromJson for Chat {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        let Value::Dict(json) = value else {
            return Err(JsonMappingError::TypeMismatch);
        };

        Ok(Self {
            id: get_or_error(&json, "id")?,
            chat_type: get_or_error(&json, "type")?,
            title: get_or_none(&json, "title")?,
            username: get_or_none(&json, "username")?,
            first_name: get_or_none(&json, "first_name")?,
            last_name: get_or_none(&json, "last_name")?
        })
    }
}
