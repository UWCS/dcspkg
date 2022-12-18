use anyhow::{anyhow, Context, Result};
use dcspkg_server::Package;
use sqlx::{
    sqlite::{self, SqliteConnection},
    Connection,
};
use std::path::Path;

pub fn check_name_unique(db_path: &Path, pkg_name: &str) -> Result<()> {
    smol::block_on(async { async_check_name_unique(db_path, pkg_name).await })
}

pub fn add_package_to_db(db_path: &Path, package: Package) -> Result<()> {
    smol::block_on(async { async_add_package_to_db(db_path, package).await })
}

async fn async_check_name_unique(db_path: &Path, pkg_name: &str) -> Result<()> {
    let mut connection = connect(db_path).await?;
    let result: Result<Option<(String, String)>, sqlx::Error> =
        sqlx::query_as("SELECT * FROM packages WHERE pkgname=?")
            .bind(pkg_name)
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

async fn async_add_package_to_db(db_path: &Path, package: Package) -> Result<()> {
    let mut connection = connect(db_path).await?;
    sqlx::query(
        "INSERT INTO packages (pkgname, fullname, description, image_url, executable_path, crc, has_installer, add_to_path) VALUES (?,?,?,?,?,?,?,?,?)")
        .bind(&package.pkgname)
        .bind(&package.fullname)
        .bind(&package.description)
        .bind(&package.image_url)
        .bind(&package.executable_path)
        .bind(package.crc)
        .bind(package.has_installer)
        .bind(package.add_to_path)
        .execute(&mut connection)
        .await.context("Could not insert package into database").map(|_|()).map_err(Into::into)
}

async fn connect(path: &Path) -> Result<SqliteConnection> {
    sqlite::SqliteConnection::connect(
        path.to_str()
            .ok_or_else(|| anyhow!("Could not convert database path to string"))?,
    )
    .await
    .context("Could not connect to database")
}
