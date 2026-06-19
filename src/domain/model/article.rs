use std::collections::HashSet;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub content: String,
    pub preview: String,
    pub has_more_content: bool,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub toc: Vec<super::toc_entry::TocEntry>,
    #[serde(default)]
    pub referenced_assets: HashSet<String>,
}
