use crate::clients::ClientError;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: Message,
}

pub async fn call_grok(prompt: &str, model: &str) -> Result<String, ClientError> {
    let api_key = std::env::var("GROK_API_KEY")
        .map_err(|_| ClientError::ApiError("GROK_API_KEY not set".to_string()))?;

    let client = reqwest::Client::new();

    let request_body = ChatRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    let response = client
        .post("https://api.x.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
        .map_err(ClientError::Network)?;

    if !response.status().is_success() {
        let err_text = response.text().await.unwrap_or_default();
        return Err(ClientError::ApiError(err_text));
    }

    let chat_res: ChatResponse = response.json().await.map_err(ClientError::Network)?;

    chat_res
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .ok_or_else(|| ClientError::ApiError("Empty response from OpenAI".to_string()))
}
