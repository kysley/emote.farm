use crate::{
    establish_connection,
    models::{NewOccurrence, NewUsage},
    schema::{emote_occurrences, emote_usage},
};
use diesel::prelude::*;

pub fn add_occurance(occurance: NewOccurrence) {
    let mut connection = establish_connection();

    diesel::insert_into(emote_occurrences::table)
        .values(&occurance)
        .execute(&mut connection)
        .ok();
}

pub fn increment_occurance(usage: NewUsage) {
    let mut connection = establish_connection();

    let result = diesel::insert_into(emote_usage::table)
        .values(&usage)
        .on_conflict((emote_usage::emote_name, emote_usage::channel_name))
        .do_update()
        .set(emote_usage::usage_count.eq(emote_usage::usage_count + 1))
        .execute(&mut connection);

    if let Err(err) = result {
        eprintln!("Error occurred while incrementing occurrence: {}", err);
    }
}
