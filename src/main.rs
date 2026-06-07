use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod data;
mod domain;
mod infrastructure;
mod presentation;

use data::data_source::markdown::local_storage_markdown_data_source::LocalStorageMarkdownDataSource;
use data::repository::markdown_article_repository::MarkdownArticleRepository;
use domain::repository::article_repository::ArticleRepository;
use infrastructure::config::Settings;
use presentation::handlers;
use presentation::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "corkboard=debug,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Corkboard blog platform...");

    let settings = Settings::new().map_err(|e| {
        tracing::error!("Failed to load configuration: {}", e);
        e
    })?;

    if !std::path::Path::new(&settings.articles_dir).exists() {
        tracing::warn!(
            "Articles directory '{}' not found, creating it.",
            settings.articles_dir
        );
        std::fs::create_dir_all(&settings.articles_dir)?;
    }

    let thumbnails_dir = "templates/thumbnails";
    if !std::path::Path::new(thumbnails_dir).exists() {
        tracing::info!(
            "Thumbnails directory '{}' not found, creating it.",
            thumbnails_dir
        );
        std::fs::create_dir_all(thumbnails_dir)?;
    }

    tracing::info!("Parsing articles from '{}'...", settings.articles_dir);
    let data_source = LocalStorageMarkdownDataSource::new(settings.articles_dir.clone());
    let repo = MarkdownArticleRepository::new(data_source, settings.truncate_lines);
    let articles = repo.load_all().map_err(|e| {
        tracing::error!("Failed to load articles: {}", e);
        e
    })?;
    tracing::info!("Loaded {} articles successfully.", articles.len());

    let state = AppState {
        settings: settings.clone(),
        articles: Arc::new(articles),
    };

    let app = presentation::router::build_router(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], settings.port));
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let port = settings.port;
    let blog_title = settings.blog_title.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        handlers::thumbnail::generate_startup_thumbnail(port, blog_title).await;
    });

    axum::serve(listener, app).await?;

    Ok(())
}
