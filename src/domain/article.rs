#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub content_html: String,
    pub subheading: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub subheading: Option<String>,
}
