use rocket::fairing::AdHoc;

mod genres;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Music Systems", |rocket| async {
        rocket.attach(genres::fairing())
    })
}
