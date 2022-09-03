use db::PackageDB;
use dcspkg_server::Package;
use rocket::serde::json::Json;
use rocket::{get, routes};
use rocket_db_pools::{Connection, Database};
mod db;

#[get("/list")]
async fn list(mut db: Connection<PackageDB>) -> Json<Vec<Package>> {
    Json(vec![db::get_package_by_name(&mut *db, "").await.unwrap()])
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let _rocket = rocket::build()
        .attach(PackageDB::init())
        .mount("/", routes![list])
        .mount("/packages", rocket::fs::FileServer::from("/packages"))
        .launch()
        .await?;

    Ok(())
}
