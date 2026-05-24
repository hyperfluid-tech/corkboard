use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum::http::StatusCode;
use crate::domain::article::Article;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub blog_title: String,
    pub blog_author: String,
    pub blog_license: String,
    pub blog_license_url: String,
    pub current_year: i32,
    pub articles: Vec<Article>,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub twitter_url: Option<String>,
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
