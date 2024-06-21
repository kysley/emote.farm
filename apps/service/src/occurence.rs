use anyhow::Result;
use edgedb_tokio::Queryable;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    establish_connection,
    models::{NewOccurrence, NewUsage},
    schema::{emote_occurrences, emote_usage},
};
use diesel::prelude::*;

pub fn add_occurance_old(occurance: NewOccurrence) {
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
#[derive(Debug, Queryable)]
struct EmoteUsage {
    emote_name: String,
}
pub async fn get_usage() -> Result<()> {
    let conn = edgedb_tokio::create_client().await?;
    let result = conn
        .query::<EmoteUsage, _>("select EmoteUsage {emote_name}", &())
        .await?;
    // .expect("WHAT WENT WRONG");

    // let result2 = conn.query::<String, _>("SELECT 'hello'", &()).await?;
    // .expect("WHAT WENT WRONG");

    println!("RESULT {:?}", result);
    // let json = serde_json::to_string(&result).unwrap();
    // println!("RESULT {:?}", json);

    Ok(())
}

pub async fn add_usage(emote_name: String, channel_name: String) -> Result<()> {
    let conn = edgedb_tokio::create_client().await?;
    let result = conn
        .query_json(
            "
        insert EmoteUsage {
            channel_name := <string>$0,
            emote_name := <string>$1,
            usage_count := 1,
        }
        unless conflict on (.channel_name, .emote_name)
        else (
            update EmoteUsage
            set {
                usage_count := .usage_count + 1
            }
        )
    ",
            &(emote_name, channel_name),
        )
        .await?;

    Ok(())
}

#[derive(Debug, Queryable)]
struct EmoteOccurance {
    id: String,
}
pub async fn add_occurance(
    channel_name: String,
    chatter_name: String,
    emote_name: String,
    timestamp: String,
) -> Result<()> {
    let conn = edgedb_tokio::create_client().await?;
    let result = conn
        .query::<EmoteOccurance, _>(
            "
        insert EmoteOccurance {
            channel_name := <string>$0
            chatter_name := <string>$1
            emote_name := <string>$2
            timestamp := <string>$3
        }
    ",
            &(channel_name, chatter_name, emote_name, timestamp),
        )
        .await?;

    Ok(())
}
