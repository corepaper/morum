use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Subcategory {
    pub id: Option<String>,
    pub title: String,
    pub topic: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Category {
    pub title: String,
    pub topic: String,
    pub subcategories: Vec<Subcategory>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Post {
    pub title: String,
    pub topic: Option<String>,
    pub id: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Comment {
    pub html: String,
    pub sender: String,
}
