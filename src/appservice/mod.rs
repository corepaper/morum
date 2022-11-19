mod client;

use tracing::{debug, trace};
use crate::{Config, Error};
use regex::Regex;
use serde::{Serialize, Deserialize};
use ruma::events::{EmptyStateKey, AnyStateEvent, StateEvent};
use ruma_macros::EventContent;

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

#[derive(Debug)]
pub struct AppService(self::client::Client);

impl AppService {
    pub async fn new(homeserver_url: String, access_token: String) -> Result<Self, Error> {
        Ok(Self(
            self::client::Client::new(
                homeserver_url,
                access_token,
            ).await?
        ))
    }

    pub async fn ensure_registered(&self, localpart: &str) -> Result<(), Error> {
        use ruma::api::client::account::register::{
            RegistrationKind, LoginType,
            v3::Request
        };

        let mut request = Request::new();
        request.username = Some(localpart);
        request.device_id = Some("morum".try_into()?);
        request.kind = RegistrationKind::User;
        request.inhibit_login = true;
        request.login_type = Some(&LoginType::ApplicationService);
        request.refresh_token = false;

        let response = self.0.send_request_force_auth(request).await;

        match response {
            Err(ruma::client::Error::FromHttpResponse(
                ruma::api::error::FromHttpResponseError::Server(
                    ruma::api::error::ServerError::Known(
                        ruma::api::client::uiaa::UiaaResponse::MatrixError(
                            ruma::api::client::Error { kind, .. }
                        )
                    )
                )
            )) if kind == ruma::api::client::error::ErrorKind::UserInUse => Ok(()),
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }

    pub async fn valid_rooms(&self) -> Result<Vec<Room>, Error> {
        let mut rooms = Vec::new();

        let request = ruma::api::client::membership::joined_rooms::v3::Request::new();
        let response = self.0.send_request_as("@forum:corepaper.org".try_into()?, request).await?;

        for room_id in response.joined_rooms {
            debug!("room id: {:?}", room_id);

            let mut title = None;
            let mut topic = None;
            let mut category = None;
            let mut post_id = None;

            let request = ruma::api::client::state::get_state_events::v3::Request::new(&room_id);
            let response = self.0.send_request_as("@forum:corepaper.org".try_into()?, request).await?;

            for raw_event in response.room_state {
                match raw_event.get_field("type")? {
                    Some("org.corepaper.morum.category") => {
                        let event = raw_event.deserialize_as::<StateEvent<MorumCategoryEventContent>>()?;

                        if let StateEvent::Original(event) = event {
                            category = event.content.category;
                        }
                    },
                    _ => {
                        let event = raw_event.deserialize_as::<AnyStateEvent>()?;

                        match event {
                            AnyStateEvent::RoomName(StateEvent::Original(event)) => {
                                title = event.content.name;
                            },
                            AnyStateEvent::RoomTopic(StateEvent::Original(event)) => {
                                topic = Some(event.content.topic);
                            },
                            _ => (),
                        }
                    },
                }
            }

            let request = ruma::api::client::room::aliases::v3::Request::new(&room_id);
            let response = self.0.send_request_as("@forum:corepaper.org".try_into()?, request).await?;

            for alias in response.aliases {
                let re = Regex::new(r"^#forum_post_(\d+):corepaper\.org$").expect("regex is valid");
                let captures = re.captures(alias.as_str());

                if let Some(captures) = captures {
                    post_id = captures.get(1).and_then(|s| s.as_str().parse::<usize>().ok());
                }
            }

            debug!("title: {:?}, topic: {:?}, category: {:?}, post id: {:?}", title, topic, category, post_id);

            if let (Some(title), Some(post_id)) = (title, post_id) {
                rooms.push(Room {
                    title, category, post_id, topic,
                    room_id: room_id.as_str().to_owned(),
                })
            }
        }

        Ok(rooms)
    }

    pub async fn set_category(&self, room_id: &str, category: Option<String>) -> Result<(), Error> {
        let content = MorumCategoryEventContent {
            category,
        };

        let mut request = ruma::api::client::state::send_state_event::v3::Request::new(
            room_id.try_into()?,
            &EmptyStateKey,
            &content,
        )?;

        let response = self.0.send_request_as("@forum:corepaper.org".try_into()?, request).await?;
        Ok(())
    }
}

pub async fn start(config: Config) -> Result<AppService, Error> {
    let appservice = AppService::new(config.homeserver_url, config.homeserver_access_token).await?;

    appservice.ensure_registered("forum").await?;

    Ok(appservice)
}
