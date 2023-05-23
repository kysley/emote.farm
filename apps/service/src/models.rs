use diesel::prelude::*;
use serde::Serialize;

use crate::schema::emote_occurrences;
use crate::schema::emote_usage;

#[derive(Queryable, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub usage_count: Option<i32>,
    pub emote_name: String,
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
