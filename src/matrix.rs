use crate::{Config, Error};
use matrix_sdk::Client;
use matrix_sdk::{config::SyncSettings, room::MessagesOptions};
use morum_base::types;
use regex::Regex;
use ruma::events::{
    room::name::RoomNameEventContent, room::topic::RoomTopicEventContent, EmptyStateKey,
    RedactContent, RedactedStateEventContent, StateEventContent, SyncStateEvent,
};
use ruma::serde::Raw;
use ruma::{assign, OwnedRoomId, RoomAliasId, RoomId, RoomOrAliasId};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};
use tokio::task;
use tracing::info;
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

    pub async fn categories(&self) -> Result<Vec<types::Category>, Error> {
        use ruma::events::space::child::SpaceChildEventContent;

        let toplevel_room_id = self
            .client
            .resolve_room_alias("#forum:corepaper.org".try_into()?)
            .await?
            .room_id;
        let toplevel_room = self
            .client
            .get_joined_room(&toplevel_room_id)
            .ok_or(Error::UnknownToplevelRoom)?;

        let child_state_events = toplevel_room
            .get_state_events_static::<SpaceChildEventContent>()
            .await?;
        let children: Vec<OwnedRoomId> = child_state_events
            .into_iter()
            .filter_map(|event| {
                if let Ok(event) = event.deserialize() {
                    Some(event.state_key().clone())
                } else {
                    None
                }
            })
            .collect();

        let mut categories = Vec::new();
        for child in children {
            let room = self
                .client
                .get_joined_room(&child)
                .ok_or(Error::UnknownCategoryRoom)?;

            let room_name_state_events = room
                .get_state_events_static::<RoomNameEventContent>()
                .await?;
            let room_topic_state_events = room
                .get_state_events_static::<RoomTopicEventContent>()
                .await?;

            let title = deserialize_sync_state_events_to_content(room_name_state_events)?
                .and_then(|e| e.name)
                .ok_or(Error::UnknownCategoryTitle)?;
            let topic = deserialize_sync_state_events_to_content(room_topic_state_events)?
                .map(|e| e.topic)
                .ok_or(Error::UnknownCategoryTopic)?;

            let room_alias = room.canonical_alias().ok_or(Error::InvalidCategoryAlias)?;
            let re = Regex::new(r"^#forum-(.+):corepaper\.org$").expect("regex is valid");
            let captures = re.captures(room_alias.as_str());

            let room_local_id = if let Some(captures) = captures {
                captures
                    .get(1)
                    .ok_or(Error::InvalidCategoryAlias)?
                    .as_str()
                    .to_owned()
            } else {
                return Err(Error::InvalidCategoryAlias);
            };

            categories.push(types::Category {
                title,
                topic,
                room_local_id,
            });
        }

        Ok(categories)
    }

    pub async fn category_posts(
        &self,
        room_alias: String,
    ) -> Result<(types::Category, Vec<types::Post>), Error> {
        use ruma::events::space::child::SpaceChildEventContent;

        let category_room_id = self
            .client
            .resolve_room_alias(&RoomAliasId::parse(&room_alias)?)
            .await?
            .room_id;
        let category_room = self
            .client
            .get_joined_room(&category_room_id)
            .ok_or(Error::UnknownCategoryRoom)?;

        let category_room_name_state_events = category_room
            .get_state_events_static::<RoomNameEventContent>()
            .await?;
        let category_room_topic_state_events = category_room
            .get_state_events_static::<RoomTopicEventContent>()
            .await?;

        let category_title =
            deserialize_sync_state_events_to_content(category_room_name_state_events)?
                .and_then(|e| e.name)
                .ok_or(Error::UnknownCategoryTitle)?;
        let category_topic =
            deserialize_sync_state_events_to_content(category_room_topic_state_events)?
                .map(|e| e.topic)
                .ok_or(Error::UnknownCategoryTopic)?;

        let category_room_alias = category_room
            .canonical_alias()
            .ok_or(Error::InvalidCategoryAlias)?;
        let re = Regex::new(r"^#forum-(.+):corepaper\.org$").expect("regex is valid");
        let captures = re.captures(category_room_alias.as_str());

        let category_room_local_id = if let Some(captures) = captures {
            captures
                .get(1)
                .ok_or(Error::InvalidCategoryAlias)?
                .as_str()
                .to_owned()
        } else {
            return Err(Error::InvalidCategoryAlias);
        };

        let category = types::Category {
            title: category_title,
            topic: category_topic,
            room_local_id: category_room_local_id,
        };

        let child_state_events = category_room
            .get_state_events_static::<SpaceChildEventContent>()
            .await?;
        let children: Vec<OwnedRoomId> = child_state_events
            .into_iter()
            .filter_map(|event| {
                if let Ok(event) = event.deserialize() {
                    Some(event.state_key().clone())
                } else {
                    None
                }
            })
            .collect();

        let mut posts = Vec::new();
        for child in children {
            let room = self
                .client
                .get_joined_room(&child)
                .ok_or(Error::UnknownCategoryRoom)?;

            let room_name_state_events = room
                .get_state_events_static::<RoomNameEventContent>()
                .await?;
            let room_topic_state_events = room
                .get_state_events_static::<RoomTopicEventContent>()
                .await?;

            let title = deserialize_sync_state_events_to_content(room_name_state_events)?
                .and_then(|e| e.name)
                .ok_or(Error::UnknownPostTitle)?;
            let topic =
                deserialize_sync_state_events_to_content(room_topic_state_events)?.map(|e| e.topic);

            let room_id = room.room_id();

            posts.push(types::Post {
                title,
                topic,
                room_id: room_id.as_str().to_owned(),
            });
        }

        Ok((category, posts))
    }

    pub async fn post_comments(
        &self,
        room_id: String,
    ) -> Result<(types::Post, Vec<types::Comment>), Error> {
        use ruma::events::room::message::{
            sanitize::{HtmlSanitizerMode, RemoveReplyFallback},
            FormattedBody, MessageFormat, MessageType, Relation,
        };
        use ruma::events::{AnyMessageLikeEvent, AnyTimelineEvent, MessageLikeEvent};

        let room = self
            .client
            .get_joined_room(&RoomId::parse(&room_id)?)
            .ok_or(Error::UnknownPost)?;

        let room_name_state_events = room
            .get_state_events_static::<RoomNameEventContent>()
            .await?;
        let room_topic_state_events = room
            .get_state_events_static::<RoomTopicEventContent>()
            .await?;

        let title = deserialize_sync_state_events_to_content(room_name_state_events)?
            .and_then(|e| e.name)
            .ok_or(Error::UnknownPostTitle)?;
        let topic =
            deserialize_sync_state_events_to_content(room_topic_state_events)?.map(|e| e.topic);

        let post = types::Post {
            title,
            topic,
            room_id: room_id,
        };

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
                let (event_id, content) = match message.content.relates_to {
                    Some(Relation::Replacement(replacement)) => (
                        replacement.event_id,
                        replacement.new_content.as_ref().clone(),
                    ),
                    _ => (message.event_id, message.content),
                };

                if let MessageType::Text(msgtype) = content.msgtype {
                    if let Some((i, _)) = messages
                        .iter()
                        .enumerate()
                        .find(|(_, (other_event_id, _, _))| *other_event_id == event_id)
                    {
                        let item = messages.remove(i);
                        messages.push(item);
                    } else {
                        messages.push((event_id, sender, msgtype));
                    }
                }
            }
        }

        let mut comments = Vec::new();
        for (_, sender, message) in messages {
            let mut message = message.formatted.unwrap_or_else(|| {
                let mut html_body = String::new();

                pulldown_cmark::html::push_html(
                    &mut html_body,
                    pulldown_cmark::Parser::new(&message.body),
                );
                FormattedBody::html(html_body)
            });

            if message.format == MessageFormat::Html {
                message.sanitize_html(HtmlSanitizerMode::Strict, RemoveReplyFallback::Yes);
                let html = message.body;

                comments.push(types::Comment {
                    sender: sender.as_str().to_owned(),
                    html,
                });
            }
        }

        comments.reverse();

        Ok((post, comments))
    }

    pub async fn add_room_to_space(
        &self,
        category_room_alias: String,
        new_room_alias_or_id: String,
    ) -> Result<(), Error> {
        use ruma::events::space::child::SpaceChildEventContent;

        let category_room_id = self
            .client
            .resolve_room_alias(&RoomAliasId::parse(&category_room_alias)?)
            .await?
            .room_id;
        let category_room = self
            .client
            .get_joined_room(&category_room_id)
            .ok_or(Error::UnknownCategoryRoom)?;

        let new_room_alias_or_id = RoomOrAliasId::parse(new_room_alias_or_id)?;
        let new_room_id = self
            .client
            .join_room_by_id_or_alias(&new_room_alias_or_id, &[])
            .await?
            .room_id;

        category_room
            .send_state_event_for_key(
                &new_room_id,
                assign!(SpaceChildEventContent::new(), {
                    via: Some(vec!["corepaper.org".try_into()?]),
                }),
            )
            .await?;

        self.client
            .sync_once(SyncSettings::default().full_state(true))
            .await?;

        Ok(())
    }
}

pub async fn start(config: Config) -> Result<MatrixService, Error> {
    let matrix =
        MatrixService::new(config.homeserver_url, config.username, config.password).await?;

    matrix
        .post_comments("!AZvsRzlxPPMqKlMwMB:pacna.org".to_string())
        .await?;

    Ok(matrix)
}
