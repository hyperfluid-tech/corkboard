use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
#[derive(Debug, Clone)]
pub struct ArticleView {
    pub slug: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub content: String,
    pub has_more_content: bool,
    pub subheading: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HeaderView {
    pub blog_title: String,
    pub blog_author: String,
}

#[derive(Debug, Clone)]
pub struct FooterView {
    pub blog_author: String,
    pub blog_license: String,
    pub blog_license_url: String,
    pub current_year: i32,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub twitter_url: Option<String>,
    pub version: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub header: HeaderView,
    pub footer: FooterView,
    pub articles: Vec<ArticleView>,
    pub is_single_article_page: bool,
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => crate::domain::error::AppError::TemplateError(err).into_response(),
        }
    }
}
