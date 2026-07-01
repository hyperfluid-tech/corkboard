pub struct SanitizeMarkdownHtmlUsecase;

impl SanitizeMarkdownHtmlUsecase {
    pub fn execute(html: String) -> String {
        html.replace(" style=\"text-align: left\"", " class=\"text-left\"")
            .replace(" style=\"text-align: center\"", " class=\"text-center\"")
            .replace(" style=\"text-align: right\"", " class=\"text-right\"")
    }
}
