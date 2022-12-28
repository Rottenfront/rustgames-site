use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub date: String,
    pub blog: Option<String>,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Blog {
    pub id: usize,
    pub author_id: usize,
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
    pub id: usize,
    pub name: String,
    pub text: String,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Comment {
    pub id: usize,
    pub author_id: usize,
    pub text: String,
}
