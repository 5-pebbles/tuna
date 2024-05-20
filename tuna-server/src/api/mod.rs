use rocket::fairing::AdHoc;

pub mod data;
pub mod endpoints;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Systems", |rocket| async {
        rocket.attach(endpoints::fairing())
    })
}
