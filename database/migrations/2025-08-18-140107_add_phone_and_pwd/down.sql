-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN phone;
ALTER TABLE users DROP COLUMN password;
