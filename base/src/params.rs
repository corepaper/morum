use serde::{Deserialize, Serialize};
use crate::types;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Categories { }

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CategoriesResponse {
    pub categories: Vec<types::Category>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Posts {
    pub category_id: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PostsResponse {
    pub category: types::Category,
    pub subcategory: types::Subcategory,
    pub posts: Vec<types::Post>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Post {
    pub id: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PostResponse {
    pub post: types::Post,
    pub comments: Vec<types::Comment>,
}
