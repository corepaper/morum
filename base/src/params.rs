use serde::{Deserialize, Serialize};
use crate::types::Category;

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
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Subcategory {
    pub id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SubcategoryResponse {

}
