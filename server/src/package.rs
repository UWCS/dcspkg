use rocket_db_pools::sqlx::Row;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
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

impl TryFrom<rocket_db_pools::sqlx::sqlite::SqliteRow> for Package {
    type Error = anyhow::Error;

    fn try_from(row: rocket_db_pools::sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        Ok(Package {
            id: row.try_get("rowid")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            version: row.try_get("version")?,
            image_url: row.try_get("image_url")?,
            archive_name: row.try_get("archive_name")?,
            crc: row.try_get("crc")?,
            has_installer: row.try_get("has_installer")?,
        })
    }
}
