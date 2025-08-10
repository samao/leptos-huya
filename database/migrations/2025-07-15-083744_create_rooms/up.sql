-- 启用外键和WAL模式
PRAGMA foreign_keys = ON;

-- 创建 users 表
CREATE TABLE
    IF NOT EXISTS users (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        user_name TEXT NOT NULL CHECK (length (user_name) BETWEEN 3 AND 8),
        avatar TEXT NOT NULL DEFAULT '//dummyimg/90/90',
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS cates (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        icon_url TEXT NOT NULL DEFAULT '/imgs/game/game_15735501549253_logo.png',
        img_url TEXT NOT NULL DEFAULT '/imgs/game/big_img.png',
        cate_name TEXT NOT NULL CHECK (length (cate_name) BETWEEN 2 AND 12),
        live_total INTEGER NOT NULL DEFAULT 0
    );

-- 创建 rooms 表
CREATE TABLE
    IF NOT EXISTS rooms (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL CHECK (length (title) BETWEEN 6 AND 14),
        is_live BOOLEAN NOT NULL DEFAULT 0 CHECK (is_live IN (0, 1)),
        img_url TEXT NOT NULL DEFAULT '//dummyimg/120/90',
        hot INTEGER NOT NULL DEFAULT 0 CHECK (hot >= 0),
        user_id INTEGER NOT NULL,
        cate_id INTEGER NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
        FOREIGN KEY (cate_id) REFERENCES cates (id) ON DELETE CASCADE
    );

-- 创建 tags 表
CREATE TABLE
    IF NOT EXISTS tags (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL UNIQUE CHECK (length (title) BETWEEN 1 AND 5),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

-- 创建 rooms_tags 关联表
CREATE TABLE
    IF NOT EXISTS rooms_tags (
        room_id INTEGER NOT NULL,
        tag_id INTEGER NOT NULL,
        PRIMARY KEY (room_id, tag_id),
        FOREIGN KEY (room_id) REFERENCES rooms (id) ON DELETE CASCADE,
        FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
    );

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_rooms_user ON rooms (user_id);

CREATE INDEX IF NOT EXISTS idx_rooms_tags_room ON rooms_tags (room_id);

CREATE INDEX IF NOT EXISTS idx_rooms_tags_tag ON rooms_tags (tag_id);

-- 创建更新触发器
CREATE TRIGGER update_room_timestamp AFTER
UPDATE ON rooms FOR EACH ROW BEGIN
UPDATE rooms
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = NEW.id;

END;