CREATE TABLE IF NOT EXISTS invite_permissions (id TEXT NOT NULL
,   code TEXT NOT NULL
,   PRIMARY KEY (id, code)
,   FOREIGN KEY (id) REFERENCES permissions(id) ON DELETE CASCADE ON UPDATE CASCADE
,   FOREIGN KEY (code) REFERENCES invites(code) ON DELETE CASCADE ON UPDATE CASCADE
);

