use super::components::footer_template::FooterTemplate;
use super::components::header_template::HeaderTemplate;
use crate::presentation::model::app_context::AppContext;
use crate::presentation::model::sidebar_entry::SidebarEntry;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Debug, Clone)]
pub struct ArticleView {
    pub slug: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub content: String,
    pub has_more_content: bool,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub app: AppContext,
    pub header: HeaderTemplate,
    pub footer: FooterTemplate,
    pub articles: Vec<ArticleView>,
    pub sidebar_entries: Vec<SidebarEntry>,
    pub is_single_article_page: bool,
    pub structured_data: super::components::structured_data_template::StructuredDataTemplate,
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => crate::domain::model::error::AppError::TemplateError(err).into_response(),
        }
    }
}
