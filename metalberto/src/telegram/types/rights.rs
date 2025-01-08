
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Rights {
    can_be_edited: bool,
    is_anonymous: bool,
    can_post_messages: bool,
    can_edit_messages: bool,
    can_delete_messages: bool,
    can_manage_voice_chats: bool,
    can_restrict_members: bool,
    can_promote_members: bool,
    can_change_info: bool,
    can_invite_users: bool,
    can_pin_messages: bool,
    is_member: bool,
    can_send_messages: bool,
    can_send_media_messages: bool,
    can_send_polls: bool,
    can_send_other_messages: bool,
    can_add_web_page_previews: bool,
}

impl Rights {

}

// MARK: Display

// MARK: Composition