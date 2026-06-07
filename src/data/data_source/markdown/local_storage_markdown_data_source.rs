use super::markdown_data_source::MarkdownDataSource;
use crate::data::model::frontmatter::FrontMatter;
use crate::data::model::markdown_document::MarkdownDocument;
use std::fs;

pub struct LocalStorageMarkdownDataSource {
    dir: String,
}

impl LocalStorageMarkdownDataSource {
    pub fn new(dir: String) -> Self {
        Self { dir }
    }
}

impl MarkdownDataSource for LocalStorageMarkdownDataSource {
    fn fetch_all(&self) -> Result<Vec<MarkdownDocument>, Box<dyn std::error::Error>> {
        let mut documents = Vec::new();
        let paths = fs::read_dir(&self.dir)?;

        for path in paths {
            let path = path?.path();

            if !path.is_file() || path.extension().map_or(true, |ext| ext != "md") {
                continue;
            }

            let content = fs::read_to_string(&path)?;
            if !content.starts_with("---") {
                continue;
            }

            let parts: Vec<&str> = content.splitn(3, "---").collect();
            if parts.len() < 3 {
                continue;
            }

            let yaml_str = parts[1];
            let body = parts[2].to_string();

            let frontmatter: FrontMatter = serde_yaml::from_str(yaml_str)?;
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            documents.push(MarkdownDocument {
                frontmatter,
                body,
                file_name,
            });
        }

        Ok(documents)
    }
}
