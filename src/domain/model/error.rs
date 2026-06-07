use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    TemplateError(askama::Error),
    ArticleNotFound,
}

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
        }
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::TemplateError(err)
    }
}
