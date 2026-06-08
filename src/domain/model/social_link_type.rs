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
    pub fn from_url(url: &str) -> Self {
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

pub fn get_hostname(url: &str) -> String {
    let without_scheme = url.split_once("://").map(|(_, rest)| rest).unwrap_or(url);
    let host_and_port = without_scheme
        .split_once('/')
        .map(|(host, _)| host)
        .unwrap_or(without_scheme);
    let host = host_and_port
        .split_once(':')
        .map(|(host, _)| host)
        .unwrap_or(host_and_port);
    host.strip_prefix("www.").unwrap_or(host).to_string()
}
