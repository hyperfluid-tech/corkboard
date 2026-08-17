use crate::domain::model::error::AppError;
use config::{Config, File};
use serde::Deserialize;

pub const DEFAULT_PORT: u16 = 3000;
pub const DEFAULT_ARTICLES_DIR: &str = "articles";
pub const DEFAULT_BLOG_TITLE: &str = "My blog";
pub const DEFAULT_BLOG_AUTHOR: &str = "Author";
pub const DEFAULT_BASE_URL: &str = "http://localhost:3000";
pub const DEFAULT_BLOG_LICENSE: &str = "CC 4.0 BY-SA";
pub const DEFAULT_BLOG_LICENSE_URL: &str = "https://creativecommons.org/licenses/by-sa/4.0/";
pub const DEFAULT_TRUNCATE_LINES: i64 = 15;
pub const DEFAULT_PREVIEW_INCLUDE_IMAGES: bool = false;
pub const DEFAULT_LANG: &str = "en";
pub const DEFAULT_THUMBNAIL_SHOW_ARTICLES: bool = false;

pub const DEFAULT_CSP_DOMAINS: &[&str] =
    &["https://fonts.googleapis.com", "https://fonts.gstatic.com"];

pub const LOCALHOST_PLACEHOLDER: &str = "localhost";
pub const DOMAIN_PLACEHOLDER: &str = "your-domain-here.com";

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub blog_title: String,
    pub blog_author: String,
    pub base_url: String,
    pub blog_license: String,
    pub blog_license_url: url::Url,
    pub articles_dir: String,
    pub port: u16,
    pub truncate_lines: usize,
    pub preview_include_images: bool,
    pub lang: String,
    #[serde(default)]
    pub social_links: Vec<url::Url>,
    pub thumbnail_show_articles: bool,
    #[serde(default)]
    pub cors_allowed_origins: Option<Vec<String>>,
    #[serde(default)]
    pub csp_allowed_origins: Option<Vec<String>>,
    #[cfg(feature = "git")]
    pub git: Option<GitSettings>,
}

#[cfg(feature = "git")]
#[derive(Debug, Deserialize, Clone)]
pub struct GitSettings {
    pub link: String,
    #[serde(default = "default_git_folder")]
    pub folder: String,
    #[serde(default = "default_git_assets_folder")]
    pub assets_folder: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default = "default_git_branch")]
    pub branch: String,
}

#[cfg(feature = "git")]
fn default_git_folder() -> String {
    String::from("")
}

#[cfg(feature = "git")]
fn default_git_assets_folder() -> String {
    String::from("")
}

#[cfg(feature = "git")]
fn default_git_branch() -> String {
    String::from("main")
}

impl Settings {
    pub fn new() -> Result<Self, AppError> {
        let settings = Self::load().map_err(|e| AppError::InvalidConfig(e.to_string()))?;
        settings.validate()?;
        Ok(settings)
    }

    pub fn validate(&self) -> Result<(), AppError> {
        for origin in self.cors_allowed_origins.iter().flatten() {
            origin.parse::<axum::http::HeaderValue>().map_err(|e| {
                AppError::InvalidConfig(format!("Invalid CORS origin '{}': {}", origin, e))
            })?;
        }

        for origin in self.csp_allowed_origins.iter().flatten() {
            origin.parse::<axum::http::HeaderValue>().map_err(|e| {
                AppError::InvalidConfig(format!("Invalid CSP origin '{}': {}", origin, e))
            })?;
        }

        self.base_url
            .parse::<axum::http::HeaderValue>()
            .map_err(|e| {
                AppError::InvalidConfig(format!(
                    "Invalid base_url configuration '{}': {}",
                    self.base_url, e
                ))
            })?;

        Ok(())
    }

    pub fn cors_origins(&self) -> Vec<String> {
        self.cors_allowed_origins
            .clone()
            .unwrap_or_else(|| vec![self.base_url.clone()])
    }

    pub fn csp_origins(&self) -> Vec<String> {
        self.csp_allowed_origins.clone().unwrap_or_else(|| {
            let mut origins = vec![self.base_url.clone()];
            origins.extend(DEFAULT_CSP_DOMAINS.iter().map(|s| s.to_string()));
            origins
        })
    }

    fn load() -> Result<Self, config::ConfigError> {
        let s = Config::builder()
            .set_default("port", DEFAULT_PORT)?
            .set_default("articles_dir", DEFAULT_ARTICLES_DIR)?
            .set_default("blog_title", DEFAULT_BLOG_TITLE)?
            .set_default("blog_author", DEFAULT_BLOG_AUTHOR)?
            .set_default("base_url", DEFAULT_BASE_URL)?
            .set_default("blog_license", DEFAULT_BLOG_LICENSE)?
            .set_default("blog_license_url", DEFAULT_BLOG_LICENSE_URL)?
            .set_default("truncate_lines", DEFAULT_TRUNCATE_LINES)?
            .set_default("preview_include_images", DEFAULT_PREVIEW_INCLUDE_IMAGES)?
            .set_default("lang", DEFAULT_LANG)?
            .set_default("thumbnail_show_articles", DEFAULT_THUMBNAIL_SHOW_ARTICLES)?
            .add_source(File::with_name("config").required(false))
            .add_source(
                config::Environment::with_prefix("CORKBOARD")
                    .prefix_separator("_")
                    .separator("__")
                    .list_separator(",")
                    .with_list_parse_key("social_links")
                    .with_list_parse_key("cors_allowed_origins")
                    .with_list_parse_key("csp_allowed_origins")
                    .try_parsing(true),
            )
            .build()?;

        s.try_deserialize()
    }

    pub fn check_defaults(&self) {
        if self.base_url == DEFAULT_BASE_URL || self.base_url.contains(LOCALHOST_PLACEHOLDER) {
            tracing::warn!(
                "Using default or localhost for base_url ('{}'). Sitemaps, RSS feeds, and social cards may have incorrect links in production.",
                self.base_url
            );
        } else if self.base_url.contains(DOMAIN_PLACEHOLDER) {
            tracing::warn!(
                "Placeholder '{}' detected in base_url. Please update it with your actual domain in production.",
                DOMAIN_PLACEHOLDER
            );
        }

        if self.blog_title == DEFAULT_BLOG_TITLE {
            tracing::warn!(
                "Using default blog_title ('{}'). Please customize this in your configuration.",
                DEFAULT_BLOG_TITLE
            );
        }

        if self.blog_author == DEFAULT_BLOG_AUTHOR {
            tracing::warn!(
                "Using default blog_author ('{}'). Please customize this in your configuration.",
                DEFAULT_BLOG_AUTHOR
            );
        }
    }
}
