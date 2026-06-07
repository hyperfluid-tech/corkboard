use crate::domain::model::article::Article;

pub trait ArticleRepository {
    fn load_all(&self) -> Result<Vec<Article>, Box<dyn std::error::Error>>;
}
