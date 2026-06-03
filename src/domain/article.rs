#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub content: String,
    pub preview: String,
    pub has_more_content: bool,
    pub subheading: Option<String>,
    pub thumbnail: Option<String>,
    pub toc: Vec<super::toc_entry::TocEntry>,
}

#[derive(serde::Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub subheading: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
}
