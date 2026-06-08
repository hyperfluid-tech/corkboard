use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "components/header.html")]
pub struct HeaderTemplate {
    pub blog_title: String,
    pub blog_author: String,
    pub base_url: String,
    pub lang: String,
    pub is_single_article_page: bool,
}
