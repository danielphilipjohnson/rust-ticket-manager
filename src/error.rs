use crate::handlers::api_response::ApiResponse;

use axum::http::StatusCode;
use axum::Json;
use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DieselError),

    #[error("Database connection error: {0}")]
    DatabaseConnection(#[from] diesel::r2d2::PoolError),

    #[error("Environment error: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Issue not found: {0}")]
    IssueNotFound(i32),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<AppError> for (i32, String) {
    fn from(error: AppError) -> Self {
        match error {
            AppError::ProjectNotFound(_) | AppError::IssueNotFound(_) => (404, error.to_string()),
            AppError::Validation(_) => (400, error.to_string()),
            AppError::Database(diesel_error) => match diesel_error {
                DieselError::NotFound => (404, "Record not found".to_string()),
                DieselError::DatabaseError(kind, _) => {
                    use diesel::result::DatabaseErrorKind::*;
                    match kind {
                        UniqueViolation => (409, "Record already exists".to_string()),
                        ForeignKeyViolation => (400, "Invalid reference".to_string()),
                        _ => (500, "Database error".to_string()),
                    }
                }
                _ => (500, "Internal server error".to_string()),
            },
            _ => (500, "Internal server error".to_string()),
        }
    }
}

pub fn internal_server_error<T>(
    message: &str,
    err: &dyn std::error::Error,
) -> (StatusCode, Json<ApiResponse<T>>) {
    tracing::error!("{}: {:?}", message, err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiResponse::Error {
            error: message.to_string(),
        }),
    )
}
