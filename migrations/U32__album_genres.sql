CREATE TABLE IF NOT EXISTS album_genres (album_id TEXT NOT NULL
,   genre_id TEXT NOT NULL
,   PRIMARY KEY (album_id, genre_id)
,   FOREIGN KEY (album_id) REFERENCES albums(id)
,   FOREIGN KEY (genre_id) REFERENCES genres(id)
);
