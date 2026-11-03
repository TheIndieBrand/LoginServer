CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    avatar TEXT,
    created_at INTEGER NOT NULL,
    last_login INTEGER NOT NULL
);
