mod client;

pub use self::client::Client;

use crate::{Config, Error};
use regex::Regex;
use ruma::serde::Raw;
use ruma::events::{SyncStateEvent, AnyStateEvent, EmptyStateKey, StaticEventContent, StateEvent, StateEventContent, RedactedStateEventContent, RedactContent, room::name::RoomNameEventContent, room::topic::RoomTopicEventContent};
use ruma_macros::EventContent;
use matrix_sdk::{config::SyncSettings, room::MessagesOptions};
use serde::{Deserialize, de::DeserializeOwned, Serialize};
use tokio::task;
use tracing::debug;

fn deserialize_sync_state_events_to_content<C>(events: Vec<Raw<SyncStateEvent<C>>>) -> Result<Option<C>, Error> where
    C: StateEventContent + RedactContent,
    C::Redacted: RedactedStateEventContent<StateKey=C::StateKey>,
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

#[derive(Debug)]
pub struct AppService {
    client: self::client::Client,
    forum_user: self::client::UserClient,
}

impl AppService {
    pub async fn new(homeserver_url: String, access_token: String) -> Result<Self, Error> {
        let client = self::client::Client::new(homeserver_url, access_token).await?;
        let forum_user = client.user("forum".try_into().expect("forum is valid user id")).await?;
        forum_user.sync_once(SyncSettings::default().full_state(true)).await?;

        let forum_user_sync = forum_user.clone();

        task::spawn(async move {
            forum_user_sync.sync(SyncSettings::default().full_state(true)).await?;

            Ok::<(), matrix_sdk::Error>(())
        });

        Ok(Self {
            client, forum_user,
        })
    }

    pub async fn ensure_registered(&self, localpart: &str) -> Result<(), Error> {
        use ruma::api::client::account::register::{v3::Request, LoginType, RegistrationKind};

        let mut request = Request::new();
        request.username = Some(localpart);
        request.device_id = Some("morum".try_into()?);
        request.kind = RegistrationKind::User;
        request.inhibit_login = true;
        request.login_type = Some(&LoginType::ApplicationService);
        request.refresh_token = false;

        let response = self.client.send_request_force_auth(request).await;

        match response {
            Err(ruma::client::Error::FromHttpResponse(
                ruma::api::error::FromHttpResponseError::Server(
                    ruma::api::error::ServerError::Known(
                        ruma::api::client::uiaa::UiaaResponse::MatrixError(
                            ruma::api::client::Error { kind, .. },
                        ),
                    ),
                ),
            )) if kind == ruma::api::client::error::ErrorKind::UserInUse => Ok(()),
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }

    pub async fn valid_rooms(&self) -> Result<Vec<Room>, Error> {
        let mut ret = Vec::new();

        let joined_rooms = self.forum_user.joined_rooms();

        for room in joined_rooms {
            debug!("room id: {:?}", room.room_id());

            let category_state_events = room.get_state_events_static::<MorumCategoryEventContent>().await?;
            let room_name_state_events = room.get_state_events_static::<RoomNameEventContent>().await?;
            let room_topic_state_events = room.get_state_events_static::<RoomTopicEventContent>().await?;

            let title = deserialize_sync_state_events_to_content(room_name_state_events)?.and_then(|e| e.name);
            let topic = deserialize_sync_state_events_to_content(room_topic_state_events)?.map(|e| e.topic);
            let category = deserialize_sync_state_events_to_content(category_state_events)?.and_then(|e| e.category);

            let aliases = room.canonical_alias().into_iter().chain(room.alt_aliases().into_iter());

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

    pub async fn set_category(
        &self,
        room_alias_id: &str,
        category: Option<String>,
    ) -> Result<(), Error> {
        let room_id = self.forum_user.resolve_room_alias(room_alias_id.try_into()?).await?.room_id;

        let content = MorumCategoryEventContent { category };

        let room = self.forum_user.get_joined_room(&room_id).ok_or(Error::UnknownPost)?;
        room.send_state_event(content).await?;

        Ok(())
    }

    pub async fn messages(&self, room_alias_id: &str) -> Result<Vec<Message>, Error> {
        use ruma::events::room::message::{
            sanitize::{HtmlSanitizerMode, RemoveReplyFallback},
            MessageFormat, MessageType,
        };
        use ruma::events::{AnyMessageLikeEvent, AnyTimelineEvent, MessageLikeEvent};

        let room_id = self.forum_user.resolve_room_alias(room_alias_id.try_into()?).await?.room_id;
        let room = self.forum_user.get_joined_room(&room_id).ok_or(Error::UnknownPost)?;

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

    pub async fn send_message(
        &self,
        localpart: &str,
        room_alias_id: &str,
        markdown: &str,
    ) -> Result<(), Error> {
        use ruma::events::room::message::{
            sanitize::{HtmlSanitizerMode, RemoveReplyFallback},
            FormattedBody, MessageType, RoomMessageEventContent, TextMessageEventContent,
        };
        use ruma::TransactionId;

        let room_id = self.forum_user.resolve_room_alias(room_alias_id.try_into()?).await?.room_id;

        self.ensure_registered(localpart).await?;

        let user_id = ruma::UserId::parse(&format!("@{}:corepaper.org", localpart))?;

        let request = ruma::api::client::membership::join_room_by_id::v3::Request::new(&room_id);
        let _response = self.client.send_request_as(&user_id, request).await?;

        let mut html_body = String::new();

        pulldown_cmark::html::push_html(&mut html_body, pulldown_cmark::Parser::new(markdown));
        let mut formatted = FormattedBody::html(html_body);
        formatted.sanitize_html(HtmlSanitizerMode::Strict, RemoveReplyFallback::Yes);

        let mut event_content = TextMessageEventContent::plain(markdown);
        event_content.formatted = Some(formatted);

        let message = RoomMessageEventContent::new(MessageType::Text(event_content));

        let transaction_id = TransactionId::new();
        let request = ruma::api::client::message::send_message_event::v3::Request::new(
            &room_id,
            &transaction_id,
            &message,
        )?;
        let _response = self.client.send_request_as(&user_id, request).await?;

        self.forum_user.sync_once(SyncSettings::default().full_state(true)).await?;

        Ok(())
    }

    pub async fn create_room(
        &self,
        room_alias_localpart: &str,
        name: &str,
        topic: &str,
    ) -> Result<(), Error> {
        use ruma::api::client::room::create_room::v3::RoomPreset;

        let mut request = ruma::api::client::room::create_room::v3::Request::new();
        request.room_alias_name = Some(room_alias_localpart);
        request.name = Some(name);
        request.topic = Some(topic);
        request.preset = Some(RoomPreset::PublicChat);
        request.is_direct = false;

        self.forum_user.create_room(request).await?;
        Ok(())
    }
}

pub async fn start(config: Config) -> Result<AppService, Error> {
    let appservice = AppService::new(config.homeserver_url, config.homeserver_access_token).await?;

    appservice.ensure_registered("forum").await?;

    Ok(appservice)
}
