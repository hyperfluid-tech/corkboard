use crate::domain::model::error::AppError;
use crate::presentation::state::AppState;
use askama::Template;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "robots.txt")]
struct RobotsTemplate<'a> {
    base_url: &'a str,
}

pub async fn robots_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let template = RobotsTemplate {
        base_url: &state.settings.base_url,
    };

    let text = template.render().map_err(AppError::RobotsGeneration)?;
    Ok(([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], text))
}
