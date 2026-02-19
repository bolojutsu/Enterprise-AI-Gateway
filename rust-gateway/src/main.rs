use axum::{routing::get, Json, Router};
use futures::future::join_all;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::task;
use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::CorsLayer;

// 1. Include the generated gRPC code from your .proto file
pub mod gateway {
    tonic::include_proto!("gateway"); // Must match the package name in your .proto
}
use gateway::llm_service_server::{LlmService, LlmServiceServer};
use gateway::{PromptRequest, PromptResponse};

// 2. Define the REST response for your Vite Frontend
#[derive(Serialize)]
struct GatewayStats {
    status: String,
    active_models: Vec<String>,
    total_requests_processed: u64,
}

// 3. The gRPC Service Logic (The "Muscle")
#[derive(Default)]
pub struct MyGateway {}

#[tonic::async_trait]
impl LlmService for MyGateway {
    async fn execute_prompt(
        &self,
        request: Request<PromptRequest>,
    ) -> Result<Response<PromptResponse>, Status> {
        let req = request.into_inner();
        println!("Python Agent requested: {}", req.model);

        // --- PARALLEL PROCESSING START ---
        // We spawn 3 parallel tasks to simulate hitting 3 different LLMs
        let models = vec!["gpt-4", "claude-3", "llama-3"];
        let mut handles = vec![];

        for m in models {
            let p = req.user_prompt.clone();
            let handle = task::spawn(async move {
                // Simulate a network delay for each API call
                tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                format!("[{}] finished processing: {}", m, p)
            });
            handles.push(handle);
        }

        // Wait for all "threads" to finish
        let results = join_all(handles).await;
        // --- PARALLEL PROCESSING END ---

        let final_text = format!("Consolidated Response: {:?}", results.len());

        Ok(Response::new(PromptResponse {
            text: final_text,
            cost: 0.015, // Mock cost
        }))
    }
}

// 4. The REST Handler (For Vite Dashboard)
async fn get_stats() -> Json<GatewayStats> {
    Json(GatewayStats {
        status: "Running".to_string(),
        active_models: vec!["GPT-4".into(), "Claude-3".into(), "Llama-3".into()],
        total_requests_processed: 124, // In a real app, use an AtomicCounter
    })
}

// 5. The Main Entry Point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- PORT 1: gRPC for Python ---
    let grpc_addr: SocketAddr = "[::1]:50051".parse()?;
    let grpc_service = MyGateway::default();

    let grpc_thread = tokio::spawn(async move {
        println!("🚀 gRPC Server (Python Bridge) on {}", grpc_addr);
        Server::builder()
            .add_service(LlmServiceServer::new(grpc_service))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    // --- PORT 2: REST for Vite/React ---
    let rest_addr: SocketAddr = "127.0.0.1:3000".parse()?;
    let rest_app = Router::new()
        .route("/stats", get(get_stats))
        .layer(CorsLayer::permissive()); // Crucial for Vite dev mode

    let rest_thread = tokio::spawn(async move {
        println!("🌐 REST API (Vite/React Bridge) on http://{}", rest_addr);
        let listener = tokio::net::TcpListener::bind(rest_addr).await.unwrap();
        axum::serve(listener, rest_app).await.unwrap();
    });

    // Wait for both "servers" to run forever
    let _ = tokio::try_join!(grpc_thread, rest_thread);

    Ok(())
}
