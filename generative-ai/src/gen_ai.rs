use crate::model::GenerativeModel;

pub struct GoogleGenerativeAI {
    api_key: String,
}

impl GoogleGenerativeAI {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    pub fn get_model(&self, model: &str) -> GenerativeModel {
        GenerativeModel::new(&self.api_key, model)
    }
}
