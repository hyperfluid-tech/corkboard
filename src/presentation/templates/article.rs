use crate::presentation::templates::index::{ArticleView, FooterView, HeaderView};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate {
    pub header: HeaderView,
    pub footer: FooterView,
    pub article: ArticleView,
    pub articles: Vec<ArticleView>,
    pub is_single_article_page: bool,
}

impl IntoResponse for ArticleTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => crate::domain::error::AppError::TemplateError(err).into_response(),
        }
    }
}
