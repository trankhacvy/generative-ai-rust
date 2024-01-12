use std::fmt::Debug;

use reqwest::Response;
use serde::Serialize;

use crate::{enum_types::Task, GenerativeAIError};

const BASE_URL: &str = "https://generativelanguage.googleapis.com";

const API_VERSION: &str = "v1";

const PACKAGE_VERSION: &str = "__PACKAGE_VERSION__";
const PACKAGE_LOG_HEADER: &str = "genai-rs";

const _RESPONSE_LINE_RE: &str = r"^data: (.*)(?:\n\n|\r\r|\r\n\r\n)";

fn get_client_headers() -> String {
    format!("{}/{}", PACKAGE_LOG_HEADER, PACKAGE_VERSION)
}

pub fn make_url(model: &str, task: Task, stream: bool) -> String {
    let mut url = format!("{}/{}/models/{}:{}", BASE_URL, API_VERSION, model, task);
    if stream {
        url += "?alt=sse";
    }

    url
}

pub async fn make_request<T: Serialize + ?Sized + Debug>(
    url: &str,
    api_key: &str,
    body: &T,
) -> Result<Response, GenerativeAIError> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "x-goog-api-client",
        reqwest::header::HeaderValue::from_str(&get_client_headers()).unwrap(),
    );
    headers.insert(
        "x-goog-api-key",
        reqwest::header::HeaderValue::from_str(&api_key).unwrap(),
    );
    println!("{:#?}", body);
    let response = client.post(url).headers(headers).json(body).send().await?;

    Ok(response)
}
