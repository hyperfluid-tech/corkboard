use crate::domain::model::social_link_type::SocialLinkType;
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "components/social_link.html")]
pub struct SocialLinkTemplate {
    pub url: String,
    pub link_type: SocialLinkType,
    pub alt_text: String,
}

impl SocialLinkTemplate {
    pub fn new(url: String) -> Self {
        let link_type = SocialLinkType::from_url(&url);
        let alt_text = match link_type {
            SocialLinkType::GitHub => "GitHub".to_string(),
            SocialLinkType::LinkedIn => "LinkedIn".to_string(),
            SocialLinkType::Twitter => "Twitter".to_string(),
            SocialLinkType::Generic => crate::domain::model::social_link_type::get_hostname(&url),
        };
        Self {
            url,
            link_type,
            alt_text,
        }
    }
}
