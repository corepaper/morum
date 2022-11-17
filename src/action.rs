use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use async_trait::async_trait;
use crate::error::Error;

#[derive(Default)]
pub struct Database {
    pub users: HashMap<String, User>,
}

pub struct User {
    pub password: String,
}

pub struct Config {
    pub jwt_secret: jsonwebtoken::EncodingKey,
}

pub struct Context {
    pub database: RwLock<Database>,
    pub config: Config,
}

impl Context {
    pub fn dev() -> Self {
        Context {
            database: RwLock::new(Default::default()),
            config: Config {
                jwt_secret: jsonwebtoken::EncodingKey::from_secret(b"devsecret"),
            }
        }
    }
}

#[async_trait]
pub trait Perform {
    type Response: Serialize + Send;

    async fn perform(
        &self,
        context: &Context,
    ) -> Result<Self::Response, Error>;
}

#[derive(Serialize)]
pub struct AccessClaim {
    pub username: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub jwt: String,
}

#[async_trait]
impl Perform for Login {
    type Response = LoginResponse;

    async fn perform(
        &self,
        context: &Context,
    ) -> Result<LoginResponse, Error> {
        if self.username == "dev" && self.password == "dev" {
            let claim = AccessClaim {
                username: self.username.clone(),
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claim,
                &context.config.jwt_secret,
            )?;

            Ok(LoginResponse {
                jwt: token,
            })
        } else {
            Err(Error::InvalidLoginCredential)
        }
    }
}
