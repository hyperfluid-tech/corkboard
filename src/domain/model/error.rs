use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    TemplateError(askama::Error),
    ArticleNotFound,
    InvalidConfig(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::TemplateError(err) => write!(f, "Template error: {}", err),
            AppError::ArticleNotFound => write!(f, "Article not found"),
            AppError::InvalidConfig(err) => write!(f, "Invalid configuration: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::TemplateError(err) => {
                let error_message = format!("Failed to render template: {}", err);
                tracing::error!("{}", error_message);
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::ArticleNotFound => {
                (StatusCode::NOT_FOUND, "Article not found").into_response()
            }
            AppError::InvalidConfig(err) => {
                let error_message = format!("Invalid configuration: {}", err);
                tracing::error!("{}", error_message);
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::TemplateError(err)
    }
}
