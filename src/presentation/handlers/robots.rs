use crate::presentation::state::AppState;
use askama::Template;
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};

#[derive(Template)]
#[template(path = "robots.txt")]
struct RobotsTemplate<'a> {
    base_url: &'a str,
}

pub async fn robots_handler(State(state): State<AppState>) -> Response {
    let template = RobotsTemplate {
        base_url: &state.settings.base_url,
    };

    match template.render() {
        Ok(text) => ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], text).into_response(),
        Err(_) => Response::builder()
            .status(500)
            .body("Error generating robots.txt".into())
            .unwrap(),
    }
}
