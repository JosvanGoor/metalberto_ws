use jb::json::FromJson;

#[derive(Default, Debug, Clone, FromJson)]
pub struct Animation {
    pub file_id:        String,
    pub file_unique_id: String,
    pub width:          u64,
    pub height:         u64,
    pub duration:       u64,
    pub file_name:      Option<String>,
    pub mime_type:      Option<String>,
    pub file_size:      Option<u64>,
}
