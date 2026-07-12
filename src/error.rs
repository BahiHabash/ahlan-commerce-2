use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    ValidationFailed,
    NotFound,
    DependencyFailed,
    Internal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorEnvelope {
    pub error: ErrorBody,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorBody {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("dependency failed: {message}")]
    Dependency {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn dependency(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Dependency {
            message: message.into(),
            source: Box::new(source),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Dependency { .. } => StatusCode::SERVICE_UNAVAILABLE,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Validation(_) => ErrorCode::ValidationFailed,
            Self::NotFound(_) => ErrorCode::NotFound,
            Self::Dependency { .. } => ErrorCode::DependencyFailed,
            Self::Internal(_) => ErrorCode::Internal,
        }
    }

    fn public_message(&self) -> String {
        match self {
            Self::Validation(message) | Self::NotFound(message) => message.clone(),
            Self::Dependency { message, .. } => message.clone(),
            Self::Internal(_) => "internal server error".to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let code = self.code();
        error!(error_code = ?code, error = %self, "request failed");
        let body = ErrorEnvelope {
            error: ErrorBody {
                code,
                message: self.public_message(),
            },
        };
        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
