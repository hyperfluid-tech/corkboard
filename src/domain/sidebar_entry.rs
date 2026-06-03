use super::article::Article;
use super::toc_entry::TocEntry;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SidebarEntry {
    pub title: String,
    pub url: String,
    pub slug: String,
    pub indent_level: u32,
    pub font_class: String,
}

impl SidebarEntry {
    pub fn from_toc_entry(entry: &TocEntry) -> Self {
        let font_class = if entry.level >= 5 {
            "text-xs font-normal text-primary/75 italic"
        } else if entry.level >= 3 {
            "text-sm font-medium text-primary/90"
        } else {
            "font-semibold text-primary"
        };

        Self {
            title: entry.title.clone(),
            url: format!("#{}", entry.slug),
            slug: entry.slug.clone(),
            indent_level: entry.level - 1,
            font_class: font_class.to_string(),
        }
    }

    pub fn from_article(article: &Article) -> Self {
        Self {
            title: article.title.clone(),
            url: format!("#{}", article.slug),
            slug: article.slug.clone(),
            indent_level: 0,
            font_class: "font-semibold text-primary".to_string(),
        }
    }
}
