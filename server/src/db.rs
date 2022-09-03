use dcspkg_server::Package;
use rocket_db_pools::Database;
use sqlx::{self, pool::PoolConnection, Sqlite};

#[derive(Database)]
#[database("sqlite_logs")]
pub struct PackageDB(rocket_db_pools::sqlx::SqlitePool);

pub async fn get_package_by_name(
    conn: &mut PoolConnection<Sqlite>,
    name: &str,
) -> Result<Package, sqlx::Error> {
    sqlx::query_as("SELECT * FROM packages WHERE name=?")
        .bind(name)
        .fetch_one(conn)
        .await
}

pub async fn get_all_packages(
    conn: &mut PoolConnection<Sqlite>,
) -> Result<Vec<Package>, sqlx::Error> {
    sqlx::query_as::<_, Package>("SELECT * FROM packages")
        .fetch_all(conn)
        .await
}
