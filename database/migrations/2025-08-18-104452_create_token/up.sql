-- Your SQL goes here
PRAGMA foreign_keys = ON;
-- 创建登录Token
CREATE TABLE
    IF NOT EXISTS tokens (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        token TEXT NOT NULL UNIQUE,
        create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expired_at TIMESTAMP NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (id) on DELETE CASCADE
    );

CREATE INDEX IF NOT EXISTS idx_token_user on users (id);
