use rocket_sync_db_pools::{database, rusqlite::Error};

use crate::database::MyConnection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[database("db")]
pub struct Database(MyConnection);

impl Database {
    pub async fn migrations(&self) -> Result<(), Error> {
        self.run(|conn| -> Result<(), Error> {
            embedded::migrations::runner().run(&mut conn.0).unwrap();

            Ok(())
        })
        .await
    }
}
