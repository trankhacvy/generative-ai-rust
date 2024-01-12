use serde::{Deserialize, Serialize};

use crate::{Content, HarmBlockThreshold, HarmCategory, InputContent, Part, TaskType};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelParams {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ModelConfig {
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
}

impl GenerateContentRequest {
    pub fn from_contents(contents: Vec<Content>) -> Self {
        GenerateContentRequest {
            contents,
            safety_settings: None,
            generation_config: None,
        }
    }
}

impl From<&str> for GenerateContentRequest {
    fn from(value: &str) -> Self {
        let content = Content {
            parts: vec![Part::from(value)],
            role: String::from("user"),
        };

        GenerateContentRequest::from_contents(vec![content])
    }
}

impl From<Vec<&str>> for GenerateContentRequest {
    fn from(value: Vec<&str>) -> Self {
        let parts: Vec<Part> = value.into_iter().map(|value| Part::from(value)).collect();

        let content = Content {
            parts,
            role: String::from("user"),
        };

        GenerateContentRequest::from_contents(vec![content])
    }
}

impl From<Vec<Part>> for GenerateContentRequest {
    fn from(value: Vec<Part>) -> Self {
        let content = Content {
            parts: value,
            role: String::from("user"),
        };

        GenerateContentRequest::from_contents(vec![content])
    }
}

impl From<Part> for GenerateContentRequest {
    fn from(value: Part) -> Self {
        let content = Content {
            parts: vec![Part::from(value)],
            role: String::from("user"),
        };

        GenerateContentRequest::from_contents(vec![content])
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetySetting {
    category: HarmCategory,
    threshold: HarmBlockThreshold,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartChatParams<T: Into<Part>> {
    #[serde(skip_serializing_if = "Option::is_none")]
    history: Option<Vec<InputContent<T>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountTokensRequest {
    contents: Vec<Content>,
}

impl From<&str> for CountTokensRequest {
    fn from(value: &str) -> Self {
        let content = Content {
            parts: vec![Part::from(value)],
            role: String::from("user"),
        };

        CountTokensRequest {
            contents: vec![content],
        }
    }
}

impl From<Vec<&str>> for CountTokensRequest {
    fn from(value: Vec<&str>) -> Self {
        let parts: Vec<Part> = value.into_iter().map(|value| Part::from(value)).collect();

        let content = Content {
            parts,
            role: String::from("user"),
        };

        CountTokensRequest {
            contents: vec![content],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedContentRequest {
    pub content: Content,
    pub task_type: Option<TaskType>,
    pub title: Option<String>,
}

impl From<&str> for EmbedContentRequest {
    fn from(value: &str) -> Self {
        let content = Content {
            parts: vec![Part::from(value)],
            role: String::from("user"),
        };

        EmbedContentRequest {
            content,
            task_type: None,
            title: None,
        }
    }
}

impl From<Vec<&str>> for EmbedContentRequest {
    fn from(value: Vec<&str>) -> Self {
        let parts: Vec<Part> = value.into_iter().map(|value| Part::from(value)).collect();

        let content = Content {
            parts,
            role: String::from("user"),
        };

        EmbedContentRequest {
            content,
            task_type: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedContentWithModelRequest {
    pub content: Content,
    pub task_type: Option<TaskType>,
    pub title: Option<String>,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchEmbedContentRequest {
    pub requests: Vec<EmbedContentRequest>,
}
