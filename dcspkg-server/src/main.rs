use db::PackageDB;
use handlers::*;
use rocket::routes;
use rocket_db_pools::Database;

mod db;
mod handlers;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let _rocket = rocket::build()
        .attach(PackageDB::init())
        .mount("/", routes![list, pkgdata])
        .mount("/download", rocket::fs::FileServer::from("packages"))
        .launch()
        .await?;

    Ok(())
}
