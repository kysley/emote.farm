use std::{collections::HashMap, net::SocketAddr};

use axum::{extract::Path, routing::get, Router};
use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, sql_types::Integer};
mod irc;
mod models;
mod occurence;
mod schema;

use irc::connect_to_irc;
use models::Usage;
use schema::emote_usage::{self};
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

    Ok(emote_map)
}

#[tokio::main]
async fn main() {
    let emote_map = fetch_bttv().await.unwrap();

    let server_task = tokio::spawn(async move {
        let app = Router::new()
            .route("/", get(root))
            .route("/totals/:channelName", get(get_emote_totals));
        let addr = SocketAddr::from(([127, 0, 0, 1], 8012));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
        println!("listening on {}", addr);
    });

    connect_to_irc(emote_map).await;
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_emote_totals(Path(channel_name): Path<String>) -> String {
    let mut connection = establish_connection();

    // Select emote useage amount and name
    let emote_usages = emote_usage::table
        .select((emote_usage::usage_count.nullable(), emote_usage::emote_name))
        .filter(emote_usage::channel_name.eq(channel_name))
        .load::<Usage>(&mut connection)
        .unwrap();

    let result = serde_json::to_string(&emote_usages).unwrap();

    result
}
