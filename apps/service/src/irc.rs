use std::collections::HashMap;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::fetch_bttv;
use crate::models::{NewOccurrence, NewUsage};
use crate::occurence::{add_occurance, add_occurance_old, increment_occurance};

#[derive(Clone, Debug)]
pub struct TwitchChannel {
    pub channel_name: String,
    channel_id: i32,
    pub emote_map: HashMap<String, String>,
}

impl TwitchChannel {
    pub fn new(channel_name: String, channel_id: i32) -> Self {
        TwitchChannel {
            channel_name,
            emote_map: HashMap::new(),
            channel_id,
        }
    }

    pub async fn connect(mut self) {
        self.load_emotes().await;
        self.irc_listen().await.unwrap();
    }

    async fn load_emotes(&mut self) {
        let map = fetch_bttv(self.channel_id).await.unwrap();
        println!("{:?}", map);
        self.emote_map = map;
    }

    // For now, fetch new emotes from bttv. This makes the request unreasonably long though
    pub async fn get_emotes(&mut self) -> &HashMap<String, String> {
        self.load_emotes().await;
        &self.emote_map
    }

    async fn irc_listen(self) -> Result<(), tokio::task::JoinError> {
        // default configuration is to join chat as anonymous.
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages, otherwise they will back up.
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    ServerMessage::Privmsg(msg) => {
                        let sender = msg.sender.name;
                        let text = msg.message_text;
                        println!(
                            "User: {:?}, Message: {:?}, Channel: {:?}",
                            sender, text, msg.channel_login
                        );

                        for part in text.split_whitespace() {
                            let mut timestamp = String::from(&msg.server_timestamp.to_string());
                            timestamp.truncate(timestamp.len() - 4);
                            match &self.emote_map.contains_key(part) {
                                // Not sure if this can be replaced with an _. We don't care about a non-match yet
                                true => {
                                    increment_occurance(NewUsage {
                                        channel_name: &msg.channel_login,
                                        emote_name: part,
                                        usage_count: 1,
                                    });
                                    add_occurance_old(NewOccurrence {
                                        emote_name: part,
                                        chatter_name: &sender,
                                        channel_name: &msg.channel_login,
                                        occurrence_timestamp: &timestamp,
                                    })
                                }
                                false => {}
                            }
                        }

                        for emote in msg.emotes.into_iter() {
                            // Hard code for now, this ignores emotes like ":)". Could make a lookup for those later
                            if emote.code.starts_with("moon2") {
                                add_occurance_old(NewOccurrence {
                                    emote_name: &emote.code,
                                    chatter_name: &sender,
                                    channel_name: &msg.channel_login,
                                    occurrence_timestamp: &msg.server_timestamp.to_string(),
                                })
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        // join a channel
        // This function only returns an error if the passed channel login name is malformed,
        // so in this simple case where the channel name is hardcoded we can ignore the potential
        // error with `unwrap`.
        client.join(self.channel_name).unwrap();

        // keep the tokio executor alive.
        // If you return instead of waiting the background task will exit.
        // join_handle.await.unwrap();
        join_handle.await
    }
}
