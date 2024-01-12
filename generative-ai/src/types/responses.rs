use serde::{Deserialize, Serialize};

use crate::{BlockReason, Content, FinishReason, HarmCategory, HarmProbability};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<GenerateContentCandidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_feedback: Option<PromptFeedback>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentCandidate {
    pub index: i32,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_message: Option<String>,
    pub safety_ratings: Vec<SafetyRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_metadata: Option<CitationMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_reason: Option<BlockReason>,
    safety_ratings: Vec<SafetyRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_reason_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    citation_sources: Vec<CitationSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentEmbedding {
    pub values: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedContentResponse {
    pub embedding: ContentEmbedding,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchEmbedContentResponse {
    pub embeddings: Vec<ContentEmbedding>,
}
