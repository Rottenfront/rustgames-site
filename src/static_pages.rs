use crate::{Cookie, header_footer, IntoResponse, PgPool, Uri};
use axum::{
    extract::{Extension, Form, Path},
    response::{Html, Redirect},
};
use serde::Deserialize;
use tera::{Context, Tera};
use tower_cookies::Cookies;
use std::string::String;

pub async fn index(Extension(tera): Extension<Tera>) -> Html<String> {
    let mut context = Context::new();
    context.insert("body", &tera.render("index.html", &Context::new()).unwrap());

    Html(tera.render("static.html", &context).unwrap())
}

pub async fn about(Extension(tera): Extension<Tera>) -> Html<String> {
    let mut context = Context::new();
    context.insert("body", &tera.render("about.html", &Context::new()).unwrap());

    Html(tera.render("static.html", &context).unwrap())
}

pub async fn handler_404(Extension(tera): Extension<Tera>) -> Html<String> {
    let mut context = Context::new();
    context.insert("body", &tera.render("404.html", &Context::new()).unwrap());

    Html(tera.render("static.html", &context).unwrap())
}

pub async fn search(Path(path): Path<String>, Extension(tera): Extension<Tera>) -> Html<String> {
    let mut context = Context::new();
    context.insert("body", &tera.render("search.html", &Context::new()).unwrap());

    Html(tera.render("static.html", &context).unwrap())
}
