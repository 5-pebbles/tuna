use rocket::fairing::AdHoc;

mod database;
pub use database::Database;

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
