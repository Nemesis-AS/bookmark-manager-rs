CREATE TABLE bookmarks (
  id Text PRIMARY KEY,
  url TEXT NOT NULL,
  title TEXT NOT NULL,
  description Text NOT NULL,
  tags TEXT NOT NULL DEFAULT ""
--   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)