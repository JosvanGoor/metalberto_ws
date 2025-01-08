use jb::json::{FromJson, JsonMappingError, Value};

use super::{Animation, Chat, User, Voice};

pub enum Payload {
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

    Unsupported,
}

#[derive(Default, Debug, Clone)]
pub struct Message {
    message_id:       i64,
    date:             i64,
    chat:             Chat,
    edit_date:        Option<i64>,
    media_group_id:   Option<String>,
    author_signature: Option<String>,
    from:             Option<User>,
}

impl FromJson for Message {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        todo!()
    }
}
