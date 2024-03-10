CREATE TABLE IF NOT EXISTS artist_albums (album_id TEXT NOT NULL
,   artist_id TEXT NOT NULL
,   PRIMARY KEY (album_id, artist_id)
,   FOREIGN KEY (album_id) REFERENCES albums(id)
,   FOREIGN KEY (artist_id) REFERENCES artists(id)
);
