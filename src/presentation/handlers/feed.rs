use crate::presentation::state::AppState;
use crate::presentation::templates::feed_template::FeedTemplate;
use axum::extract::State;
use axum::response::IntoResponse;

pub async fn feed_handler(State(state): State<AppState>) -> impl IntoResponse {
    FeedTemplate {
        base_url: state.settings.base_url.clone(),
        blog_title: state.settings.blog_title.clone(),
        blog_author: state.settings.blog_author.clone(),
        articles: (*state.articles).clone(),
    }
}
