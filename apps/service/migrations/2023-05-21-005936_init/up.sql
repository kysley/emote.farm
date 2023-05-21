-- Your SQL goes here
CREATE TABLE emote_usage (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  emote_name TEXT NOT NULL,
  chatter_name TEXT NOT NULL,
  channel_name TEXT NOT NULL,
  usage_count INTEGER DEFAULT 1,
  UNIQUE(emote_name, chatter_name, channel_name)
);

CREATE TABLE emote_occurrences (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  emote_name TEXT NOT NULL,
  chatter_name TEXT NOT NULL,
  channel_name TEXT NOT NULL,
  occurrence_timestamp TEXT DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now'))
);
