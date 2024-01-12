use serde_json::json;

use crate::{
    enum_types::Task,
    helper::request::{make_request, make_url},
    requests::{
        BatchEmbedContentRequest, CountTokensRequest, EmbedContentRequest,
        EmbedContentWithModelRequest, GenerateContentRequest, GenerationConfig, ModelConfig,
        SafetySetting,
    },
    responses::{
        BatchEmbedContentResponse, CountTokensResponse, EmbedContentResponse, GenerateContentResult,
    },
    GenerativeAIError,
};

pub struct GenerativeModel {
    api_key: String,
    model: String,
    generation_config: Option<GenerationConfig>,
    safety_settings: Option<Vec<SafetySetting>>,
}

impl GenerativeModel {
    pub fn new(api_key: &str, model: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: model.to_string(),
            generation_config: None,
            safety_settings: None,
        }
    }

    pub fn with_config(&mut self, config: ModelConfig) -> &Self {
        self.generation_config = config.generation_config;
        self.safety_settings = config.safety_settings;
        self
    }

    pub async fn generate_content<T>(
        &self,
        request: T,
    ) -> Result<GenerateContentResult, GenerativeAIError>
    where
        T: Into<GenerateContentRequest>,
    {
        let request: GenerateContentRequest = request.into();

        let request = json!({
           "contents": request.contents,
           "safety_settings": if request.safety_settings.is_some() { &request.safety_settings } else { &self.safety_settings },
           "generation_config": if request.generation_config.is_some() { &request.generation_config } else { &self.generation_config },
        });

        let url = make_url(&self.model, Task::GenerateContent, false);

        let response = make_request(&url, &self.api_key, &request).await?;

        let result = response
            .json::<GenerateContentResult>()
            .await
            .map_err(|e| GenerativeAIError::Reqwest(e));

        result
    }

    pub async fn count_tokens<T>(
        &self,
        request: T,
    ) -> Result<CountTokensResponse, GenerativeAIError>
    where
        T: Into<CountTokensRequest>,
    {
        let request = request.into();

        let url = make_url(&self.model, Task::CountTokens, false);

        let response = make_request(&url, &self.api_key, &request).await?;

        let result = response
            .json::<CountTokensResponse>()
            .await
            .map_err(|e| GenerativeAIError::Reqwest(e));

        result
    }

    pub async fn embed_content<T>(
        &self,
        request: T,
    ) -> Result<EmbedContentResponse, GenerativeAIError>
    where
        T: Into<EmbedContentRequest>,
    {
        let request = request.into();

        let url = make_url(&self.model, Task::EmbedContent, false);

        let response = make_request(&url, &self.api_key, &request).await?;

        let result = response
            .json::<EmbedContentResponse>()
            .await
            .map_err(|e| GenerativeAIError::Reqwest(e));

        result
    }

    pub async fn batch_embed_contents(
        &self,
        request: BatchEmbedContentRequest,
    ) -> Result<BatchEmbedContentResponse, GenerativeAIError> {
        let url = make_url(&self.model, Task::BatchEmbedContents, false);

        let requests = request
            .requests
            .into_iter()
            .map(|req| EmbedContentWithModelRequest {
                model: format!("models/${}", self.model),
                content: req.content,
                task_type: req.task_type,
                title: req.title,
            })
            .collect::<Vec<EmbedContentWithModelRequest>>();

        let requests = json!({ "requests": serde_json::to_value(&requests).unwrap() });

        let response = make_request(&url, &self.api_key, &requests).await?;

        let result = response
            .json::<BatchEmbedContentResponse>()
            .await
            .map_err(|e| GenerativeAIError::Reqwest(e));

        result
    }
}
