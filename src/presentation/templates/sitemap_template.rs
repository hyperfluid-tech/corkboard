use crate::domain::model::article::Article;
use crate::domain::model::error::AppError;
use askama::Template;
use axum::http::header;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "sitemap.xml")]
pub struct SitemapTemplate {
    pub base_url: String,
    pub articles: Vec<Article>,
}

impl IntoResponse for SitemapTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(xml) => (
                [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
                xml,
            )
                .into_response(),
            Err(err) => AppError::SitemapGeneration(err).into_response(),
        }
    }
}
