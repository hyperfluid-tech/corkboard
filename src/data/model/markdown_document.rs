use super::frontmatter::FrontMatter;

pub struct MarkdownDocument {
    pub frontmatter: FrontMatter,
    pub body: String,
    pub file_name: String,
}
