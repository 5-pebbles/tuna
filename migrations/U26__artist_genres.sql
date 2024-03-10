CREATE TABLE IF NOT EXISTS artist_genres (artist_id TEXT NOT NULL
,   genre_id TEXT NOT NULL
,   PRIMARY KEY (artist_id, genre_id)
,   FOREIGN KEY (artist_id) REFERENCES artists(id)
,   FOREIGN KEY (genre_id) REFERENCES genres(id)
);
