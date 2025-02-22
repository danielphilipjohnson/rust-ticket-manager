use crate::handlers::api_response::ApiResponse;
use crate::models::{Issue, IssueChanges, Project};
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
pub struct CreateIssueRequest {
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub assigned_to: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IssueResponse {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub description: String,
    pub created_by: String,
    pub assigned_to: Option<String>,
    pub status: String,
    pub is_open: bool,
}

const VALID_STATUSES: &[&str] = &["open", "in_progress", "resolved", "closed"];

pub async fn get_project_issues(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<Vec<IssueResponse>>::Error {
                    error: format!("DB pool error: {err}"),
                }),
            )
        }
    };

    // First check if project exists
    match Project::find_by_id(&mut conn, project_id) {
        Ok(Some(_)) => {
            // Project exists, get its issues
            match Issue::get_by_project(&mut conn, project_id) {
                Ok(issues) => {
                    let response: Vec<IssueResponse> = issues
                        .into_iter()
                        .map(|issue| IssueResponse {
                            id: issue.id,
                            project_id: issue.project_id,
                            title: issue.title,
                            description: issue.description,
                            created_by: issue.created_by,
                            assigned_to: issue.assigned_to,
                            status: issue.status,
                            is_open: issue.is_open,
                        })
                        .collect();
                    (StatusCode::OK, Json(ApiResponse::Success(response)))
                }
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<Vec<IssueResponse>>::Error {
                        error: format!("Failed to fetch issues: {err}"),
                    }),
                ),
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Vec<IssueResponse>>::Error {
                error: format!("Project with ID {} not found", project_id),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<IssueResponse>>::Error {
                error: format!("Database error: {err}"),
            }),
        ),
    }
}

pub async fn create_issue(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<i32>,
    Json(payload): Json<CreateIssueRequest>,
) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<IssueResponse>::Error {
                    error: format!("DB pool error: {err}"),
                }),
            )
        }
    };

    let status = payload
        .status
        .as_deref()
        .unwrap_or("open")
        .to_lowercase();

    if !VALID_STATUSES.contains(&status.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<IssueResponse>::Error {
                error: format!("Invalid status value. Allowed values: {:?}", VALID_STATUSES),
            }),
        );
    }

    match Project::find_by_id(&mut conn, project_id) {
        Ok(Some(_)) => {
            match Issue::create(
                &mut conn,
                project_id,
                &payload.title,
                &payload.description,
                &payload.created_by,
                payload.assigned_to.as_deref(),
                &status,
            ) {
                Ok(issue) => {
                    let response = ApiResponse::Success(IssueResponse {
                        id: issue.id,
                        project_id: issue.project_id,
                        title: issue.title,
                        description: issue.description,
                        created_by: issue.created_by,
                        assigned_to: issue.assigned_to,
                        status: issue.status,
                        is_open: issue.is_open,
                    });
                    (StatusCode::CREATED, Json(response))
                }
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<IssueResponse>::Error {
                        error: format!("Failed to create issue: {err}"),
                    }),
                ),
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<IssueResponse>::Error {
                error: format!("Project with ID {} not found", project_id),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<IssueResponse>::Error {
                error: format!("Database error: {err}"),
            }),
        ),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateIssueRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub assigned_to: Option<Option<String>>,
    pub status: Option<String>,
    pub is_open: Option<bool>,
}

pub async fn update_issue(
    State(state): State<Arc<AppState>>,
    Path((project_id, issue_id)): Path<(i32, i32)>,
    Json(payload): Json<UpdateIssueRequest>,
) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<IssueResponse>::Error {
                    error: format!("DB pool error: {err}"),
                }),
            )
        }
    };

    match Project::find_by_id(&mut conn, project_id) {
        Ok(Some(_)) => match Issue::get_by_id(&mut conn, issue_id) {
            Ok(issue) => {
                if issue.project_id != project_id {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::<IssueResponse>::Error {
                            error: "Issue does not belong to this project".to_string(),
                        }),
                    );
                }

                let changes = IssueChanges {
                    title: payload.title,
                    description: payload.description,
                    assigned_to: payload.assigned_to,
                    status: payload.status,
                    is_open: payload.is_open,
                };

                match issue.update(&mut conn, changes) {
                    Ok(updated_issue) => {
                        let response = ApiResponse::Success(IssueResponse {
                            id: updated_issue.id,
                            project_id: updated_issue.project_id,
                            title: updated_issue.title,
                            description: updated_issue.description,
                            created_by: updated_issue.created_by,
                            assigned_to: updated_issue.assigned_to,
                            status: updated_issue.status,
                            is_open: updated_issue.is_open,
                        });
                        (StatusCode::OK, Json(response))
                    }
                    Err(err) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::<IssueResponse>::Error {
                            error: format!("Failed to update issue: {err}"),
                        }),
                    ),
                }
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<IssueResponse>::Error {
                    error: format!("Database error: {err}"),
                }),
            ),
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<IssueResponse>::Error {
                error: format!("Project with ID {} not found", project_id),
            }),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<IssueResponse>::Error {
                error: format!("Database error: {err}"),
            }),
        ),
    }
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
}

pub async fn delete_issue(
    State(state): State<Arc<AppState>>,
    Path((project_id, issue_id)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let mut conn = match state.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            tracing::error!("Database pool error: {:?}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<DeleteResponse>::Error {
                    error: "Internal server error".to_string(),
                }),
            );
        }
    };

    match Project::find_by_id(&mut conn, project_id) {
        Ok(Some(_)) => match Issue::get_by_id(&mut conn, issue_id) {
            Ok(issue) => {
                if issue.project_id != project_id {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::<DeleteResponse>::Error {
                            error: "Issue does not belong to this project".to_string(),
                        }),
                    );
                }

                match Issue::delete(&mut conn, issue_id) {
                    Ok(true) => {
                        tracing::info!("Deleted issue {} successfully", issue_id);
                        let response = ApiResponse::Success(DeleteResponse {
                            success: true,
                            message: format!("Issue {} successfully deleted", issue_id),
                        });
                        (StatusCode::OK, Json(response))
                    }
                    Ok(false) => {
                        tracing::warn!("Issue {} not found", issue_id);
                        (
                            StatusCode::NOT_FOUND,
                            Json(ApiResponse::<DeleteResponse>::Error {
                                error: format!("Issue {} not found", issue_id),
                            }),
                        )
                    }
                    Err(err) => {
                        tracing::error!("Failed to delete issue {}: {:?}", issue_id, err);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::<DeleteResponse>::Error {
                                error: "Failed to delete issue".to_string(),
                            }),
                        )
                    }
                }
            }
            Err(err) => {
                tracing::error!("Database error retrieving issue {}: {:?}", issue_id, err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<DeleteResponse>::Error {
                        error: "Database error".to_string(),
                    }),
                )
            }
        },
        Ok(None) => {
            tracing::warn!("Project {} not found", project_id);
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::<DeleteResponse>::Error {
                    error: format!("Project with ID {} not found", project_id),
                }),
            )
        }
        Err(err) => {
            tracing::error!(
                "Database error retrieving project {}: {:?}",
                project_id,
                err
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<DeleteResponse>::Error {
                    error: "Database error".to_string(),
                }),
            )
        }
    }
}
