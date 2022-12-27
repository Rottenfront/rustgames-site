use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub date: String,
    pub blog: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Blog {
    pub id: usize,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub favourite_food: Option<String>,
}