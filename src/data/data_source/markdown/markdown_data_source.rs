use crate::data::model::markdown_document::MarkdownDocument;

pub trait MarkdownDataSource {
    fn fetch_all(&self) -> Result<Vec<MarkdownDocument>, Box<dyn std::error::Error>>;
}
