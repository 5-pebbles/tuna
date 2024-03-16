use rocket::fairing::AdHoc;

mod invites;
mod permissions;
mod tokens;
mod users;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API User Systems", |rocket| async {
        rocket
            .attach(invites::fairing())
            .attach(permissions::fairing())
            .attach(users::fairing())
            .attach(tokens::fairing())
    })
}
