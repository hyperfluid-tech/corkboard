use crate::presentation::templates::index::HeaderView;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "thumbnail.html")]
pub struct ThumbnailTemplate {
    pub header: HeaderView,
    pub current_title: String,
}

impl IntoResponse for ThumbnailTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => crate::domain::error::AppError::TemplateError(err).into_response(),
        }
    }
}
