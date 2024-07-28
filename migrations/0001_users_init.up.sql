-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    -- just rowid
    'id' INTEGER PRIMARY KEY AUTOINCREMENT,

    -- just username
    'username' TEXT NOT NULL UNIQUE,

    -- 0: reader, 1: writer, 2: admin
    'perms' INTEGER NOT NULL CHECK (perms >= 0 AND perms <= 2),

    -- 32 bytes for the salt (minimum size is output size of hash function)
    'salt' BLOB NOT NULL UNIQUE CHECK (length(salt) >= 32),

    -- 32 bytes for the hash (output size of SHA-256)
    'hash' BLOB NOT NULL UNIQUE CHECK (length(hash) = 32),

    -- user creation time
    'created_at' TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- user last updated time
    'updated_at' TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);