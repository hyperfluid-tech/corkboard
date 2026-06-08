use crate::domain::model::error::AppError;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub blog_title: String,
    pub blog_author: String,
    pub blog_license: String,
    pub blog_license_url: url::Url,
    pub articles_dir: String,
    pub port: u16,
    pub truncate_lines: usize,
    pub lang: String,
    #[serde(default)]
    pub social_links: Vec<url::Url>,
    pub thumbnail_show_articles: bool,
}

impl Settings {
    pub fn new() -> Result<Self, AppError> {
        Self::load().map_err(|e| AppError::InvalidConfig(e.to_string()))
    }

    fn load() -> Result<Self, config::ConfigError> {
        let s = Config::builder()
            .set_default("port", 3000)?
            .set_default("articles_dir", "articles")?
            .set_default("blog_title", "My blog")?
            .set_default("blog_author", "Author")?
            .set_default("blog_license", "CC 4.0 BY-SA")?
            .set_default(
                "blog_license_url",
                "https://creativecommons.org/licenses/by-sa/4.0/",
            )?
            .set_default("truncate_lines", 15)?
            .set_default("lang", "en")?
            .set_default("thumbnail_show_articles", false)?
            .add_source(File::with_name("config").required(false))
            .add_source(
                config::Environment::with_prefix("CORKBOARD")
                    .list_separator(",")
                    .with_list_parse_key("social_links")
                    .try_parsing(true),
            )
            .build()?;

        s.try_deserialize()
    }
}
