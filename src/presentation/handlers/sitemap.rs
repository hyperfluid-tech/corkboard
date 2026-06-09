use crate::presentation::state::AppState;
use crate::presentation::templates::sitemap_template::SitemapTemplate;
use axum::extract::State;
use axum::response::IntoResponse;

pub async fn sitemap_handler(State(state): State<AppState>) -> impl IntoResponse {
    SitemapTemplate {
        base_url: state.settings.base_url.clone(),
        articles: (*state.articles).clone(),
    }
}
