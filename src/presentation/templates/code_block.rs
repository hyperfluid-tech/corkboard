use askama::Template;

#[derive(Template)]
#[template(path = "components/code_block.html")]
pub struct CodeBlockTemplate<'a> {
    pub content: &'a str,
}
