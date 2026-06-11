use crate::domain::model::article::Article;
use crate::domain::repository::article_repository::ArticleRepository;
use std::collections::HashSet;

pub struct ArticleService {
    repos: Vec<Box<dyn ArticleRepository + Send + Sync>>,
}

impl ArticleService {
    pub fn new(repos: Vec<Box<dyn ArticleRepository + Send + Sync>>) -> Self {
        Self { repos }
    }

    pub fn get_all_articles(&self) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
        let mut articles = Vec::new();

        for repo in &self.repos {
            let mut repo_articles = repo.load_all()?;
            articles.append(&mut repo_articles);
        }

        let mut seen_slugs = HashSet::new();
        for article in articles.iter_mut() {
            let base_slug = article.slug.clone();
            let mut counter = 2;
            while seen_slugs.contains(&article.slug) {
                article.slug = format!("{}-{}", base_slug, counter);
                counter += 1;
            }
            seen_slugs.insert(article.slug.clone());
        }

        articles.sort_by_key(|b| std::cmp::Reverse(b.date));

        Ok(articles)
    }
}
