use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Category {
    pub title: String,
    pub topic: String,
    pub room_local_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Post {
    pub title: String,
    pub topic: Option<String>,
    pub room_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Comment {
    pub html: String,
    pub sender: String,
}
