use crate::db::{get_all_packages, get_package_by_name};
use dcspkg::Package;
use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/list")]
pub async fn list(db: &State<sqlx::SqlitePool>) -> Json<Vec<Package>> {
    match get_all_packages(db.inner()).await {
        Ok(x) => Json(x),
        Err(e) => panic!("{e:?}"), //TODO, work out how to handle failure in reponder
    }
}

#[get("/pkgdata/<name>")]
pub async fn pkgdata(db: &State<sqlx::SqlitePool>, name: &str) -> Option<Json<Package>> {
    get_package_by_name(db.inner(), name)
        .await
        .ok()
        .flatten()
        .map(Json)
}
