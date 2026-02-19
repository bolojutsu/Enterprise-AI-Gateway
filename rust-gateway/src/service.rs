use crate::gateway::llm_service_server::LlmService;
use crate::gateway::{PromptRequest, PromptResponse};
use crate::state::AppState;
use futures::future::join_all;
use std::sync::{atomic::Ordering, Arc};
use tokio::task;
use tonic::{Request, Response, Status};

pub struct GatewayService {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl LlmService for GatewayService {
    async fn execute_prompt(
        &self,
        request: Request<PromptRequest>,
    ) -> Result<Response<PromptResponse>, Status> {
        // Increment global state
        self.state.total_requests.fetch_add(1, Ordering::Relaxed);

        let req = request.into_inner();

        // Execute Parallel Logic (The "Scatter")
        let results = self.scatter_gather(&req.user_prompt).await;

        Ok(Response::new(PromptResponse {
            text: format!("Processed by {} models", results.len()),
            cost: 0.015,
        }))
    }
}

impl GatewayService {
    async fn scatter_gather(&self, prompt: &str) -> Vec<String> {
        let models = vec!["gpt-4", "claude-3"];
        let mut handles = vec![];

        for m in models {
            let p = prompt.to_string();
            handles.push(task::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                format!("[{}] Done", m)
            }));
        }

        join_all(handles)
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect()
    }
}
