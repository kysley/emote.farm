use std::collections::HashMap;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
mod irc;
mod models;
mod occurence;
mod schema;

use irc::connect_to_irc;
use serde::Deserialize;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "db.sqlite";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
#[derive(Debug, Deserialize)]
struct BTTVEmote {
    id: String,
    code: String,
}
#[derive(Debug, Deserialize)]
struct BTTVResponse {
    channelEmotes: Vec<BTTVEmote>,
    sharedEmotes: Vec<BTTVEmote>,
}

async fn fetch_bttv() -> Result<HashMap<String, String>, reqwest::Error> {
    let channel_id = 121059319;
    let url = format!(
        "https://api.betterttv.net/3/cached/users/twitch/{}",
        channel_id
    );
    let res: BTTVResponse = reqwest::Client::new().get(url).send().await?.json().await?;
    let mut emote_map: HashMap<String, String> = HashMap::new();

    for emote in res.channelEmotes.into_iter() {
        emote_map.insert(emote.code, emote.id);
    }

    for emote in res.sharedEmotes.into_iter() {
        emote_map.insert(emote.code, emote.id);
    }

    for (key, value) in &emote_map {
        println!("{}: {}", key, value);
    }

    Ok(emote_map)
}

#[tokio::main]
async fn main() {
    let emote_map = fetch_bttv().await.unwrap();

    connect_to_irc(emote_map).await;
}
