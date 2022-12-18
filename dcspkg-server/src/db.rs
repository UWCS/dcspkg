use dcspkg_server::Package;

pub async fn get_package_by_name(
    conn: &sqlx::SqlitePool,
    name: &str,
) -> Result<Option<Package>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM packages WHERE name=?")
        .bind(name)
        .fetch_optional(conn)
        .await
}

pub async fn get_all_packages(conn: &sqlx::SqlitePool) -> Result<Vec<Package>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM packages")
        .fetch_all(conn)
        .await
}
