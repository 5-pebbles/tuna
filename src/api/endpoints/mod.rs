use rocket::fairing::AdHoc;

mod albums;
mod artists;
mod genres;
mod invites;
mod permissions;
mod tokens;
mod tracks;
mod users;

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
