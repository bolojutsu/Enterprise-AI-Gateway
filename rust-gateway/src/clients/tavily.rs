use crate::clients::ClientError;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SearchRequest {
    pub api_key: String,
    pub query: String,
    pub search_depth: String,
}

#[derive(Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

pub async fn call_tavily(query: &str) -> Result<String, ClientError> {
    let api_key = std::env::var("TAVILY_API_KEY")
        .map_err(|_| ClientError::ApiError("TAVILY_API_KEY not set".into()))?;

    let client = reqwest::Client::new();
    let body = SearchRequest {
        api_key,
        query: query.into(),
        search_depth: "basic".into(),
    };

    let res = client
        .post("https://api.tavily.com/search")
        .json(&body)
        .send()
        .await
        .map_err(ClientError::Network)?
        .json::<SearchResponse>()
        .await
        .map_err(ClientError::Network)?;

    let summary = res
        .results
        .iter()
        .take(3)
        .map(|r| format!("{}: {}", r.title, r.content))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(summary)
}
