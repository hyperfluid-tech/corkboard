use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub blog_title: String,
    pub blog_author: String,
    pub blog_license: String,
    pub blog_license_url: String,
    pub articles_dir: String,
    pub port: u16,
    pub truncate_lines: usize,
    pub lang: String,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub twitter_url: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
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
            .add_source(File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("CORKBOARD"))
            .build()?;

        s.try_deserialize()
    }
}
