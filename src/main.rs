//#![allow(clippy::too_many_arguments)]

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
    fn ensure_dir(dir: &str) {
        let path = Path::new(dir);
        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to create directory");
        }
    }
    ensure_dir("./database/sqlite");
    ensure_dir("./database/audio");
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
