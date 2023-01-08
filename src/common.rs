use serde::{Serialize, Deserialize};

// Structs

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

// Functions

fn md_to_html(str: String) -> String {
    let mut res = String::new();
    let mut i = 0;
    while i < str.len() {
        if str[i..].starts_with("```") && str.as_bytes()[i - 1] != b'\\' {
            res.push_str("<pre><code>");
            i += 3;
            while i < str.len() && !str[i..].starts_with("```") {
                res.push(str.as_bytes()[i] as char);
                i += 1;
            }
            res.push_str("</code></pre>");
            i += 3;
        } else {
            let start = i;
            while i < str.len() && !str[i..].starts_with("```") {
                i += 1;
            }
            res.push_str(&markdown::to_html(&str[start..i]));
            i += 1;
        }
    }
    res
}

// Macros

#[macro_export]
macro_rules! header_footer {
    ($ctx:ident, $title:expr, $tera:ident) => {{
        let mut ctx = Context::new();
        ctx.insert("title", $title);
        $ctx.insert("header", $tera.render("header.html", &ctx).unwrap().as_str());
        $ctx.insert("footer", $tera.render("footer.html", &ctx).unwrap().as_str());
    }};
}