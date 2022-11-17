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

#[derive(Clone)]
pub struct Context {
    pub database: Arc<RwLock<Database>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            database: Arc::new(RwLock::new(Default::default())),
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
        unimplemented!()
    }
}
