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
