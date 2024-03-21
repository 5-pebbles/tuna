#[macro_use]
extern crate rocket;

use std::{fs, path::Path};

mod api;
mod database;
mod docs;
mod error;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn create_working_dir() {
    let db_dir = Path::new("./database/sqlite");
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).expect("Failed to create database directory");
    }
}

#[launch]
fn rocket() -> _ {
    create_working_dir();

    rocket::build()
        .attach(database::fairing())
        .attach(api::fairing())
        .attach(docs::fairing())
        .mount("/", routes![index])
}
