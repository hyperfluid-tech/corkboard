use url::Url;

const GITHUB_HOST: &str = "github.com";
const LINKEDIN_HOST: &str = "linkedin.com";
const TWITTER_HOSTS: &[&str] = &["twitter.com", "x.com"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocialLinkType {
    GitHub,
    LinkedIn,
    Twitter,
    Generic,
}

impl SocialLinkType {
    pub fn from_url(url: &Url) -> Self {
        let hostname = get_hostname(url);
        if is_matching_host(&hostname, GITHUB_HOST) {
            return Self::GitHub;
        }
        if is_matching_host(&hostname, LINKEDIN_HOST) {
            return Self::LinkedIn;
        }
        if TWITTER_HOSTS
            .iter()
            .any(|&host| is_matching_host(&hostname, host))
        {
            return Self::Twitter;
        }
        Self::Generic
    }
}

fn is_matching_host(hostname: &str, expected: &str) -> bool {
    if hostname == expected {
        return true;
    }
    if let Some(sub) = hostname.strip_suffix(expected) {
        return sub.ends_with('.');
    }
    false
}

pub fn get_hostname(url: &Url) -> String {
    if let Some(host) = url.host_str() {
        return host.strip_prefix("www.").unwrap_or(host).to_string();
    }
    url.to_string()
}
