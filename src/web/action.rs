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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessClaim {
    pub username: String,
    pub exp: u64,
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
                    break;
                }
            }

            found
        };

        if valid {
            use std::time::{SystemTime, UNIX_EPOCH};

            let claim = AccessClaim {
                username: self.username.clone(),
                exp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 604800,
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claim,
                &jsonwebtoken::EncodingKey::from_secret(context.config.jwt_secret.as_bytes()),
            )?;

            Ok(LoginResponse {
                access_token: token,
            })
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
            categories: context.config.categories.clone(),
        })
    }
}

#[async_trait]
impl Perform for Posts {
    type Response = PostsResponse;

    async fn perform(&self, context: &Arc<Context>) -> Result<PostsResponse, Error> {
        let mut category = None;
        let mut subcategory = None;

        for c in &context.config.categories {
            for sc in &c.subcategories {
                if sc.id == self.category_id {
                    category = Some(c.clone());
                    subcategory = Some(sc.clone());
                }
            }
        }

        let category = category.ok_or(Error::UnknownCategory)?;
        let subcategory = subcategory.ok_or(Error::UnknownCategory)?;

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

        Ok(PostsResponse {
            posts,
            category,
            subcategory,
        })
    }
}

#[async_trait]
impl Perform for Post {
    type Response = PostResponse;

    async fn perform(&self, context: &Arc<Context>) -> Result<PostResponse, Error> {
        let rooms = context.appservice.valid_rooms().await?;
        let mut post = None;
        for room in rooms {
            if room.post_id == self.id {
                post = Some(types::Post {
                    title: room.title,
                    topic: room.topic,
                    id: room.post_id,
                });
            }
        }
        let post = post.ok_or(Error::UnknownPost)?;

        let messages = context
            .appservice
            .messages(&format!("#forum_post_{}:corepaper.org", post.id))
            .await?;
        let mut comments = Vec::new();
        for message in messages {
            comments.push(types::Comment {
                html: message.html,
                sender: message.sender,
            });
        }

        Ok(PostResponse { comments, post })
    }
}

#[async_trait]
impl Perform for NewComment {
    type Response = NewCommentResponse;

    async fn perform(&self, context: &Arc<Context>) -> Result<NewCommentResponse, Error> {
        if &self.markdown == "" {
            return Err(Error::BlankContent);
        }

        let claim = jsonwebtoken::decode::<AccessClaim>(
            &self.access_token,
            &jsonwebtoken::DecodingKey::from_secret(context.config.jwt_secret.as_bytes()),
            &Default::default(),
        )?;

        let localpart = format!("forum_user_{}", claim.claims.username);

        let rooms = context.appservice.valid_rooms().await?;
        let mut post = None;
        for room in rooms {
            if room.post_id == self.post_id {
                post = Some(types::Post {
                    title: room.title,
                    topic: room.topic,
                    id: room.post_id,
                });
            }
        }
        let post = post.ok_or(Error::UnknownPost)?;

        let room_alias = format!("#forum_post_{}:corepaper.org", post.id);

        context
            .appservice
            .send_message(&localpart, &room_alias, &self.markdown)
            .await?;

        Ok(NewCommentResponse {})
    }
}

#[async_trait]
impl Perform for NewPost {
    type Response = NewPostResponse;

    async fn perform(&self, context: &Arc<Context>) -> Result<NewPostResponse, Error> {
        if &self.title == "" {
            return Err(Error::BlankTitle);
        }

        if &self.topic == "" {
            return Err(Error::BlankTopic);
        }

        if &self.markdown == "" {
            return Err(Error::BlankContent);
        }

        let claim = jsonwebtoken::decode::<AccessClaim>(
            &self.access_token,
            &jsonwebtoken::DecodingKey::from_secret(context.config.jwt_secret.as_bytes()),
            &Default::default(),
        )?;

        let localpart = format!("forum_user_{}", claim.claims.username);

        let rooms = context.appservice.valid_rooms().await?;
        let room_id = rooms
            .iter()
            .map(|r| r.post_id)
            .reduce(|acc, r| if acc >= r { acc } else { r })
            .unwrap_or(1)
            + 1;

        let room_alias_localpart = format!("forum_post_{}", room_id);
        context
            .appservice
            .create_room(&room_alias_localpart, &self.title, &self.topic)
            .await?;

        let room_alias = format!("#forum_post_{}:corepaper.org", room_id);
        context
            .appservice
            .set_category(&room_alias, self.category_id.clone())
            .await?;

        context
            .appservice
            .send_message(&localpart, &room_alias, &self.markdown)
            .await?;

        Ok(NewPostResponse { post_id: room_id })
    }
}
