use super::social_link_template::SocialLinkTemplate;
use crate::presentation::model::app_context::AppContext;
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "components/footer.html")]
pub struct FooterTemplate {
    pub app: AppContext,
    pub blog_author: String,
    pub blog_license: String,
    pub blog_license_url: String,
    pub current_year: i32,
    pub social_links: Vec<SocialLinkTemplate>,
}

impl FooterTemplate {
    pub fn new(
        app: AppContext,
        blog_author: String,
        blog_license: String,
        blog_license_url: String,
        current_year: i32,
        social_links: &[String],
    ) -> Self {
        let social_links = social_links
            .iter()
            .map(|url| SocialLinkTemplate::new(url.clone()))
            .collect();

        Self {
            app,
            blog_author,
            blog_license,
            blog_license_url,
            current_year,
            social_links,
        }
    }
}
