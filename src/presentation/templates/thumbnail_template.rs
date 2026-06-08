use super::components::header_template::HeaderTemplate;
use crate::presentation::model::app_context::AppContext;
use crate::presentation::templates::index_template::ArticleView;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "thumbnail.html")]
pub struct ThumbnailTemplate {
    pub app: AppContext,
    pub header: HeaderTemplate,
    pub current_title: String,
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
