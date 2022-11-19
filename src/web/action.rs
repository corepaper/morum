use super::Context;
use crate::error::Error;
use async_trait::async_trait;
use morum_base::{params::*, types};
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
        let valid = {
            let mut found = false;

            for user in &context.config.closed_beta_users {
                if self.username == user.username && self.password == user.password {
                    found = true;
                    break
                }
            }

            found
        };

        if valid {
            let claim = AccessClaim {
                username: self.username.clone(),
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claim,
                &jsonwebtoken::EncodingKey::from_secret(context.config.jwt_secret.as_bytes()),
            )?;

            Ok(LoginResponse { access_token: token })
        } else {
            Err(Error::InvalidLoginCredential)
        }
    }
}

#[async_trait]
impl Perform for Categories {
    type Response = CategoriesResponse;

    async fn perform(&self, context: &Arc<Context>) -> Result<CategoriesResponse, Error> {
        Ok(CategoriesResponse {
            categories: context.config.categories.clone()
        })
    }
}

#[async_trait]
impl Perform for Posts {
    type Response = PostsResponse;

    async fn perform(&self, context: &Arc<Context>) -> Result<PostsResponse, Error> {
        let rooms = context.appservice.valid_rooms().await?;
        let mut posts = Vec::new();

        for room in rooms {
            if room.category == self.category_id {
                posts.push(types::Post {
                    title: room.title,
                    topic: room.topic,
                    id: room.post_id,
                });
            }
        }

        Ok(PostsResponse { posts })
    }
}
