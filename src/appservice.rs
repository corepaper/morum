use crate::{Config, Error};
use matrix_sdk::{
    config::SyncSettings,
    ruma::{self, api::client::Direction},
    room::MessagesOptions,
};
use matrix_sdk_appservice::{
    AppService,
    AppServiceRegistration,
};

pub struct AppServiceHandle(pub AppService);

impl AppServiceHandle {
    pub async fn ensure_registered(&self, localpart: &str) -> Result<(), Error> {
        let res = self.0.register_user(localpart, Some("morum".try_into()?)).await;

        match res {
            Err(matrix_sdk_appservice::Error::Matrix(err)) if err.client_api_error_kind() == Some(&matrix_sdk::ruma::api::client::error::ErrorKind::UserInUse) => Ok(()),
            res => res.map_err(Into::into),
        }
    }
}

pub async fn start(config: Config, registration_file_path: String) -> Result<(), Error> {
    let registration = AppServiceRegistration::try_from_yaml_file(&registration_file_path)?;

    let appservice =
        AppService::builder(
            config.homeserver_url.as_str().try_into()?,
            config.homeserver_name.try_into()?,
            registration,
        )
        .build()
        .await?;

    let handle = AppServiceHandle(appservice);

    // handle.ensure_registered("forum").await?;

    let client = handle.0.user(Some("forum")).await?;
    client.sync_once(SyncSettings::default()).await?;

    Ok(())
}
