use crate::state::AppState;
use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub async fn get_rest_stats(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "Status": "Running",
        "active_models": ["GPT-4", "Claude-3", "Llama-3"],
        "total_requests_processed": state.total_requests.load(Ordering::Relaxed),
        "uptime": "stable"
    }))
}

pub async fn get_leader_board_state(State(state): State<Arc<AppState>>) -> Json<Value> {
    let results: Vec<(String, i64)> = sqlx::query_as(
        "SELECT winner, COUNT(*) as win_count FROM request_logs GROUP BY winner ORDER BY win_count DESC",
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let leaderboard: Vec<Value> = results
        .into_iter()
        .map(|(winner, win_count)| json!({ "winner": winner, "win_count": win_count }))
        .collect();

    Json(json!({ "leaderboard": leaderboard }))
}

pub async fn get_recent_logs(State(state): State<Arc<AppState>>) -> Json<Value> {
    let logs: Vec<Value> = sqlx::query!(
        "SELECT prompt, winner, response_text, duration_ms FROM request_logs ORDER BY created_at DESC LIMIT 5"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|row| json!({
        "prompt": row.prompt,
        "winner": row.winner,
        "response": row.response_text,
        "latency": row.duration_ms
    }))
    .collect();

    Json(json!({ "logs": logs }))
}

// Inside handler.rs
pub async fn handle_test_query(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let prompt = payload["prompt"].as_str().unwrap_or("Hello");
    let mode = payload["mode"].as_str().unwrap_or("race");

    // We manually call our own internal service logic here
    // In a real app, this would be the gRPC call from Python
    let start = std::time::Instant::now();

    // Simulate a race (For demo purposes, just call one or use your internal logic)
    let response = "This is a simulated response from the gateway engine.";
    let duration = start.elapsed().as_millis() as u64;

    Json(json!({
        "winner": "OpenAI (Simulated)",
        "response": response,
        "latency": duration
    }))
}
