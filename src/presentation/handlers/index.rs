use axum::extract::State;
use axum::response::IntoResponse;
use crate::presentation::state::AppState;
use crate::presentation::templates::index::IndexTemplate;
use chrono::Datelike;

pub async fn index_handler(State(state): State<AppState>) -> impl IntoResponse {
    let current_year = chrono::Utc::now().year();
    IndexTemplate {
        blog_title: state.settings.blog_title.clone(),
        blog_author: state.settings.blog_author.clone(),
        blog_license: state.settings.blog_license.clone(),
        blog_license_url: state.settings.blog_license_url.clone(),
        current_year,
        articles: (*state.articles).clone(),
    }
}
