use crate::{Config, Error};
use matrix_sdk::Client;
use matrix_sdk::{config::SyncSettings, room::MessagesOptions};
use regex::Regex;
use ruma::events::{
    room::name::RoomNameEventContent, room::topic::RoomTopicEventContent, EmptyStateKey,
    RedactContent, RedactedStateEventContent, StateEventContent, SyncStateEvent,
};
use ruma::serde::Raw;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};
use tokio::task;
use tracing::{debug, info};
use url::Url;

fn deserialize_sync_state_events_to_content<C>(
    events: Vec<Raw<SyncStateEvent<C>>>,
) -> Result<Option<C>, Error>
where
    C: StateEventContent + RedactContent,
    C::Redacted: RedactedStateEventContent<StateKey = C::StateKey>,
{
    Ok(if let Some(event) = events.first() {
        let event = event.deserialize()?;

        if let SyncStateEvent::Original(event) = event {
            Some(event.content)
        } else {
            None
        }
    } else {
        None
    })
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "org.corepaper.morum.category", kind = State, state_key_type = EmptyStateKey)]
pub struct MorumCategoryEventContent {
    #[serde(default, deserialize_with = "ruma::serde::empty_string_as_none")]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Room {
    pub title: String,
    pub topic: Option<String>,
    pub category: Option<String>,
    pub post_id: usize,
    pub room_id: String,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Message {
    pub html: String,
    pub sender: String,
}

pub struct MatrixService {
    client: Client,
}

impl MatrixService {
    pub async fn new(
        homeserver_url: String,
        username: String,
        password: String,
    ) -> Result<Self, Error> {
        let client = Client::new(Url::parse(&homeserver_url)?).await?;

        let login_res = client
            .login_username(&username, &password)
            .device_id("morum")
            .initial_device_display_name("Morum")
            .send()
            .await?;

        client
            .sync_once(SyncSettings::default().full_state(true))
            .await?;

        let client_sync = client.clone();
        task::spawn(async move {
            client_sync
                .sync(SyncSettings::default().full_state(true))
                .await?;

            Ok::<(), matrix_sdk::Error>(())
        });

        info!(
            "Logged in as {}, got device_id {}",
            username, login_res.device_id,
        );

        Ok(Self { client })
    }

    pub async fn valid_rooms(&self) -> Result<Vec<Room>, Error> {
        let mut ret = Vec::new();

        let joined_rooms = self.client.joined_rooms();

        for room in joined_rooms {
            debug!("room id: {:?}", room.room_id());

            let category_state_events = room
                .get_state_events_static::<MorumCategoryEventContent>()
                .await?;
            let room_name_state_events = room
                .get_state_events_static::<RoomNameEventContent>()
                .await?;
            let room_topic_state_events = room
                .get_state_events_static::<RoomTopicEventContent>()
                .await?;

            let title = deserialize_sync_state_events_to_content(room_name_state_events)?
                .and_then(|e| e.name);
            let topic =
                deserialize_sync_state_events_to_content(room_topic_state_events)?.map(|e| e.topic);
            let category = deserialize_sync_state_events_to_content(category_state_events)?
                .and_then(|e| e.category);

            let aliases = room
                .canonical_alias()
                .into_iter()
                .chain(room.alt_aliases().into_iter());

            let mut post_id = None;
            for alias in aliases {
                let re = Regex::new(r"^#forum_post_(\d+):corepaper\.org$").expect("regex is valid");
                let captures = re.captures(alias.as_str());

                if let Some(captures) = captures {
                    post_id = captures
                        .get(1)
                        .and_then(|s| s.as_str().parse::<usize>().ok());
                }
            }

            debug!(
                "title: {:?}, topic: {:?}, category: {:?}, post id: {:?}",
                title, topic, category, post_id
            );

            if let (Some(title), Some(post_id)) = (title, post_id) {
                ret.push(Room {
                    title,
                    category,
                    post_id,
                    topic,
                    room_id: room.room_id().as_str().to_owned(),
                })
            }
        }

        Ok(ret)
    }

    pub async fn messages(&self, room_alias_id: &str) -> Result<Vec<Message>, Error> {
        use ruma::events::room::message::{
            sanitize::{HtmlSanitizerMode, RemoveReplyFallback},
            MessageFormat, MessageType,
        };
        use ruma::events::{AnyMessageLikeEvent, AnyTimelineEvent, MessageLikeEvent};

        let room_id = self
            .client
            .resolve_room_alias(room_alias_id.try_into()?)
            .await?
            .room_id;
        let room = self
            .client
            .get_joined_room(&room_id)
            .ok_or(Error::UnknownPost)?;

        let types_filter = ["m.room.message".to_string()];
        let mut messages_options = MessagesOptions::backward();
        messages_options.limit = js_int::UInt::MAX;
        messages_options.filter.types = Some(&types_filter);

        let messages_chunk = room.messages(messages_options).await?.chunk;

        let mut messages = Vec::new();
        for message_raw in messages_chunk {
            let message = message_raw.event.deserialize()?;

            if let AnyTimelineEvent::MessageLike(AnyMessageLikeEvent::RoomMessage(
                MessageLikeEvent::Original(message),
            )) = message
            {
                let sender = message.sender;

                if let MessageType::Text(message) = message.content.msgtype {
                    if let Some(mut message) = message.formatted {
                        if message.format == MessageFormat::Html {
                            message
                                .sanitize_html(HtmlSanitizerMode::Strict, RemoveReplyFallback::Yes);
                            let html = message.body;

                            messages.push(Message {
                                sender: sender.as_str().to_owned(),
                                html,
                            });
                        }
                    }
                }
            }
        }

        messages.reverse();

        Ok(messages)
    }
}

pub async fn start(config: Config) -> Result<MatrixService, Error> {
    let matrix =
        MatrixService::new(config.homeserver_url, config.username, config.password).await?;

    Ok(matrix)
}
