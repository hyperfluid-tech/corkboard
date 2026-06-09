use crate::domain::model::article::Article;
use crate::domain::model::error::AppError;
use askama::Template;
use axum::http::header;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "feed.xml")]
pub struct FeedTemplate {
    pub base_url: String,
    pub blog_title: String,
    pub blog_author: String,
    pub articles: Vec<Article>,
}

impl IntoResponse for FeedTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(xml) => (
                [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
                xml,
            )
                .into_response(),
            Err(err) => AppError::FeedGeneration(err).into_response(),
        }
    }
}
