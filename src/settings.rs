use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub blog_title: String,
    pub articles_dir: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .set_default("port", 8080)?
            .set_default("articles_dir", "articles")?
            .set_default("blog_title", "My Rust Blog")?
            .add_source(File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("CARBON"))
            .build()?;

        s.try_deserialize()
    }
}
