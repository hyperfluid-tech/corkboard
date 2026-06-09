use super::components::footer_template::FooterTemplate;
use super::components::header_template::HeaderTemplate;
use crate::domain::model::error::AppError;
use crate::presentation::model::app_context::AppContext;
use crate::presentation::model::sidebar_entry::SidebarEntry;
use crate::presentation::templates::index_template::ArticleView;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate {
    pub app: AppContext,
    pub header: HeaderTemplate,
    pub footer: FooterTemplate,
    pub article: ArticleView,
    pub sidebar_entries: Vec<SidebarEntry>,
    pub is_single_article_page: bool,
    pub structured_data: super::components::structured_data_template::StructuredDataTemplate,
}

impl IntoResponse for ArticleTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => AppError::TemplateError(err).into_response(),
        }
    }
}
