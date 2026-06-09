use crate::domain::model::article::Article;
use crate::domain::model::error::AppError;
use crate::presentation::state::AppState;
use askama::Template;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "sitemap.xml")]
struct SitemapTemplate<'a> {
    base_url: &'a str,
    articles: &'a [Article],
}

pub async fn sitemap_handler(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let template = SitemapTemplate {
        base_url: &state.settings.base_url,
        articles: &state.articles,
    };

    let xml = template.render().map_err(AppError::SitemapGeneration)?;
    Ok((
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    ))
}
