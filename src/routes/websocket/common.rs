use tokio::time::Duration;

/// Refresh interval for WebSocket data updates (5 seconds)
pub const REFRESH_INTERVAL: Duration = Duration::from_secs(5);