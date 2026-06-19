#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TocEntry {
    pub absolute_level: u32,
    pub title: String,
    pub slug: String,
    pub relative_level: u32,
}
