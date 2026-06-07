use crate::presentation::model::sidebar_entry::SidebarEntry;
use crate::presentation::state::AppState;
use crate::presentation::templates::app_context::AppContext;
use crate::presentation::templates::index::{ArticleView, FooterView, HeaderView, IndexTemplate};
use axum::extract::State;
use axum::response::IntoResponse;
use chrono::Datelike;

pub async fn index_handler(State(state): State<AppState>) -> impl IntoResponse {
    let current_year = chrono::Utc::now().year();

    let articles: Vec<ArticleView> = state
        .articles
        .iter()
        .map(|a| ArticleView {
            slug: a.slug.clone(),
            title: a.title.clone(),
            date: a.date,
            content: a.preview.clone(),
            has_more_content: a.has_more_content,
            description: a.description.clone(),
            thumbnail: a.thumbnail.clone(),
        })
        .collect();

    let app = AppContext::new();

    let header = HeaderView {
        blog_title: state.settings.blog_title.clone(),
        blog_author: state.settings.blog_author.clone(),
        lang: state.settings.lang.clone(),
    };

    let footer = FooterView {
        blog_author: state.settings.blog_author.clone(),
        blog_license: state.settings.blog_license.clone(),
        blog_license_url: state.settings.blog_license_url.clone(),
        current_year,
        linkedin_url: state
            .settings
            .linkedin_url
            .as_ref()
            .filter(|s| !s.is_empty())
            .cloned(),
        github_url: state
            .settings
            .github_url
            .as_ref()
            .filter(|s| !s.is_empty())
            .cloned(),
        twitter_url: state
            .settings
            .twitter_url
            .as_ref()
            .filter(|s| !s.is_empty())
            .cloned(),
    };

    let sidebar_entries: Vec<SidebarEntry> = state
        .articles
        .iter()
        .map(SidebarEntry::from_article)
        .collect();

    IndexTemplate {
        app,
        header,
        footer,
        articles,
        sidebar_entries,
        is_single_article_page: false,
    }
}
