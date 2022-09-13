use anyhow::{anyhow, Context, Result};
use dcspkg_common::Package;
use sqlx::{
    sqlite::{self, SqliteConnection},
    Connection,
};
use std::path::Path;

pub fn validate_name_and_version(db_path: &Path, pkg_name: &str, version: &str) -> Result<()> {
    smol::block_on(async { async_validate_name_and_version(db_path, pkg_name, version).await })
}

pub fn add_package_to_db(db_path: &Path, package: &mut Package) -> Result<()> {
    smol::block_on(async { async_add_package_to_db(db_path, package).await })
}

async fn async_validate_name_and_version(
    db_path: &Path,
    pkg_name: &str,
    version: &str,
) -> Result<()> {
    let mut connection = connect(db_path).await?;
    let result: Result<Option<(String, String)>, sqlx::Error> =
        sqlx::query_as("SELECT name, version FROM packages WHERE name=? AND version=?")
            .bind(pkg_name)
            .bind(version)
            .fetch_optional(&mut connection)
            .await;

    match result {
        Ok(None) => Ok(()),
        Err(e) => Err(e).context("Error in checking against database"),
        Ok(Some(_)) => Err(anyhow!(
            "Package with that name and version already exists in database"
        )),
    }
}

async fn async_add_package_to_db(db_path: &Path, package: &mut Package) -> Result<()> {
    let mut connection = connect(db_path).await?;
    let query = sqlx::query(
        "INSERT INTO packages (name, description, version, image_url, archive_path, executable_path, crc, has_installer, add_to_path) VALUES (?,?,?,?,?,?,?,?,?)")
        .bind(&package.name)
        .bind(&package.description)
        .bind(&package.version)
        .bind(&package.image_url)
        .bind(&package.archive_path)
        .bind(&package.executable_path)
        .bind(package.crc)
        .bind(package.has_installer)
        .bind(package.add_to_path)
        .execute(&mut connection)
        .await.context("Could not insert package into database")?;
    package.id = query.last_insert_rowid();
    Ok(())
}

async fn connect(path: &Path) -> Result<SqliteConnection> {
    sqlite::SqliteConnection::connect(
        path.to_str()
            .ok_or_else(|| anyhow!("Could not convert database path to string"))?,
    )
    .await
    .context("Could not connect to database")
}
