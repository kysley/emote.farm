use std::collections::HashMap;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::models::{NewOccurrence, NewUsage};
use crate::occurence::{add_occurance, increment_occurance};

pub async fn connect_to_irc(emote_map: HashMap<String, String>) {
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
                        "User: {:?}, Message: {:?}, Emote: {:?}",
                        sender,
                        text,
                        msg.emotes.len()
                    );

                    for part in text.split_whitespace() {
                        match emote_map.contains_key(part) {
                            // Not sure if this can be replaced with an _. We don't care about a non-match yet
                            true => {
                                increment_occurance(NewUsage {
                                    channel_name: &msg.channel_login,
                                    emote_name: part,
                                    usage_count: 1,
                                });
                                add_occurance(NewOccurrence {
                                    emote_name: part,
                                    chatter_name: &sender,
                                    channel_name: &msg.channel_login,
                                    occurrence_timestamp: &msg.server_timestamp.to_string(),
                                })
                            }
                            false => {}
                        }
                    }

                    for emote in msg.emotes.into_iter() {
                        // Hard code for now, this ignores emotes like ":)". Could make a lookup for those later
                        if emote.code.starts_with("moon2") {
                            add_occurance(NewOccurrence {
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
    client.join("moonmoon".to_owned()).unwrap();

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
