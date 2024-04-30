use rocket::fairing::AdHoc;
use rocket_sync_db_pools::{database, rusqlite::Error};

mod connection;
mod pool_manager;

pub use connection::MyConnection;
pub use pool_manager::MyPoolManager;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[database("db")]
pub struct MyDatabase(MyConnection);

impl MyDatabase {
    pub async fn migrations(&self) -> Result<(), Error> {
        self.run(|conn| -> Result<(), Error> {
            embedded::migrations::runner().run(&mut conn.0).unwrap();

            Ok(())
        })
        .await
    }
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Database Systems", |rocket| async {
        rocket
            .attach(MyDatabase::fairing())
            .attach(AdHoc::on_ignite("Database Migrations", |rocket| async {
                <MyDatabase>::get_one(&rocket)
                    .await
                    .expect("Mount Database")
                    .migrations()
                    .await
                    .expect("Database Migrations Failed");
                rocket
            }))
    })
}
