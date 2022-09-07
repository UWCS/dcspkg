use anyhow::{anyhow, Context, Result};
use sqlx::{
    sqlite::{self, SqliteConnection},
    Connection,
};

pub fn validate_name_and_version(db_path: &str, pkg_name: &str, version: &str) -> Result<()> {
    smol::block_on(async { async_validate_name_and_version(db_path, pkg_name, version).await })
}

pub fn add_package_to_db(db_path: &str, package: dcspkg_server::Package) -> Result<()> {
    smol::block_on(async { async_add_package_to_db(db_path, package).await })
}

async fn async_validate_name_and_version(
    db_path: &str,
    pkg_name: &str,
    version: &str,
) -> Result<()> {
    let mut connection = connect(db_path).await?;
    let result: Result<(String, String), sqlx::Error> =
        sqlx::query_as("SELECT (name, version) FROM packages WHERE name=? AND version=?")
            .bind(pkg_name)
            .bind(version)
            .fetch_one(&mut connection)
            .await;

    match result {
        Err(sqlx::Error::RowNotFound) => Ok(()),
        Err(e) => Err(e).context("Error in checking against database"),
        Ok(_) => Err(anyhow!(
            "Package with that name and version already exists in database"
        )),
    }
}

async fn async_add_package_to_db(db_path: &str, package: dcspkg_server::Package) -> Result<()> {
    Ok(())
}

async fn connect(path: &str) -> Result<SqliteConnection> {
    sqlite::SqliteConnection::connect(path)
        .await
        .context("Could not connect to database")
}
