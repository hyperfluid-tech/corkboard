use pulldown_cmark::{Event, Options, Parser, Tag};
use std::collections::HashSet;

pub struct ExtractAssetReferencesUsecase;

impl ExtractAssetReferencesUsecase {
    pub fn execute(markdown_body: &str, thumbnail: Option<&str>) -> HashSet<String> {
        let mut asset_paths = HashSet::new();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        let parser = Parser::new_ext(markdown_body, options);

        for event in parser {
            if let Event::Start(Tag::Image { dest_url, .. }) = event {
                if let Some(normalized) = Self::normalize_asset_path(&dest_url) {
                    asset_paths.insert(normalized);
                }
            }
        }

        if let Some(thumb) = thumbnail {
            if let Some(normalized) = Self::normalize_asset_path(thumb) {
                asset_paths.insert(normalized);
            }
        }

        asset_paths
    }

    fn normalize_asset_path(url: &str) -> Option<String> {
        if url.starts_with("http://") || url.starts_with("https://") {
            return None;
        }

        let path = url.strip_prefix('/').unwrap_or(url);

        if let Some(rest) = path.strip_prefix("assets/") {
            let trimmed = rest.trim_start_matches('/');
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
        None
    }
}
