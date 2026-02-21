use crate::clients::ClientError;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: GeminiContent,
}

pub async fn call_gemini(prompt: &str, model: &str) -> Result<String, ClientError> {
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| ClientError::ApiError("GEMINI_API_KEY not set".to_string()))?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );
    let client = reqwest::Client::new();
    let body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart {
                text: prompt.to_string(),
            }],
        }],
    };

    let res = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(ClientError::Network)?
        .json::<GeminiResponse>()
        .await
        .map_err(ClientError::Network)?;

    res.candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.clone())
        .ok_or(ClientError::ApiError("Empty".into()))
}
