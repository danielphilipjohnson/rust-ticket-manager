use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success(T),
    Error { error: String },
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse::Success(data)
    }

    pub fn error(message: impl Into<String>) -> Self {
        ApiResponse::Error {
            error: message.into(),
        }
    }
}
