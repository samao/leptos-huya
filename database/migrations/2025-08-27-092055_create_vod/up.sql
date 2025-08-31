-- Your SQL goes here

CREATE TABLE 
    IF NOT EXISTS vod_cate (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        cate_name TEXT NOT NULL CHECK (length (cate_name) BETWEEN 2 AND 20),
        img_url TEXT NOT NULL
    );

CREATE TABLE 
    IF NOT EXISTS sub_cate (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        vod_cate_id INTEGER NOT NULL,
        cate_name TEXT NOT NULL CHECK (length (cate_name) BETWEEN 2 AND 20),
        FOREIGN KEY (vod_cate_id) REFERENCES vod_cate (id) on DELETE CASCADE
    );
    
CREATE TABLE 
    IF NOT EXISTS vods (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        img_url TEXT NOT NULL,
        duration INTEGER NOT NULL DEFAULT 0,
        user_id INTEGER NOT NULL,
        title TEXT NOT NULL,
        vod_cate_id INTEGER NOT NULL,
        hots INTEGER NOT NULL DEFAULT 0,
        FOREIGN KEY (user_id) REFERENCES users (id) on DELETE CASCADE,
        FOREIGN KEY (vod_cate_id) REFERENCES vod_cate (id) on DELETE CASCADE
    );

CREATE INDEX IF NOT EXISTS idx_sub_cate_to_cates ON sub_cate (vod_cate_id);
CREATE INDEX IF NOT EXISTS idx_vod_to_cate ON vods (vod_cate_id);
CREATE INDEX IF NOT EXISTS idx_vod_to_user ON vods (user_id);
