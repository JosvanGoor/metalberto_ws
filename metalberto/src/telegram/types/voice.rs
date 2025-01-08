use jb::json::FromJson;

#[derive(Default, Debug, Clone, FromJson)]
pub struct Voice {
    pub file_id:        String,
    pub file_unique_id: String,
    pub mime_type:      String,
    pub duration:       Option<u64>,
    pub file_size:      Option<u64>,
}
