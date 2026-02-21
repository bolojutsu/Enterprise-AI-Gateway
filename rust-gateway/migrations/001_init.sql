CREATE TABLE IF NOT EXISTS request_logs (
    id TEXT PRIMARY KEY,
    prompt TEXT NOT NULL,
    winner TEXT NOT NULL,
    response_text TEXT NOT NULL,
    duration_ms INTEGER NOT NULL,
    mode TEXT NOT NULL,
    -- 'race' or 'research'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);