use rocket::fairing::AdHoc;

pub mod errors;
mod music;
mod user;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Systems", |rocket| async {
        rocket.attach(user::fairing()).attach(music::fairing())
    })
}
