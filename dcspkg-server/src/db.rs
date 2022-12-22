use dcspkg::Package;
use rocket::futures::TryStreamExt;
use sqlx::{sqlite::SqliteRow, Row};

pub async fn get_package_by_name(
    conn: &sqlx::SqlitePool,
    name: &str,
) -> Result<Option<Package>, sqlx::Error> {
    sqlx::query("SELECT * FROM packages WHERE pkgname=?")
        .bind(name)
        .fetch_optional(conn)
        .await
        .map(|r| r.map(from_sqlite_row))
}

pub async fn get_all_packages(conn: &sqlx::SqlitePool) -> Result<Vec<Package>, sqlx::Error> {
    sqlx::query("SELECT * FROM packages")
        .fetch(conn)
        .map_ok(from_sqlite_row)
        .try_collect()
        .await
}

// fucking orphan rule
fn from_sqlite_row(row: SqliteRow) -> Package {
    assert!(
        row.len() == 8,
        "Database row has wrong number of columns. Has someone fucked with the schema?"
    );

    Package {
        pkgname: row
            .try_get("pkgname")
            .expect("Could not get database row pkgname. Is the schema correct?"),
        fullname: row
            .try_get("fullname")
            .expect("Could not get database row fullname. Is the schema correct?"),
        description: row.try_get("description").ok(),
        image_url: row.try_get("image_url").ok(),
        executable_path: row.try_get("executable_path").ok(),
        crc: row
            .try_get("crc")
            .expect("Could not get database row crc. Is the schema correct?"),
        has_installer: row
            .try_get("has_installer")
            .expect("Could not get database row has_intaller. Is the schema correct?"),
        add_to_path: row
            .try_get("add_to_path")
            .expect("Could not get database row add_to_path. Is the database schema correct?"),
    }
}
