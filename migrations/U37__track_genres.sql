CREATE TABLE IF NOT EXISTS track_genres (track_id TEXT NOT NULL
,   genre_id TEXT NOT NULL
,   PRIMARY KEY (track_id, genre_id)
,   FOREIGN KEY (track_id) REFERENCES tracks(id)
,   FOREIGN KEY (genre_id) REFERENCES genre(id)
);
