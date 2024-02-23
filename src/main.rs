#[macro_use]
extern crate rocket;

mod api;
mod database;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::fairing())
        .attach(api::fairing())
        .mount("/", routes![index])
}
