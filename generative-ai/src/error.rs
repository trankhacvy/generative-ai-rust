#[derive(Debug, thiserror::Error)]
pub enum GenerativeAIError {
    /// An error occurred while performing an HTTP request.
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
