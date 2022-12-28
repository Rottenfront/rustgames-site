//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PUT /todos/:id`: update a specific Todo.
//! - `DELETE /todos/:id`: delete a specific Todo.

mod db;
mod types;

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use serde::Deserialize;
use std::{
    net::SocketAddr,
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use db::*;
use types::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Compose the routes
    let app = Router::new()
        .route("/projects", get(all_projects_get).post(project_create))
        .route("/projects/:id", get(project_get).patch(project_update).delete(project_delete))
        .route("/blogs", get(all_blogs_get).post(blog_create))
        .route("/blogs/:id", get(blog_get).patch(blog_update).delete(blog_delete))
        // Add middleware to all routes
        .layer(
                ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .route("/projects", get(all_projects_get).post(project_create))
        .route("/projects/:id", patch(project_update).delete(project_delete));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// The query parameters
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn all_projects_get(pagination: Option<Query<Pagination>>) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    Json(get_all_projects(pagination.offset, pagination.limit))
}

async fn project_create(Json(input): Json<Project>) -> impl IntoResponse {
    create_project(input.clone());
    (StatusCode::CREATED, Json(input))
}

async fn project_get(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(get_project(id))
}

async fn project_update(Path(id): Path<Uuid>, Json(input): Json<Project>) -> Result<impl IntoResponse, StatusCode> {
    // TODO: do normal anyhow::Result returning
    update_project(id, input.clone());
    Ok(Json(input))
}

async fn project_delete(Path(id): Path<Uuid>) -> impl IntoResponse {
    if let Result::Err(_) = delete_project(id) {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::NO_CONTENT
    }
}


async fn all_blogs_get(pagination: Option<Query<Pagination>>) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    Json(get_all_blogs(pagination.offset, pagination.limit))
}

async fn blog_create(Json(input): Json<Blog>) -> impl IntoResponse {
    create_blog(input.clone());
    (StatusCode::CREATED, Json(input))
}

async fn blog_get(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(get_blog(id))
}

async fn blog_update(Path(id): Path<Uuid>, Json(input): Json<Blog>) -> Result<impl IntoResponse, StatusCode> {
    // TODO: do normal anyhow::Result returning
    update_blog(id, input.clone());
    Ok(Json(input))
}

async fn blog_delete(Path(id): Path<Uuid>) -> impl IntoResponse {
    if let Result::Err(_) = delete_blog(id) {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::NO_CONTENT
    }
}
