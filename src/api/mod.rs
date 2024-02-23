use rocket::fairing::AdHoc;

mod user;
pub mod errors;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Systems", |rocket| async {
        rocket.attach(user::fairing())
    })
}
