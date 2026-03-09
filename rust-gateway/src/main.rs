mod clients;
mod handler;
mod service;
mod state;

use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

// Internal imports
use crate::handler::{get_leader_board_state, get_recent_logs, get_rest_stats, handle_test_query};
use crate::service::GatewayService;
use crate::state::AppState;

// Proto generation
pub mod gateway {
    tonic::include_proto!("gateway");
}
use crate::gateway::llm_service_server::LlmServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url: String =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://gateway.db".to_string());

    let connection_options: SqliteConnectOptions =
        SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Database connected and migrations applied.");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 1. Shared State (Atomic & Thread-Safe)
    let shared_state = Arc::new(AppState::new(pool));

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
    let rest_app: Router = Router::new()
        .route("/stats", get(get_rest_stats))
        .route("/leaderboard", get(get_leader_board_state))
        .route("/logs", get(get_recent_logs))
        .route("/test-query", post(handle_test_query))
        .with_state(Arc::clone(&shared_state))
        .layer(cors);

    let rest_handle: JoinHandle<()> = tokio::spawn(async move {
        println!("🌐 REST API listening on http://{}", rest_addr);
        let listener = tokio::net::TcpListener::bind(rest_addr).await.unwrap();
        axum::serve(listener, rest_app).await.unwrap();
    });

    // 4. Run concurrent servers
    let _ = tokio::try_join!(grpc_handle, rest_handle);

    Ok(())
}
