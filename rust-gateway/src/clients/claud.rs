use crate::clients::ClientError;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ClaudeRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<ClaudeMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ClaudeResponse {
    pub content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
pub struct ContentBlock {
    pub text: String,
}

pub async fn call_claude(prompt: &str, model: &str) -> Result<String, ClientError> {
    let api_key = std::env::var("CLAUDE_API_KEY")
        .map_err(|_| ClientError::ApiError("CLAUDE_API_KEY not set".to_string()))?;

    let client = reqwest::Client::new();
    let body = ClaudeRequest {
        model: model.to_string(),
        max_tokens: 1024,
        messages: vec![ClaudeMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };
    let res = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(ClientError::Network)?
        .json::<ClaudeResponse>()
        .await
        .map_err(ClientError::Network)?;

    res.content
        .get(0)
        .map(|c| c.text.clone())
        .ok_or(ClientError::ApiError("Empty".into()))
}
