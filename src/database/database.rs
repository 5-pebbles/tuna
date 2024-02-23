use rocket_sync_db_pools::{
    database,
    rusqlite::{params, Connection, Error},
};

#[database("user_db")]
pub struct Database(Connection);

impl Database {
    pub async fn migrations(&self) -> Result<(), Error> {
        self.run(|conn| -> Result<(), Error> {
            let tx = conn.transaction()?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS invites (
    code TEXT PRIMARY KEY,
    permissions TEXT NOT NULL DEFAULT '',
    remaining INTEGER NOT NULL DEFAULT  1,
    creator TEXT NOT NULL,
    FOREIGN KEY (creator) REFERENCES users(username)
)
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS users (
    username TEXT PRIMARY KEY,
    permissions TEXT NOT NULL DEFAULT '',
    hash TEXT NOT NULL,
    sessions TEXT NOT NULL DEFAULT ''
)
                ",
                params![],
            )?;
            tx.commit()?;
            Ok(())
        })
        .await
    }
}
