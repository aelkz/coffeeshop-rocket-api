-- Your SQL goes here
CREATE TABLE customers (
   id TEXT PRIMARY KEY,
   name TEXT NOT NULL,
   email TEXT NOT NULL UNIQUE,
   created_at TEXT NOT NULL,
   updated_at TEXT NOT NULL,
   deleted_at TEXT
);