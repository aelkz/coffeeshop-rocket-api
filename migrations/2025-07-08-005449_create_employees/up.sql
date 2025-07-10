-- Your SQL goes here
CREATE TABLE employees (
   id TEXT PRIMARY KEY,
   name TEXT NOT NULL,
   email TEXT NOT NULL UNIQUE,
   birth_date TEXT NOT NULL,
   created_at TEXT NOT NULL,
   updated_at TEXT NOT NULL,
   deleted_at TEXT
);