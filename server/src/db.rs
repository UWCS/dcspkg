use dcspkg_server::Package;
use rocket_db_pools::{
    sqlx::{self, pool::PoolConnection, Sqlite},
    Database,
};

#[derive(Database)]
#[database("sqlite_logs")]
pub struct PackageDB(sqlx::SqlitePool);

pub async fn get_package_by_name(
    conn: &mut PoolConnection<Sqlite>,
    name: &str,
) -> anyhow::Result<Package> {
    let row = sqlx::query("SELECT * FROM packages WHERE name=?")
        .bind(name)
        .fetch_one(conn)
        .await?;

    row.try_into()
}

pub async fn get_all_packages(
    conn: &mut PoolConnection<Sqlite>,
    name: &str,
) -> anyhow::Result<Vec<Package>> {
    //TODO
    Ok(vec![])
}
