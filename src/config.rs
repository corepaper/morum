use serde::{Deserialize, Serialize};
use morum_base::types::Category;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ClosedBetaUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Config {
    pub jwt_secret: String,
    pub closed_beta_users: Vec<ClosedBetaUser>,
    pub homeserver_url: String,
    pub homeserver_name: String,
    pub homeserver_access_token: String,
    pub categories: Vec<Category>,
}
