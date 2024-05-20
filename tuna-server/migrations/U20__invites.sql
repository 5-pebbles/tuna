CREATE TABLE IF NOT EXISTS invites (code TEXT PRIMARY KEY
,   permissions TEXT NOT NULL DEFAULT ''
,   remaining INTEGER NOT NULL DEFAULT  1
,   creator TEXT NOT NULL
,   FOREIGN KEY (creator) REFERENCES users(username)
)
