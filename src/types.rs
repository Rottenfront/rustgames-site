use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub date: String,
    pub blog: Option<Blog>,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Blog {
    pub id: u32,
    pub author_id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BlogList {
    pub blog: Blog,
    pub messages: Vec<BlogMsg>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BlogMsg {
    pub id: u32,
    pub name: String,
    pub text: String,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Comment {
    pub id: u32,
    pub author_id: u32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub avatar: String,
}
