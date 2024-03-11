use rocket::{fairing::AdHoc, serde::Serialize};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;

use crate::database::permissions::Permission;

mod invites;
mod permissions;
mod tokens;
mod users;

#[derive(Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct UserApiItem {
    username: String,
    permissions: SqlVec<Permission>,
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API User Systems", |rocket| async {
        rocket
            .attach(invites::fairing())
            .attach(permissions::fairing())
            .attach(users::fairing())
            .attach(tokens::fairing())
    })
}
