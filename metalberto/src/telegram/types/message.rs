use jb::json::helpers::{get_or_error, get_or_none};
use jb::json::{FromJson, JsonMappingError, Value};

use super::{Animation, Chat, User, Voice};

#[derive(Default, Debug, Clone)]
pub enum MessageType {
    Text {
        body: String,
    },
    Animation {
        body:    Animation,
        caption: Option<String>,
    },
    Voice {
        body:    Voice,
        caption: Option<String>,
    },
    #[default]
    Unsupported,
}

#[derive(Default, Debug, Clone)]
pub struct Message {
    pub message_id:       i64,
    pub date:             i64,
    pub chat:             Chat,
    pub edit_date:        Option<i64>,
    pub media_group_id:   Option<String>,
    pub author_signature: Option<String>,
    pub from:             Option<User>,
    pub payload:          MessageType,
}

impl FromJson for Message {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        let Value::Dict(json) = value else {
            return Err(JsonMappingError::TypeMismatch);
        };

        let payload = if json.contains_key("text") {
            MessageType::Text { body: get_or_error(&json, "text")?, }
        } else if json.contains_key("animation") {
            MessageType::Animation { body:    get_or_error(&json, "animation")?,
                                 caption: get_or_none(&json, "caption")?, }
        } else if json.contains_key("voice") {
            MessageType::Voice { body:    get_or_error(&json, "voice")?,
                             caption: get_or_none(&json, "caption")?, }
        } else {
            MessageType::Unsupported
        };

        Ok(Self { message_id: get_or_error(&json, "message_id")?,
                  date: get_or_error(&json, "date")?,
                  chat: get_or_error(&json, "chat")?,
                  edit_date: get_or_none(&json, "edit_date")?,
                  media_group_id: get_or_none(&json, "media_group_id")?,
                  author_signature: get_or_none(&json, "author_signature")?,
                  from: get_or_none(&json, "from")?,
                  payload })
    }
}
