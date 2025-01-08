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
    MyChatMember
}

#[derive(Default, Debug, Clone)]
pub struct Update {
    pub update_id: i64,
    pub update_type: UpdateType,
}

impl FromJson for Update {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        todo!()
    }
}