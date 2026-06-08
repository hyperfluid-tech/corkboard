use askama::Template;

#[derive(Template)]
#[template(path = "components/markdown_image.html")]
pub struct MarkdownImageTemplate<'a> {
    pub src: &'a str,
    pub alt: &'a str,
    pub caption: &'a str,
}
