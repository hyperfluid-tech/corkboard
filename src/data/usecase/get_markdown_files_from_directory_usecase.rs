use super::parse_markdown_file_usecase::ParseMarkdownFileUsecase;
use crate::data::model::markdown_document::MarkdownDocument;
use std::fs;

pub struct GetMarkdownFilesFromDirectoryUsecase;

impl GetMarkdownFilesFromDirectoryUsecase {
    pub fn execute(dir: &str) -> Result<Vec<MarkdownDocument>, Box<dyn std::error::Error>> {
        let mut documents = Vec::new();
        let paths = fs::read_dir(dir)?;

        for path in paths {
            let path = path?.path();

            if !path.is_file() || path.extension().is_none_or(|ext| ext != "md") {
                continue;
            }

            let content = fs::read_to_string(&path)?;
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if let Some(document) = ParseMarkdownFileUsecase::execute(file_name, &content)? {
                documents.push(document);
            }
        }

        Ok(documents)
    }
}
