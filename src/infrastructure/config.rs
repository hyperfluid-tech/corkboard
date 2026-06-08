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
    #[serde(default, deserialize_with = "deserialize_social_links")]
    pub social_links: Vec<String>,
    pub thumbnail_show_articles: bool,
}

pub use deserializers::deserialize_social_links;

mod deserializers {
    use serde::Deserializer;
    use serde::de::{SeqAccess, Visitor};

    pub fn deserialize_social_links<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SocialLinksVisitor;

        impl<'de> Visitor<'de> for SocialLinksVisitor {
            type Value = Vec<String>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of strings or a comma-separated string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect())
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut links = Vec::new();
                while let Some(value) = seq.next_element::<String>()? {
                    let trimmed = value.trim().to_string();
                    if !trimmed.is_empty() {
                        links.push(trimmed);
                    }
                }
                Ok(links)
            }
        }

        deserializer.deserialize_any(SocialLinksVisitor)
    }
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
            .set_default("thumbnail_show_articles", false)?
            .add_source(File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("CORKBOARD"))
            .build()?;

        s.try_deserialize()
    }
}
