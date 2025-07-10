-- Your SQL goes here
CREATE TABLE drinks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    base_price TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);