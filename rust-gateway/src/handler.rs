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
