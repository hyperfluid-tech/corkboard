use super::components::footer_template::FooterTemplate;
use super::components::header_template::HeaderTemplate;
use crate::domain::model::error::AppError;
use crate::presentation::model::app_context::AppContext;
use crate::presentation::model::sidebar_entry::SidebarEntry;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate {
    pub app: AppContext,
    pub header: HeaderTemplate,
    pub footer: FooterTemplate,
    pub sidebar_entries: Vec<SidebarEntry>,
    pub is_single_article_page: bool,
}

impl IntoResponse for NotFoundTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
            Err(err) => AppError::TemplateError(err).into_response(),
        }
    }
}
