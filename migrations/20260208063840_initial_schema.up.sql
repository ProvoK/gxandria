-- Create games table
CREATE TABLE IF NOT EXISTS games (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    summary TEXT,
    storyline TEXT,
    genres TEXT,
    store_name TEXT NOT NULL,
    store_id TEXT NOT NULL
);
