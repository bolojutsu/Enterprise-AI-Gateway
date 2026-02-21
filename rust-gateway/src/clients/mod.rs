pub mod claud;
pub mod gemini;
pub mod grok;
pub mod openai;
pub mod tavily;
use tonic::Status;

#[derive(Debug)]
pub enum ClientError {
    Network(reqwest::Error),
    Parsing(serde_json::Error),
    ApiError(String),
}

impl From<ClientError> for tonic::Status {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Network(_) => Status::unavailable("Provider network error"),
            ClientError::ApiError(msg) => Status::internal(msg),
            _ => Status::unknown("An unexpected gateway error occurred"),
        }
    }
}
