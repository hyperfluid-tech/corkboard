use crate::presentation::state::AppState;
use crate::presentation::templates::article::ArticleTemplate;
use crate::presentation::templates::index::{ArticleView, FooterView, HeaderView};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use chrono::Datelike;

pub async fn article_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let current_year = chrono::Utc::now().year();

    let matching_article = state.articles.iter().find(|a| a.slug == slug);

    match matching_article {
        Some(article) => {
            let article_view = ArticleView {
                slug: article.slug.clone(),
                title: article.title.clone(),
                date: article.date,
                content: article.content.clone(),
                has_more_content: article.has_more_content,
                subheading: article.subheading.clone(),
                thumbnail: article.thumbnail.clone(),
            };

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
                version: env!("CARGO_PKG_VERSION").to_string(),
            };

            let template = ArticleTemplate {
                header,
                footer,
                article: article_view,
                articles: Vec::new(),
                is_single_article_page: true,
            };
            template.into_response()
        }
        None => crate::domain::error::AppError::ArticleNotFound.into_response(),
    }
}
