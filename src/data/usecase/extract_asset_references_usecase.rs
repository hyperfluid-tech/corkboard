use pulldown_cmark::{Event, Options, Parser, Tag};
use std::collections::HashSet;

pub struct AssetReferences {
    pub local_assets: HashSet<String>,
    pub external_origins: HashSet<String>,
}

pub struct ExtractAssetReferencesUsecase;

impl ExtractAssetReferencesUsecase {
    pub fn execute(markdown_body: &str, thumbnail: Option<&str>) -> AssetReferences {
        let mut local_assets = HashSet::new();
        let mut external_origins = HashSet::new();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        let parser = Parser::new_ext(markdown_body, options);

        for event in parser {
            if let Event::Start(Tag::Image { dest_url, .. }) = event {
                Self::process_asset_url(&dest_url, &mut local_assets, &mut external_origins);
            }
        }

        if let Some(thumb) = thumbnail {
            Self::process_asset_url(thumb, &mut local_assets, &mut external_origins);
        }

        AssetReferences {
            local_assets,
            external_origins,
        }
    }

    fn process_asset_url(
        url: &str,
        local_assets: &mut HashSet<String>,
        external_origins: &mut HashSet<String>,
    ) {
        if let Some(origin) = Self::extract_external_origin(url) {
            external_origins.insert(origin);
        } else if let Some(normalized) = Self::normalize_asset_path(url) {
            local_assets.insert(normalized);
        }
    }

    fn extract_external_origin(url_str: &str) -> Option<String> {
        if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
            return None;
        }

        let parsed = url::Url::parse(url_str).ok()?;
        let host = parsed.host_str()?;
        let scheme = parsed.scheme();

        let origin = match parsed.port() {
            Some(port) => format!("{}://{}:{}", scheme, host, port),
            None => format!("{}://{}", scheme, host),
        };

        Some(origin)
    }

    fn normalize_asset_path(url: &str) -> Option<String> {
        if url.starts_with("http://") || url.starts_with("https://") {
            return None;
        }

        let path = url.strip_prefix('/').unwrap_or(url);
        let rest = path.strip_prefix("assets/")?;
        let trimmed = rest.trim_start_matches('/');

        if trimmed.is_empty() {
            return None;
        }

        Some(trimmed.to_string())
    }
}
