extern crate dotenv;
#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket::State;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use rocket::serde::json::Json;
use std::env;
use entities::{*, prelude::*};

mod entities;

#[get("/list")]
async fn list(db: &State<DatabaseConnection>) -> Json<Vec<package::Model>> {
    let db = db as &DatabaseConnection;
    Json(Package::find().all(db).await.unwrap())
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(db_url).await.unwrap();

    rocket::build()
        .manage(db)
        .mount("/", routes![list])
        .mount("/games", rocket::fs::FileServer::from("games/"))
}
