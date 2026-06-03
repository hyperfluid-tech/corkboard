#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TocEntry {
    pub level: u32,
    pub title: String,
    pub slug: String,
}
