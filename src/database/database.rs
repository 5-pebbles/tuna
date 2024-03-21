use rocket_sync_db_pools::{
    database,
    rusqlite::{Connection, Error},
};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[database("db")]
pub struct Database(Connection);

impl Database {
    pub async fn migrations(&self) -> Result<(), Error> {
        self.run(|conn| -> Result<(), Error> {
            embedded::migrations::runner().run(conn).unwrap();

            Ok(())
        })
        .await
    }
}
