use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    Text(TextPart),
    InlineData(InlineDataPart),
}

impl From<&str> for Part {
    fn from(text: &str) -> Self {
        Part::Text(TextPart {
            text: text.to_string(),
            inline_data: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextPart {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineDataPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub inline_data: GenerativeContentBlob,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerativeContentBlob {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputContent<T: Into<Part>> {
    parts: T,
    role: String,
}
