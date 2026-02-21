use sqlx::SqlitePool;
use std::sync::atomic::AtomicU64;

pub struct AppState {
    pub total_requests: AtomicU64,
    pub db: SqlitePool,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            db: pool,
        }
    }
}
