CREATE TABLE IF NOT EXISTS sessions (id TEXT PRIMARY KEY
,   username TEXT NOT NULL
,   FOREIGN KEY (username) REFERENCES users(username)
)
