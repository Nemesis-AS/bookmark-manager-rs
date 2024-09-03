-- Your SQL goes here
CREATE TABLE tags (
    id Text NOT NULL PRIMARY KEY NOT NULL,
    title Text NOT NULL,
    -- icon,
    color Text NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)