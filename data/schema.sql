PRAGMA user_version = 1;

CREATE TABLE IF NOT EXISTS kv (key TEXT, value TEXT);

CREATE TABLE groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    epoch INTEGER,
    name TEXT NOT NULL UNIQUE,
    schedule JSONB
);
