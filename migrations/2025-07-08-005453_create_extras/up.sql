-- Your SQL goes here
CREATE TABLE extras (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    extra_price TEXT NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT 1
);