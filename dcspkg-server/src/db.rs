use dcspkg_common::Package;
use rocket_db_pools::Database;
use sqlx::{self, pool::PoolConnection, Sqlite};

#[derive(Database)]
#[database("packagedb")]
pub struct PackageDB(rocket_db_pools::sqlx::SqlitePool);

pub async fn get_package_by_name(
    conn: &mut PoolConnection<Sqlite>,
    name: &str,
) -> Result<Option<Package>, sqlx::Error> {
    let mut all: Vec<Package> = sqlx::query_as("SELECT * FROM packages WHERE name=?")
        .bind(name)
        .fetch_all(conn)
        .await?;
    //get latest version
    all.sort_by_key(|p| semver::Version::parse(&p.version).unwrap());
    Ok(all.pop())
}

pub async fn get_all_packages(
    conn: &mut PoolConnection<Sqlite>,
) -> Result<Vec<Package>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM packages")
        .fetch_all(conn)
        .await
}
