use super::app_context::AppContext;
use crate::presentation::templates::index::{ArticleView, HeaderView};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "thumbnail.html")]
pub struct ThumbnailTemplate {
    pub app: AppContext,
    pub header: HeaderView,
    pub current_title: String,
    pub is_single_article_page: bool,
    pub articles: Vec<ArticleView>,
}

impl IntoResponse for ThumbnailTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => crate::domain::model::error::AppError::TemplateError(err).into_response(),
        }
    }
}
