CREATE TABLE IF NOT EXISTS tracks (id TEXT PRIMARY KEY
,   name TEXT NOT NULL
,   release INTEGER NOT NULL DEFAULT  0
,   duration INTEGER NOT NULL DEFAULT  0
,   lyrics TEXT NOT NULL DEFAULT ''
);
