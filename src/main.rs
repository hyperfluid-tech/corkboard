use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod domain;
mod infrastructure;
mod presentation;
mod use_cases;

use infrastructure::config::Settings;
use presentation::state::AppState;
use use_cases::article_loader::load_articles;
use presentation::handlers;

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
        tracing::warn!("Articles directory '{}' not found, creating it.", settings.articles_dir);
        std::fs::create_dir_all(&settings.articles_dir)?;
    }

    let thumbnails_dir = "templates/thumbnails";
    if !std::path::Path::new(thumbnails_dir).exists() {
        tracing::info!("Thumbnails directory '{}' not found, creating it.", thumbnails_dir);
        std::fs::create_dir_all(thumbnails_dir)?;
    }

    tracing::info!("Parsing articles from '{}'...", settings.articles_dir);
    let articles = load_articles(&settings.articles_dir, settings.truncate_lines).map_err(|e| {
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
