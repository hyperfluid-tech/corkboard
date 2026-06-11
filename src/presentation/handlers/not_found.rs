use crate::presentation::model::app_context::AppContext;
use crate::presentation::state::AppState;
use crate::presentation::templates::components::footer_template::FooterTemplate;
use crate::presentation::templates::components::header_template::HeaderTemplate;
use crate::presentation::templates::not_found_template::NotFoundTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use chrono::Datelike;

pub async fn not_found_handler(State(state): State<AppState>) -> impl IntoResponse {
    let current_year = chrono::Utc::now().year();
    let app = AppContext::new();

    let header = HeaderTemplate {
        blog_title: state.settings.blog_title.clone(),
        blog_author: state.settings.blog_author.clone(),
        base_url: state.settings.base_url.clone(),
        lang: state.settings.lang.clone(),
        is_single_article_page: false,
    };

    let footer = FooterTemplate::new(
        app.clone(),
        state.settings.blog_author.clone(),
        state.settings.blog_license.clone(),
        state.settings.blog_license_url.clone(),
        current_year,
        &state.settings.social_links,
    );

    NotFoundTemplate {
        app,
        header,
        footer,
        sidebar_entries: Vec::new(),
        is_single_article_page: false,
    }
}
