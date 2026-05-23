use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum::http::StatusCode;
use crate::domain::article::Article;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub blog_title: String,
    pub articles: Vec<Article>,
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}
