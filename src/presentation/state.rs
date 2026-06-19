use crate::domain::model::article::Article;
use crate::infrastructure::config::Settings;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub articles: Arc<Vec<Article>>,
    pub allowed_assets: Arc<HashSet<String>>,
}
