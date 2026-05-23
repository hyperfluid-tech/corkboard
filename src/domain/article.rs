#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub content_html: String,
}

#[derive(serde::Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
}
