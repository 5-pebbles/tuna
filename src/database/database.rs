use rocket_sync_db_pools::{
    database,
    rusqlite::{params, Connection, Error},
};

#[database("db")]
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
CREATE TABLE IF NOT EXISTS genres (
    id TEXT PRIMARY KEY
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS artists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    bio TEXT NOT NULL DEFAULT ''
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS artist_genres (
    artist_id TEXT NOT NULL,
    genre_id TEXT NOT NULL,
    PRIMARY KEY (artist_id, genre_id),
    FOREIGN KEY (artist_id) REFERENCES artists(id),
    FOREIGN KEY (genre_id) REFERENCES genres(id)
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
    count INTEGER NOT NULL DEFAULT 1
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS album_genres (
    album_id TEXT NOT NULL,
    genre_id TEXT NOT NULL,
    PRIMARY KEY (album_id, genre_id),
    FOREIGN KEY (album_id) REFERENCES albums(id),
    FOREIGN KEY (genre_id) REFERENCES genres(id)
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS artist_albums (
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
CREATE TABLE IF NOT EXISTS tracks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    release INTEGER NOT NULL DEFAULT  0,
    duration INTEGER NOT NULL DEFAULT  0,
    lyrics TEXT NOT NULL DEFAULT ''
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS track_genres (
    track_id TEXT NOT NULL,
    genre_id TEXT NOT NULL,
    PRIMARY KEY (track_id, genre_id),
    FOREIGN KEY (track_id) REFERENCES tracks(id),
    FOREIGN KEY (genre_id) REFERENCES genre(id)
);
                ",
                params![],
            )?;

            tx.execute(
                "
CREATE TABLE IF NOT EXISTS album_tracks (
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
