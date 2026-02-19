use std::sync::atomic::AtomicU64;

pub struct AppState {
    pub total_requests: AtomicU64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
        }
    }
}
