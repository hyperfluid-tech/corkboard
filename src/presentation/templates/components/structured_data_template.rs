use crate::presentation::model::structured_data::StructuredData;
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "components/structured_data.html")]
pub struct StructuredDataTemplate {
    pub payload: String,
}

impl StructuredDataTemplate {
    pub fn new(data: &StructuredData) -> Self {
        let payload = serde_json::to_string(data).unwrap_or_default();
        Self { payload }
    }
}
