use jb::json::FromJson;

use super::Update;

#[derive(Debug, Clone, FromJson)]
pub struct Response<T: FromJson>
{
    pub ok:          bool,
    pub result:      T,
    pub description: Option<String>,
}
