use chrono::Duration;
use diesel::dsl::{count, sql};
use std::{collections::HashMap, net::SocketAddr};

use axum::extract::Query;
use axum::Extension;
use axum::{extract::Path, routing::get, Router};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
mod irc;
mod models;
mod occurence;
mod schema;
use irc::TwitchChannel;
use models::Usage;
use schema::emote_occurrences;
use schema::emote_usage::{self};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

use crate::models::EmoteCount;

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

async fn fetch_bttv(channel_id: i32) -> Result<HashMap<String, String>, reqwest::Error> {
    // let channel_id = 121059319;
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
    let channels: Vec<TwitchChannel> = vec![
        TwitchChannel::new("moonmoon".to_string(), 121059319),
        TwitchChannel::new("trainwreckstv".to_string(), 71190292),
    ];

    let channels_extension = Extension(channels.to_owned());

    tokio::spawn(async move {
        let app = Router::new()
            .route("/", get(root))
            .route("/channel/:channel_name/since", get(get_emotes_since))
            .route("/channel/:channel_name/totals", get(get_totals))
            .route("/channel/:channel_name/emotes", get(get_ids))
            .layer(channels_extension)
            .layer(CorsLayer::new().allow_origin(Any));
        let addr = SocketAddr::from(([127, 0, 0, 1], 8012));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
        println!("listening on {}", addr);
    });

    let mut tasks = Vec::new();

    for channel in channels {
        let task = tokio::spawn(async move {
            channel.connect().await;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Debug, Deserialize)]
struct TotalsRangeRequest {
    since: Option<i32>,
}
async fn get_emotes_since(
    Path(channel_name): Path<String>,
    Query(TotalsRangeRequest { since }): Query<TotalsRangeRequest>,
) -> String {
    let mut connection = establish_connection();

    let results: Vec<(String, i64)> = emote_occurrences::table
        .group_by(emote_occurrences::emote_name)
        .filter(emote_occurrences::channel_name.eq(channel_name).and(
            emote_occurrences::occurrence_timestamp.ge(sql(&format!(
                "datetime('now', '-{} hours')",
                since.unwrap_or_else(|| 1)
            ))),
        ))
        .select((
            emote_occurrences::emote_name,
            count(emote_occurrences::emote_name),
        ))
        .load::<(String, i64)>(&mut connection)
        .unwrap();

    let transformed_results: Vec<EmoteCount> = results
        .into_iter()
        .map(|(name, count)| EmoteCount {
            emote_name: name,
            count,
        })
        .collect();

    let result = serde_json::to_string(&transformed_results).unwrap();

    result
}

async fn get_totals(Path(channel_name): Path<String>) -> String {
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
// For now this acts as a way to indirectly refresh the emote map for irc
async fn get_ids(
    Extension(channels): Extension<Vec<TwitchChannel>>,
    Path(channel_name): Path<String>,
) -> String {
    let channel = channels
        .iter()
        .find(|c| c.channel_name == channel_name)
        .unwrap();
    serde_json::to_string(channel.to_owned().get_emotes().await).unwrap()
}
