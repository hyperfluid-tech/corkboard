use crate::data::model::frontmatter::FrontMatter;
use crate::data::model::markdown_document::MarkdownDocument;

pub struct ParseMarkdownFileUsecase;

impl ParseMarkdownFileUsecase {
    pub fn execute(
        file_name: String,
        content: &str,
    ) -> Result<Option<MarkdownDocument>, Box<dyn std::error::Error>> {
        if !content.starts_with("---") {
            return Ok(None);
        }

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Ok(None);
        }

        let yaml_str = parts[1];
        let body = parts[2].to_string();

        let frontmatter: FrontMatter = serde_yaml::from_str(yaml_str)?;

        Ok(Some(MarkdownDocument {
            frontmatter,
            body,
            file_name,
        }))
    }
}
