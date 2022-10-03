use crate::db::{get_all_packages, get_package_by_name, get_package_by_id, PackageDB};
use dcspkg_common::Package;
use rocket::get;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/list")]
pub async fn list(mut db: Connection<PackageDB>) -> Json<Vec<Package>> {
    match get_all_packages(&mut *db).await {
        Ok(x) => Json(x),
        Err(e) => panic!("{e:?}"), //TODO, work out how to handle failure in reponder
    }
}

#[get("/pkgdata/<name>")]
pub async fn pkgdata(mut db: Connection<PackageDB>, name: &str) -> Option<Json<Package>> {
    let mut pkg = get_package_by_name(&mut *db, name).await.ok()?;
    // If nothing, attempt fetching by ID
    if pkg.is_none() {
        let id: i64 = name.parse().ok()?;
        pkg = get_package_by_id(&mut *db, id).await.ok()?;
    }
    pkg.map(Json)
}
