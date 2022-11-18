use super::Context;
use crate::error::Error;
use async_trait::async_trait;
use morum_base::params::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait Perform {
    type Response: Serialize + Send;

    async fn perform(&self, context: &Arc<Context>) -> Result<Self::Response, Error>;
}

#[derive(Serialize, Deserialize)]
pub struct AccessClaim {
    pub username: String,
}

#[async_trait]
impl Perform for Login {
    type Response = LoginResponse;

    async fn perform(&self, context: &Arc<Context>) -> Result<LoginResponse, Error> {
        if self.username == "dev" && self.password == "dev" {
            let claim = AccessClaim {
                username: self.username.clone(),
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claim,
                &jsonwebtoken::EncodingKey::from_secret(context.config.jwt_secret.as_bytes()),
            )?;

            Ok(LoginResponse { jwt: token })
        } else {
            Err(Error::InvalidLoginCredential)
        }
    }
}
