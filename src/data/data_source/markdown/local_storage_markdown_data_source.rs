use super::markdown_data_source::MarkdownDataSource;
use crate::data::model::markdown_document::MarkdownDocument;
use crate::data::usecase::get_markdown_files_from_directory_usecase::GetMarkdownFilesFromDirectoryUsecase;

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
        GetMarkdownFilesFromDirectoryUsecase::execute(&self.dir)
    }
}
