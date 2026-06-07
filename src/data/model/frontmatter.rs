#[derive(serde::Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub heading: Option<String>,
    pub date: String,
    #[serde(default)]
    pub subheading: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
}
