use db::{get_all_packages, PackageDB};
use dcspkg_server::Package;
use rocket::serde::json::Json;
use rocket::{get, routes};
use rocket_db_pools::{Connection, Database};
mod db;

#[get("/list")]
async fn list(mut db: Connection<PackageDB>) -> Json<Vec<Package>> {
    match get_all_packages(&mut *db).await {
        Ok(x) => Json(x),
        Err(_) => panic!(), //TODO, work out how to handle failure in reponder
    }
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
