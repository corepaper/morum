use serde::{Deserialize, Serialize};

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
}
