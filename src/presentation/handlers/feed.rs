use crate::domain::model::article::Article;
use crate::presentation::state::AppState;
use askama::Template;
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "feed.xml")]
struct FeedTemplate<'a> {
    base_url: &'a str,
    blog_title: &'a str,
    blog_author: &'a str,
    articles: &'a [Article],
}

pub async fn feed_handler(State(state): State<AppState>) -> Response {
    let template = FeedTemplate {
        base_url: &state.settings.base_url,
        blog_title: &state.settings.blog_title,
        blog_author: &state.settings.blog_author,
        articles: &state.articles,
    };

    match template.render() {
        Ok(xml) => (
            [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
            xml,
        )
            .into_response(),
        Err(_) => Response::builder()
            .status(500)
            .body("Error generating feed".into())
            .unwrap(),
    }
}
