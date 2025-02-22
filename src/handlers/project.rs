use crate::error::internal_server_error;
use crate::handlers::api_response::ApiResponse;
use crate::models::Project;
use crate::models::UpdateProject;
use crate::routes::router::AppState;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProject>,
) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return internal_server_error::<ProjectResponse>("Database connection failed", &err);
        }
    };

    if payload.name.is_none() || payload.name.as_ref().unwrap().trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<ProjectResponse>::Error {
                error: "Project name is required.".to_string(),
            }),
        );
    }

    let name = payload.name.unwrap();
    let description = payload.description.unwrap_or_default();

    match Project::create(&mut conn, &name, &description) {
        Ok(project) => {
            let response = ApiResponse::Success(ProjectResponse {
                id: project.id,
                name: project.name,
                description: project.description,
            });
            (StatusCode::CREATED, Json(response))
        }
        Err(err) => {
            return internal_server_error::<ProjectResponse>("Failed to create project", &err);
        }
    }
}


pub async fn get_projects(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return internal_server_error::<Vec<ProjectResponse>>(
                "Database connection failed",
                &err,
            )
        }
    };

    match Project::get_first_n(&mut conn, 10) {
        Ok(projects) => {
            let response: Vec<ProjectResponse> = projects
                .into_iter()
                .map(|p| ProjectResponse {
                    id: p.id,
                    name: p.name,
                    description: p.description,
                })
                .collect();
            (StatusCode::OK, Json(ApiResponse::Success(response)))
        }
        Err(err) => {
            return internal_server_error::<Vec<ProjectResponse>>(
                "Failed to retrieve project list",
                &err,
            )
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    success: bool,
    message: String,
}

pub async fn delete_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return internal_server_error::<DeleteResponse>("Database connection failed", &err)
        }
    };

    match Project::find_by_id(&mut conn, id) {
        Ok(Some(_)) => match Project::delete_by_id(&mut conn, id) {
            Ok(_) => {
                tracing::info!("Project with ID {} deleted successfully", id);
                let response = ApiResponse::Success(DeleteResponse {
                    success: true,
                    message: format!("Project with ID {} successfully deleted", id),
                });
                (StatusCode::OK, Json(response))
            }
            Err(err) => {
                return internal_server_error::<DeleteResponse>("Failed to delete project", &err)
            }
        },
        Ok(None) => {
            tracing::warn!("Attempt to delete non-existent project: {}", id);
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::Error {
                    error: format!("Project with ID {} not found", id),
                }),
            )
        }
        Err(err) => return internal_server_error::<DeleteResponse>("Database error", &err),
    }
}

pub async fn update_project(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<i32>,
    Json(payload): Json<UpdateProject>,
) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<ProjectResponse>::Error {
                    error: format!("DB pool error: {err}"),
                }),
            );
        }
    };

    match Project::find_by_id(&mut conn, project_id) {
        Ok(Some(_)) => {
            if payload.name.is_none() && payload.description.is_none() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<ProjectResponse>::Error {
                        error: "At least one field must be provided for update.".to_string(),
                    }),
                );
            }

            match Project::update(&mut conn, project_id, &payload) {
                Ok(updated_project) => {
                    let response = ApiResponse::Success(ProjectResponse {
                        id: updated_project.id,
                        name: updated_project.name,
                        description: updated_project.description,
                    });
                    (StatusCode::OK, Json(response))
                }
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<ProjectResponse>::Error {
                        error: format!("Failed to update project: {err}"),
                    }),
                ),
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<ProjectResponse>::Error {
                error: format!("Project with ID {} not found", project_id),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<ProjectResponse>::Error {
                error: format!("Database error: {err}"),
            }),
        ),
    }
}
