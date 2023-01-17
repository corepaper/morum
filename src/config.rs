use morum_base::types::Category;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Config {
    pub homeserver_url: String,
    pub username: String,
    pub password: String,
    pub categories: Vec<Category>,
}
