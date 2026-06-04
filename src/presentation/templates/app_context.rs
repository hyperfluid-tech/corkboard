#[derive(Debug, Clone)]
pub struct AppContext {
    pub is_dev: bool,
    pub version: String,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            is_dev: cfg!(debug_assertions),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for AppContext {
    fn default() -> Self {
        Self::new()
    }
}
