CREATE TABLE IF NOT EXISTS album_tracks (track_id TEXT NOT NULL
,   album_id TEXT NOT NULL
,   PRIMARY KEY (track_id, album_id)
,   FOREIGN KEY (track_id) REFERENCES tracks(id)
,   FOREIGN KEY (album_id) REFERENCES albums(id)
);
