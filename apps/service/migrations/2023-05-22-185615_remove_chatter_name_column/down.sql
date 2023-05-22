-- This file should undo anything in `up.sql`
ALTER TABLE emote_usage
ADD COLUMN chatter_name TEXT NOT NULL DEFAULT '';
