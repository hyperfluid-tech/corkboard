use crate::domain::model::article::Article;
use crate::presentation::state::AppState;
use askama::Template;
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "sitemap.xml")]
struct SitemapTemplate<'a> {
    base_url: &'a str,
    articles: &'a [Article],
}

pub async fn sitemap_handler(State(state): State<AppState>) -> Response {
    let template = SitemapTemplate {
        base_url: &state.settings.base_url,
        articles: &state.articles,
    };

    match template.render() {
        Ok(xml) => (
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
            xml,
        )
            .into_response(),
        Err(_) => Response::builder()
            .status(500)
            .body("Error generating sitemap".into())
            .unwrap(),
    }
}
