use std::sync::Arc;
use crate::domain::article::Article;
use crate::infrastructure::config::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub articles: Arc<Vec<Article>>,
}
