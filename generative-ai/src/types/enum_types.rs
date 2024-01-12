use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    HarmCategoryUnspecified,
    HarmCategoryHateSpeech,
    HarmCategorySexuallyExplicit,
    HarmCategoryHarassment,
    HarmCategoryDangerousContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    HarmBlockThresholdUnspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    HarmProbabilityUnspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockReason {
    BlockedReasonUnspecified,
    Safety,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    FinishReasonUnspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskType {
    TaskTypeUnspecified,
    RetrievalQuery,
    RetrievalDocument,
    SemanticSimilarity,
    Classification,
    Clustering,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Task {
    GenerateContent,
    StreamGenerateContent,
    CountTokens,
    EmbedContent,
    BatchEmbedContents,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Task::GenerateContent => write!(f, "generateContent"),
            Task::StreamGenerateContent => write!(f, "streamGenerateContent"),
            Task::CountTokens => write!(f, "countTokens"),
            Task::EmbedContent => write!(f, "embedContent"),
            Task::BatchEmbedContents => write!(f, "batchEmbedContents"),
        }
    }
}
