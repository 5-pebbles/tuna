CREATE TABLE IF NOT EXISTS tokens (id TEXT PRIMARY KEY
,   username TEXT NOT NULL
,   FOREIGN KEY (username) REFERENCES users(username)
)
