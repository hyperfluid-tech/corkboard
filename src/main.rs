use std::collections::HashSet;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod data;
mod domain;
mod infrastructure;
mod presentation;

#[cfg(feature = "git")]
use data::data_source::markdown::git_markdown_data_source::GitMarkdownDataSource;
use data::data_source::markdown::local_storage_markdown_data_source::LocalStorageMarkdownDataSource;
use data::repository::markdown_article_repository::MarkdownArticleRepository;
use domain::repository::article_repository::ArticleRepository;
use domain::service::article_service::ArticleService;
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

    settings.check_defaults();

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

    {
        use infrastructure::markdown::highlight_css_generator::write_highlight_css;
        let highlight_css_path = "templates/style/syntax.css";
        if let Err(e) = write_highlight_css(highlight_css_path) {
            tracing::warn!("Failed to write syntax.css: {}", e);
        }
    }

    tracing::info!("Parsing articles from '{}'...", settings.articles_dir);
    let data_source = LocalStorageMarkdownDataSource::new(settings.articles_dir.clone());
    let repo = MarkdownArticleRepository::new(
        data_source,
        settings.truncate_lines,
        settings.preview_include_images,
    );

    #[allow(unused_mut)]
    let mut repos: Vec<Box<dyn ArticleRepository + Send + Sync>> = vec![Box::new(repo)];

    #[cfg(feature = "git")]
    if let Some(git) = &settings.git {
        tracing::info!("Initializing Git data source...");
        match GitMarkdownDataSource::new(
            &git.link,
            &git.folder,
            &git.assets_folder,
            git.username.as_deref(),
            git.password.as_deref(),
            &git.branch,
        ) {
            Ok(git_source) => {
                repos.push(Box::new(MarkdownArticleRepository::new(
                    git_source,
                    settings.truncate_lines,
                    settings.preview_include_images,
                )));
            }
            Err(e) => {
                tracing::error!("Failed to initialize Git data source: {}", e);
            }
        }
    }

    let articles = {
        let article_service = ArticleService::new(repos);
        article_service.get_all_articles().map_err(|e| {
            tracing::error!("Failed to load and merge articles: {}", e);
            e
        })?
    };

    tracing::info!("Loaded {} articles successfully.", articles.len());

    let allowed_assets: HashSet<String> = articles
        .iter()
        .flat_map(|a| a.referenced_assets.iter().cloned())
        .collect();

    if !allowed_assets.is_empty() {
        tracing::info!(
            "Registered {} allowed asset path(s) for serving.",
            allowed_assets.len()
        );
    }

    let allowed_external_origins: HashSet<String> = articles
        .iter()
        .flat_map(|a| a.referenced_external_origins.iter().cloned())
        .collect();

    if !allowed_external_origins.is_empty() {
        tracing::info!(
            "Registered {} allowed external asset origin(s) for CSP.",
            allowed_external_origins.len()
        );
    }

    let state = AppState {
        settings: settings.clone(),
        articles: Arc::new(articles),
        allowed_assets: Arc::new(allowed_assets),
        allowed_external_origins: Arc::new(allowed_external_origins),
    };

    let app = presentation::router::build_router(state)?;

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
