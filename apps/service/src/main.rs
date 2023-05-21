use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod schema;
use schema::emote_occurrences;

#[derive(Queryable, Debug)]
struct EmoteOccurrence {
    id: Option<i32>,
    emote_name: String,
    chatter_name: String,
    channel_name: String,
    occurrence_timestamp: Option<String>,
}

#[derive(Insertable)]
#[table_name = "emote_occurrences"]
struct NewOccurrence<'a> {
    emote_name: &'a str,
    chatter_name: &'a str,
    channel_name: &'a str,
    occurrence_timestamp: &'a str,
}

fn main() {
    let mut connection = establish_connection();

    let new_occurence = NewOccurrence {
        emote_name: "Kappa",
        chatter_name: "swan1",
        channel_name: "moonmoon",
        occurrence_timestamp: "5/20/2023, 9:36:47 PM",
    };

    diesel::insert_into(emote_occurrences::table).values(&new_occurence).execute(&mut connection).ok();

    let results = emote_occurrences::table
        .load::<EmoteOccurrence>(&mut connection)
        .expect("Error loading emote occurrences");

    for occurrence in results {
        println!("{:?}", occurrence);
    }
}

fn establish_connection() -> SqliteConnection {
    let database_url = "db.sqlite";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
