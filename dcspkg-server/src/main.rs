use db::PackageDB;
use handlers::*;
use rocket::routes;
use rocket_db_pools::Database;

mod db;
mod handlers;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let package_path =
        std::env::var("PACKAGE_PATH").unwrap_or_else(|_| "./packages/packages".to_owned());

    let _rocket = rocket::build()
        .attach(PackageDB::init())
        .mount("/", routes![list, pkgdata])
        .mount("/download", rocket::fs::FileServer::from(package_path))
        .launch()
        .await?;

    Ok(())
}
