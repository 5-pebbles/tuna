use rocket::fairing::AdHoc;

pub mod albums;
pub mod artists;
pub mod genres;
pub mod invites;
pub mod permissions;
pub mod tokens;
pub mod tracks;
pub mod users;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Music Systems", |rocket| async {
        rocket
            .attach(genres::fairing())
            .attach(artists::fairing())
            .attach(albums::fairing())
            .attach(tracks::fairing())
            .attach(invites::fairing())
            .attach(permissions::fairing())
            .attach(users::fairing())
            .attach(tokens::fairing())
    })
}
