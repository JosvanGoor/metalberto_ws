use jb::json::FromJson;

#[derive(Default, Debug, Clone, FromJson)]
pub struct User {
    id:            i64,
    is_bot:        bool,
    first_name:    String,
    last_name:     Option<String>,
    nickname:      Option<String>,
    language_code: Option<String>,
}
