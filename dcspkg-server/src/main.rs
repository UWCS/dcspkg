use handlers::*;
use rocket::routes;

mod db;
mod handlers;
mod package;
pub use package::Package;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let package_path =
        std::env::var("PACKAGE_PATH").unwrap_or_else(|_| "./packages/packages".to_owned());

    let db = sqlx::SqlitePool::connect("").await?;

    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes![list, pkgdata])
        .mount("/download", rocket::fs::FileServer::from(package_path))
        .launch()
        .await?;

    Ok(())
}
