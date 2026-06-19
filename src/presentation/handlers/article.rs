use crate::presentation::model::app_context::AppContext;
use crate::presentation::model::sidebar_entry::SidebarEntry;
use crate::presentation::model::structured_data::StructuredData;
use crate::presentation::state::AppState;
use crate::presentation::templates::article_template::ArticleTemplate;
use crate::presentation::templates::components::footer_template::FooterTemplate;
use crate::presentation::templates::components::header_template::HeaderTemplate;
use crate::presentation::templates::components::structured_data_template::StructuredDataTemplate;
use crate::presentation::templates::index_template::ArticleView;
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
                description: article.description.clone(),
                thumbnail: article.thumbnail.clone(),
            };

            let app = AppContext::new();

            let header = HeaderTemplate {
                blog_title: state.settings.blog_title.clone(),
                blog_author: state.settings.blog_author.clone(),
                base_url: state.settings.base_url.clone(),
                lang: state.settings.lang.clone(),
                is_single_article_page: true,
            };

            let footer = FooterTemplate::new(
                app.clone(),
                state.settings.blog_author.clone(),
                state.settings.blog_license.clone(),
                state.settings.blog_license_url.clone(),
                current_year,
                &state.settings.social_links,
            );

            let sidebar_entries: Vec<SidebarEntry> = std::iter::once(SidebarEntry::from_article(article))
                .chain(article.toc.iter().map(SidebarEntry::from_toc_entry))
                .collect();

            let structured_data = StructuredDataTemplate::new(&StructuredData::blog_posting(
                article,
                &state.settings.base_url,
                state.settings.blog_author.clone(),
                state.settings.blog_title.clone(),
            ));

            let template = ArticleTemplate {
                app,
                header,
                footer,
                article: article_view,
                sidebar_entries,
                is_single_article_page: true,
                structured_data,
            };
            template.into_response()
        }
        None => super::not_found::not_found_handler(State(state))
            .await
            .into_response(),
    }
}
