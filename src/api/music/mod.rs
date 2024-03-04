use rocket::fairing::AdHoc;

mod albums;
mod artists;
mod genres;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Music Systems", |rocket| async {
        rocket
            .attach(genres::fairing())
            .attach(artists::fairing())
            .attach(albums::fairing())
    })
}
