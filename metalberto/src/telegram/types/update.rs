use jb::json::helpers::get_or_error;
use jb::json::{FromJson, JsonMappingError, Value};

use super::message::Message;

#[derive(Default, Debug, Clone)]
pub enum UpdateType {
    // internal types
    #[default]
    Empty,
    Unknown,

    // telegram types
    Message(Message),
    MessageEdit(Message),
    ChannelPost,
    ChannelPostEdit,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    MyChatMember,
}

#[derive(Default, Debug, Clone)]
pub struct Update {
    pub update_id:   i64,
    pub update_type: UpdateType,
}

impl FromJson for Update {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        let Value::Dict(json) = value else {
            return Err(JsonMappingError::TypeMismatch);
        };

        let update_id = get_or_error(&json, "update_id")?;

        let payload = if json.contains_key("message") {
            let message = Message::from_json(json.get("message").unwrap().clone())?;
            UpdateType::Message(message)
        } else if json.contains_key("edited_message") {
            let message = Message::from_json(json.get("edited_message").unwrap().clone())?;
            UpdateType::MessageEdit(message)
        } else {
            UpdateType::Unknown
        };

        Ok(Self { update_id,
                  update_type: payload })
    }
}
