use crate::domain::article::Article;
use crate::infrastructure::config::Settings;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub articles: Arc<Vec<Article>>,
}
