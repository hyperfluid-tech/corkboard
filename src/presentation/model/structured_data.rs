use crate::domain::model::article::Article;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Person {
    #[serde(rename = "@type")]
    pub r#type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Organization {
    #[serde(rename = "@type")]
    pub r#type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@type")]
pub enum StructuredData {
    WebSite {
        #[serde(rename = "@context")]
        context: String,
        name: String,
        url: String,
        author: Person,
    },
    BlogPosting {
        #[serde(rename = "@context")]
        context: String,
        headline: String,
        url: String,
        #[serde(rename = "datePublished")]
        date_published: chrono::NaiveDate,
        author: Person,
        publisher: Organization,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        image: Option<String>,
    },
}

impl StructuredData {
    pub fn website(name: String, url: String, author_name: String) -> Self {
        Self::WebSite {
            context: "https://schema.org".to_string(),
            name,
            url,
            author: Person {
                r#type: "Person".to_string(),
                name: author_name,
            },
        }
    }

    pub fn blog_posting(
        article: &Article,
        base_url: &str,
        author_name: String,
        publisher_name: String,
    ) -> Self {
        Self::BlogPosting {
            context: "https://schema.org".to_string(),
            headline: article.title.clone(),
            url: format!("{}/article/{}", base_url, article.slug),
            date_published: article.date,
            author: Person {
                r#type: "Person".to_string(),
                name: author_name,
            },
            publisher: Organization {
                r#type: "Organization".to_string(),
                name: publisher_name,
            },
            description: article.description.clone(),
            image: article.thumbnail.clone(),
        }
    }
}
