use rocket::fairing::AdHoc;

mod database;
mod connection;
mod pool_manager;

pub use database::MyDatabase;
pub use connection::MyConnection;
pub use pool_manager::MyPoolManager;

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Database Systems", |rocket| async {
        rocket.attach(MyDatabase::fairing()).attach(AdHoc::on_ignite(
            "Database Migrations",
            |rocket| async {
                <MyDatabase>::get_one(&rocket)
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
