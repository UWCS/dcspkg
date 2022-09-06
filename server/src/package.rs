use serde::{Deserialize, Serialize};

/// Represents a package, and contains all the metadata assoicated with it.
///
/// [`sqlx::FromRow`][sqlx::FromRow] is derived, so this should match the database schema
/// as specified in `scripts/init_db.py`.
#[derive(Deserialize, Default, Serialize, Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Package {
    pub id: i64,
    pub name: String,
    #[sqlx(default)]
    pub description: String,
    pub version: String,
    pub image_url: Option<String>,
    pub archive_path: String,
    pub executable_path: String,
    pub crc: u32,
    pub has_installer: bool,
    pub add_to_path: bool,
}
