-- Your SQL goes here
DROP TABLE emote_usage;

CREATE TABLE emote_usage (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  emote_name TEXT NOT NULL,
  channel_name TEXT NOT NULL,
  usage_count INTEGER DEFAULT 1,
  UNIQUE(emote_name, channel_name)
);
