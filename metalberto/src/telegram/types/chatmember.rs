use jb::json::helpers::{get_or_error, get_or_none};
use jb::json::{FromJson, JsonMappingError, Value};

use super::User;

#[derive(Debug)]
pub enum ChatMemberStatus {
    Owner {
        is_anonymous: bool,
        custom_title: Option<String>,
    },
    Administrator {
        can_be_edited:          bool,
        is_anonymous:           bool,
        can_manage_chat:        bool,
        can_delete_messages:    bool,
        can_manage_video_chats: bool,
        can_restrict_members:   bool,
        can_promote_members:    bool,
        can_change_info:        bool,
        can_invite_users:       bool,
        can_post_stories:       bool,
        can_edit_stories:       bool,
        can_delete_stories:     bool,
        can_post_messages:      Option<bool>,
        can_edit_messages:      Option<bool>,
        can_pin_messages:       Option<bool>,
        can_manage_topics:      Option<bool>,
        custom_title:           Option<String>,
    },
    Member {
        until_date: u64,
    },
    Restricted {
        is_member:                 bool,
        can_send_messages:         bool,
        can_send_audios:           bool,
        can_send_documents:        bool,
        can_send_photos:           bool,
        can_send_videos:           bool,
        can_send_video_notes:      bool,
        can_send_voice_notes:      bool,
        can_send_polls:            bool,
        can_send_other_messages:   bool,
        can_add_web_page_previews: bool,
        can_change_info:           bool,
        can_invite_users:          bool,
        can_pin_messages:          bool,
        can_manage_topics:         bool,
        until_date:                u64,
    },
    Left,
    Banned {
        until_date: u64,
    },
}

pub struct ChatMember {
    user:   User,
    status: ChatMemberStatus,
}

impl FromJson for ChatMember {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        let Value::Dict(json) = value else {
            return Err(JsonMappingError::TypeMismatch);
        };

        let status = match get_or_error::<String>(&json, "status")?.as_str() {
            "creator" => ChatMemberStatus::Owner { is_anonymous: get_or_error(&json, "is_anonymous")?,
                                                   custom_title: get_or_none(&json, "custom_title")?, },
            "administrator" => ChatMemberStatus::Administrator { can_be_edited:          get_or_error(&json, "can_be_edited")?,
                                                                 is_anonymous:           get_or_error(&json, "is_anonymous")?,
                                                                 can_manage_chat:        get_or_error(&json, "can_manage_chat")?,
                                                                 can_delete_messages:    get_or_error(&json, "can_delete_messages")?,
                                                                 can_manage_video_chats: get_or_error(&json, "can_manage_video_chats")?,
                                                                 can_restrict_members:   get_or_error(&json, "can_restrict_members")?,
                                                                 can_promote_members:    get_or_error(&json, "can_promote_members")?,
                                                                 can_change_info:        get_or_error(&json, "can_change_info")?,
                                                                 can_invite_users:       get_or_error(&json, "can_invite_users")?,
                                                                 can_post_stories:       get_or_error(&json, "can_post_stories")?,
                                                                 can_edit_stories:       get_or_error(&json, "can_edit_stories")?,
                                                                 can_delete_stories:     get_or_error(&json, "can_delete_stories")?,
                                                                 can_post_messages:      get_or_none(&json, "can_post_messages")?,
                                                                 can_edit_messages:      get_or_none(&json, "can_edit_messages")?,
                                                                 can_pin_messages:       get_or_none(&json, "can_pin_messages")?,
                                                                 can_manage_topics:      get_or_none(&json, "can_manage_topics")?,
                                                                 custom_title:           get_or_none(&json, "custom_title")?, },
            "member" => ChatMemberStatus::Member { until_date: get_or_error(&json, "until_date")?, },
            "restricted" => ChatMemberStatus::Restricted { is_member:                 get_or_error(&json, "is_member")?,
                                                           can_send_messages:         get_or_error(&json, "can_send_messages")?,
                                                           can_send_audios:           get_or_error(&json, "can_send_audios")?,
                                                           can_send_documents:        get_or_error(&json, "can_send_documents")?,
                                                           can_send_photos:           get_or_error(&json, "can_send_photos")?,
                                                           can_send_videos:           get_or_error(&json, "can_send_videos")?,
                                                           can_send_video_notes:      get_or_error(&json, "can_send_video_notes")?,
                                                           can_send_voice_notes:      get_or_error(&json, "can_send_voice_notes")?,
                                                           can_send_polls:            get_or_error(&json, "can_send_polls")?,
                                                           can_send_other_messages:   get_or_error(&json, "can_send_other_messages")?,
                                                           can_add_web_page_previews: get_or_error(&json, "can_add_web_page_previews")?,
                                                           can_change_info:           get_or_error(&json, "can_change_info")?,
                                                           can_invite_users:          get_or_error(&json, "can_invite_users")?,
                                                           can_pin_messages:          get_or_error(&json, "can_pin_messages")?,
                                                           can_manage_topics:         get_or_error(&json, "can_manage_topics")?,
                                                           until_date:                get_or_error(&json, "until_date")?, },
            "left" => ChatMemberStatus::Left,
            "kicked" => ChatMemberStatus::Banned { until_date: get_or_error(&json, "until_date")? },
            _ => return Err(JsonMappingError::TypeMismatch),
        };

        Ok(Self {
            user: get_or_error(&json, "user")?,
            status,
        })
    }
}
