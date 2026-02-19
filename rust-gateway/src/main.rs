mod handler;
mod service;
mod state;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tower_http::cors::CorsLayer;

// Modules we just created
use crate::handler::get_rest_stats;
use crate::service::GatewayService;
use crate::state::AppState;

// Proto generation
pub mod gateway {
    tonic::include_proto!("gateway");
}
use crate::gateway::llm_service_server::LlmServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Shared State (Atomic & Thread-Safe)
    let shared_state = Arc::new(AppState::new());

    // 2. gRPC Infrastructure (For Python Agent)
    let grpc_addr: SocketAddr = "[::1]:50051".parse()?;
    let grpc_service = GatewayService {
        state: Arc::clone(&shared_state),
    };

    let grpc_handle = tokio::spawn(async move {
        println!("🚀 gRPC Server listening on {}", grpc_addr);
        Server::builder()
            .add_service(LlmServiceServer::new(grpc_service))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    // 3. REST Infrastructure (For Vite Frontend)
    let rest_addr: SocketAddr = "127.0.0.1:3000".parse()?;
    let rest_app = Router::new()
        .route("/stats", get(get_rest_stats))
        .with_state(Arc::clone(&shared_state))
        .layer(CorsLayer::permissive());

    let rest_handle = tokio::spawn(async move {
        println!("🌐 REST API listening on http://{}", rest_addr);
        let listener = tokio::net::TcpListener::bind(rest_addr).await.unwrap();
        axum::serve(listener, rest_app).await.unwrap();
    });

    // 4. Run concurrent servers
    let _ = tokio::try_join!(grpc_handle, rest_handle);

    Ok(())
}
