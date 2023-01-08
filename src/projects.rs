use crate::{IntoResponse, PgPool};
use axum::extract::{Extension, Form, Path};
use axum::response::{Html, Redirect};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tower_cookies::Cookies;

#[derive(Serialize)]
pub struct Todo {
    id: i32,
    title: String,
    description: String,
    checked: bool,
    login: String,
}

pub async fn get_description(
        Path(id): Path<u32>,
        Extension(pool): Extension<PgPool>,
        Extension(tera): Extension<Tera>,
        ) -> Html<String> {
    let todo = sqlx::query!("SELECT * FROM todos WHERE id = $1", id as i32)
        .fetch_one(&pool)
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("title", &todo.title);
    context.insert("description", &todo.description);
    context.insert("id", &todo.id);
    context.insert(
            "checked",
        &if todo.checked { "Done" } else { "Not yet Done" },
    );

    Html(tera.render("project_description.html", &context).unwrap())
}

pub async fn delete_todo(
        Path(id): Path<u32>,
        Extension(pool): Extension<PgPool>,
        ) -> impl IntoResponse {
    sqlx::query!("DELETE FROM todos WHERE id = $1", id as i32)
        .execute(&pool)
        .await
        .unwrap();

    Redirect::to("/")
}

#[derive(Deserialize)]
pub struct NewTodo {
    title: String,
    description: String,
}

pub async fn editing_new_todo<'a>() -> Html<&'a str> {
    Html(include_str!("../templates/project_new.html"))
}

pub async fn create_todo(
        Form(todo): Form<NewTodo>,
        cookies: Cookies,
        Extension(pool): Extension<PgPool>,
        ) -> impl IntoResponse {
    sqlx::query!(
            "INSERT INTO todos(title, description, login) VALUES($1, $2, $3)",
        todo.title,
        todo.description,
        cookies.get("login").unwrap().value().to_string()
    )
    .execute(&pool)
    .await
    .unwrap();

    Redirect::to("/")
}

#[derive(Deserialize)]
pub struct UpdatedTodo {
    title: String,
    description: String,
    checked: Option<String>,
}

pub async fn edit_todo(
        Path(id): Path<u32>,
        Extension(pool): Extension<PgPool>,
        Extension(tera): Extension<Tera>,
        ) -> Html<String> {
    let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id as i32)
        .fetch_one(&pool)
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("todo", &todo);

    Html(tera.render("projects_edit.html", &context).unwrap())
}

pub async fn update_todo(
        Path(id): Path<u32>,
        Form(new_content): Form<UpdatedTodo>,
        Extension(pool): Extension<PgPool>,
        ) -> impl IntoResponse {
    sqlx::query!(
            "UPDATE todos SET title = $1, description = $2, checked = $3 WHERE id = $4",
        new_content.title,
        new_content.description,
        new_content.checked.is_some(),
        id as i32,
    )
    .execute(&pool)
    .await
    .unwrap();

    Redirect::to(&format!("/blog/{id}"))
}


pub async fn all_projects_list(Extension(pool): Extension<PgPool>,
                               Extension(tera): Extension<Tera>) -> Html<String> {
    let projects = sqlx::query_as!(Project, "SELECT * FROM projects")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("projects", &projects);
    let mut ctx = Context::new();
    ctx.insert("body", &tera.render("projects_list.html", &context).unwrap());
    Html(tera.render("static.html", &ctx).unwrap())
}
pub async fn project_new(Extension(pool): Extension<PgPool>, Extension(tera): Extension<Tera>, cookies: Cookies) -> Html<String> {

}
pub async fn project_create(Extension(pool): Extension<PgPool>, Extension(tera): Extension<Tera>, cookies: Cookies) -> Html<String> {

}
pub async fn project_edit(Extension(pool): Extension<PgPool>, Extension(tera): Extension<Tera>, cookies: Cookies) -> Html<String> {

}
pub async fn project_update(Extension(pool): Extension<PgPool>, Extension(tera): Extension<Tera>, cookies: Cookies) -> Html<String> {

}
pub async fn project_get(Extension(pool): Extension<PgPool>, Extension(tera): Extension<Tera>, cookies: Cookies) -> Html<String> {

}
pub async fn project_delete(Extension(pool): Extension<PgPool>, Extension(tera): Extension<Tera>, cookies: Cookies) -> Html<String> {

}