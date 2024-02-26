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

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS artists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    genres TEXT NOT NULL DEFAULT '',
    bio TEXT NOT NULL DEFAULT ''
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS albums (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    release INTEGER NOT NULL DEFAULT  0,
    genres TEXT NOT NULL DEFAULT '',
    count INTEGER NOT NULL DEFAULT 1
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE album_artist (
    album_id TEXT NOT NULL,
    artist_id TEXT NOT NULL,
    PRIMARY KEY (album_id, artist_id),
    FOREIGN KEY (album_id) REFERENCES albums(id),
    FOREIGN KEY (artist_id) REFERENCES artists(id)
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE tracks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    release INTEGER NOT NULL DEFAULT  0,
    duration INTEGER NOT NULL DEFAULT  0,
    segments TEXT NOT NULL DEFAULT '',
    genres TEXT NOT NULL DEFAULT '',
    lyrics TEXT NOT NULL DEFAULT ''
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE track_album (
    track_id TEXT NOT NULL,
    album_id TEXT NOT NULL,
    PRIMARY KEY (track_id, album_id),
    FOREIGN KEY (track_id) REFERENCES tracks(id),
    FOREIGN KEY (album_id) REFERENCES albums(id)
);
                ",
                params![],
            )?;

            tx.commit()?;
            Ok(())
        })
        .await
    }
}
