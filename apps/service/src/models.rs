use diesel::{prelude::*, sql_types::Integer};

use crate::schema::emote_occurrences;
use crate::schema::emote_usage;

#[derive(Queryable, Debug)]
pub struct EmoteOccurrence {
    id: Option<i32>,
    emote_name: String,
    chatter_name: String,
    channel_name: String,
    occurrence_timestamp: Option<String>,
}

#[derive(Insertable)]
#[table_name = "emote_occurrences"]
pub struct NewOccurrence<'a> {
    pub emote_name: &'a str,
    pub chatter_name: &'a str,
    pub channel_name: &'a str,
    pub occurrence_timestamp: &'a str,
}

#[derive(Insertable)]
#[table_name = "emote_usage"]
pub struct NewUsage<'a> {
    pub emote_name: &'a str,
    pub channel_name: &'a str,
    pub usage_count: i32,
}
