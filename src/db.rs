use crate::types::*;
use axum::Json;
use uuid::Uuid;

pub fn get_all_blogs(offset: Option<usize>, limit: Option<usize>) -> Vec<Blog> {
    vec![]
}

pub fn get_all_projects(offset: Option<usize>, limit: Option<usize>) -> Vec<Project> {
    Vec::new()
}

pub fn create_blog(blog: Blog) -> Uuid {
    Uuid::default()
}

pub fn create_project(project: Project) -> Uuid {
    Uuid::default()
}

pub fn update_blog(id: Uuid, blog: Blog) -> anyhow::Result<Json<Blog>> {
    Ok(Json(blog))
}

pub fn update_project(id: Uuid, proj: Project) -> anyhow::Result<Json<Project>> {
    Ok(Json(proj))
}

pub fn delete_blog(id: Uuid) -> anyhow::Result<()> {
    Ok(())
}

pub fn delete_project(id: Uuid) -> anyhow::Result<()> {
    Ok(())
}

pub fn get_blog(id: Uuid) -> BlogList {
    BlogList::default()
}

pub fn get_project(id: Uuid) -> Project {
    Project::default()
}
