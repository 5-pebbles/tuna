#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate rocket;

mod api;
mod database;
pub mod docs;
pub mod error;

/// Launch the server
pub fn launch() {
    rocket::execute(
        rocket::build()
            .attach(database::fairing())
            .attach(api::fairing())
            .launch(),
    )
    .unwrap();
}
