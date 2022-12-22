use handlers::*;
use rocket::routes;

mod db;
mod handlers;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let package_path =
        std::env::var("PACKAGE_PATH").unwrap_or_else(|_| "./packages/packages".to_owned());

    let db = {
        let path =
            std::env::var("DB_PATH").unwrap_or_else(|_| "./packages/packagedb.sqlite".to_owned());
        sqlx::SqlitePool::connect(&path).await?
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![list, pkgdata])
        .mount("/download", rocket::fs::FileServer::from(package_path))
        .launch()
        .await
        .map(|_| ())
        .map_err(Into::into)
}
