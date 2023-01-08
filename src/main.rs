mod blogs;
mod login;
mod projects;
mod static_pages;
mod common;

use blogs::*;
use login::*;
use projects::*;

use axum::{
    extract::Extension,
    http::uri::Uri,
    response::IntoResponse,
    routing::get,
    Router
};
use sqlx::PgPool;
use tera::Tera;
use tower_cookies::{Cookie, CookieManagerLayer};

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:1597@localhost:3000/postgres".to_string());
    let pool = PgPool::connect(&db_url).await.unwrap();
    let tera = Tera::new("assets/*.html").unwrap();

    let app = Router::new()
        // Home section
        .route("/", get(static_pages::index))
        .route("/about", get(static_pages::about))
        .route("/search/:id", get(static_pages::search))

        // Login section
        .route("/login",    get(login_page).post(login_into_account))
        .route("/register", get(register_page).post(register))
        .route("/logout",   get(logout))

        // Projects section
        .route("/projects",     get(all_projects_list))
        .route("/project/new",  get(project_new).post(project_create))
        .route("/project/:id",  get(project_get).post(project_delete).patch(project_update))

        // Blogs section
        .route("/blogs",    get(all_blogs_list))
        .route("/blog/new", get(blog_new).post(blog_create))
        .route("/blog/:id", get(blog_get).post(blog_delete).patch(blog_update))

        // Extensions section
        .layer(Extension(pool))
        .layer(Extension(tera))
        .layer(CookieManagerLayer::new())

        .fallback(static_pages::handler_404);

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}