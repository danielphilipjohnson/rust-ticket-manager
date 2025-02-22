use axum::{
    routing::{delete, get, post, put},
    Router,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;

use crate::handlers::{
    issue::{create_issue, delete_issue, get_project_issues, update_issue},
    project::{create_project, delete_project, get_projects, update_project},
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub pool: DbPool,
}

pub fn create_router(pool: DbPool) -> Router {
    let state = Arc::new(AppState { pool });
    Router::new()
        // Project routes
        .route("/api/projects", post(create_project))
        .route("/api/projects", get(get_projects))
        .route("/api/projects/{project_id}", delete(delete_project))
        .route("/api/projects/{project_id}", put(update_project))
        // Issue routes
        .route("/api/projects/{project_name}/issues", post(create_issue))
        .route(
            "/api/projects/{project_name}/issues",
            get(get_project_issues),
        )
        .route(
            "/api/projects/{project_name}/issues/{issue_id}",
            put(update_issue),
        )
        .route(
            "/api/projects/{project_name}/issues/{issue_id}",
            delete(delete_issue),
        )
        .with_state(state)
}
