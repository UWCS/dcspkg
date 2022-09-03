use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Serialize, Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Package {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub version: String,
    pub image_url: Option<String>,
    pub archive_name: String,
    pub crc: u32,
    pub has_installer: bool,
}
