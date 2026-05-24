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
        linkedin_url: state.settings.linkedin_url.as_ref().filter(|s| !s.is_empty()).cloned(),
        github_url: state.settings.github_url.as_ref().filter(|s| !s.is_empty()).cloned(),
        twitter_url: state.settings.twitter_url.as_ref().filter(|s| !s.is_empty()).cloned(),
    }
}
