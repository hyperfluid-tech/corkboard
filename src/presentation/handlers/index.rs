use axum::extract::State;
use axum::response::IntoResponse;
use crate::presentation::state::AppState;
use crate::presentation::templates::index::IndexTemplate;

pub async fn index_handler(State(state): State<AppState>) -> impl IntoResponse {
    IndexTemplate {
        blog_title: state.settings.blog_title.clone(),
        articles: (*state.articles).clone(),
    }
}
