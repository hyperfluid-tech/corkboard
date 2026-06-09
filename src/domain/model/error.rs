use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    TemplateError(askama::Error),
    InvalidConfig(String),
    FeedGeneration(askama::Error),
    RobotsGeneration(askama::Error),
    SitemapGeneration(askama::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::TemplateError(err) => write!(f, "Template error: {}", err),
            AppError::InvalidConfig(err) => write!(f, "Invalid configuration: {}", err),
            AppError::FeedGeneration(err) => write!(f, "Feed generation error: {}", err),
            AppError::RobotsGeneration(err) => write!(f, "Robots generation error: {}", err),
            AppError::SitemapGeneration(err) => write!(f, "Sitemap generation error: {}", err),
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
            AppError::InvalidConfig(err) => {
                let error_message = format!("Invalid configuration: {}", err);
                tracing::error!("{}", error_message);
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::FeedGeneration(err) => {
                let error_message = format!("Error generating feed: {}", err);
                tracing::error!("{}", error_message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error generating feed").into_response()
            }
            AppError::RobotsGeneration(err) => {
                let error_message = format!("Error generating robots.txt: {}", err);
                tracing::error!("{}", error_message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error generating robots.txt",
                )
                    .into_response()
            }
            AppError::SitemapGeneration(err) => {
                let error_message = format!("Error generating sitemap: {}", err);
                tracing::error!("{}", error_message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error generating sitemap",
                )
                    .into_response()
            }
        }
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::TemplateError(err)
    }
}
