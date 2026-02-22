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
