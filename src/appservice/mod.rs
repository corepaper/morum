mod client;

use crate::{Config, Error};

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
}

pub async fn start(config: Config) -> Result<AppService, Error> {
    let appservice = AppService::new(config.homeserver_url, config.homeserver_access_token).await?;

    appservice.ensure_registered("forum").await?;

    Ok(appservice)
}
