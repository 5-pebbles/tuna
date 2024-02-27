use rocket::fairing::AdHoc;

pub mod database;
use database::Database;

pub mod invites;
pub mod permissions;
pub mod users;

pub mod genres;

pub mod albums;
pub mod artists;
pub mod tracks;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Database Systems", |rocket| async {
        rocket.attach(Database::fairing()).attach(AdHoc::on_ignite(
            "Database Migrations",
            |rocket| async {
                <Database>::get_one(&rocket)
                    .await
                    .expect("Mount Database")
                    .migrations()
                    .await
                    .expect("Database Migrations Failed");
                rocket
            },
        ))
    })
}
