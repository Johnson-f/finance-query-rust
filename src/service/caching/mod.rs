use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tracing::{error, info, warn};

pub struct CacheService {
    connection: Option<Arc<ConnectionManager>>,
}

impl CacheService {
    pub async fn new(redis_url: Option<String>) -> Self {
        let redis_url = match redis_url {
            Some(url) => url,
            None => {
                warn!("REDIS_URL environment variable not set. Caching will be disabled.");
                return Self { connection: None };
            }
        };

        info!("Attempting to connect to Redis...");
        
        match redis::Client::open(redis_url.as_str()) {
            Ok(client) => {
                // Add a 10-second timeout for remote connections
                match timeout(Duration::from_secs(10), ConnectionManager::new(client)).await {
                    Ok(Ok(connection)) => {
                        info!("✓ Redis connection established successfully");
                        Self {
                            connection: Some(Arc::new(connection)),
                        }
                    }
                    Ok(Err(e)) => {
                        warn!("✗ Failed to connect to Redis: {}. Caching will be disabled.", e);
                        warn!("  Check your network connection and Redis credentials");
                        Self { connection: None }
                    }
                    Err(_) => {
                        warn!("✗ Redis connection timeout (10s). Caching will be disabled.");
                        warn!("  This usually means the Redis server is unreachable");
                        warn!("  Check your firewall, network, or Redis host configuration");
                        Self { connection: None }
                    }
                }
            }
            Err(e) => {
                warn!("✗ Failed to create Redis client: {}. Caching will be disabled.", e);
                warn!("  Check your REDIS_URL format");
                Self { connection: None }
            }
        }
    }

    pub async fn get<T>(&self, key: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(conn) = &self.connection {
            // Clone the connection manager for this operation
            let mut conn = (**conn).clone();
            match conn.get::<_, String>(key).await {
                Ok(value) => {
                    match serde_json::from_str::<T>(&value) {
                        Ok(deserialized) => {
                            tracing::debug!("Cache hit for key: {}", key);
                            Some(deserialized)
                        }
                        Err(e) => {
                            error!("Failed to deserialize cached value for key {}: {}", key, e);
                            None
                        }
                    }
                }
                Err(e) if e.kind() == redis::ErrorKind::TypeError => {
                    // Key doesn't exist - this is normal
                    tracing::debug!("Cache miss for key: {}", key);
                    None
                }
                Err(e) => {
                    warn!("Redis error when getting key {}: {}. Bypassing cache.", key, e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T, ttl_seconds: u64)
    where
        T: Serialize,
    {
        if let Some(conn) = &self.connection {
            // Clone the connection manager for this operation
            let mut conn = (**conn).clone();
            match serde_json::to_string(value) {
                Ok(serialized) => {
                    if let Err(e) = conn.set_ex::<_, _, ()>(key, serialized, ttl_seconds).await {
                        warn!("Failed to set cache for key {}: {}", key, e);
                    } else {
                        tracing::debug!("Cached value for key: {} with TTL: {} seconds", key, ttl_seconds);
                    }
                }
                Err(e) => {
                    error!("Failed to serialize value for caching key {}: {}", key, e);
                }
            }
        }
    }
}

// TTL constants
pub const TTL_EARNINGS_TRANSCRIPT: u64 = 7_776_000; // 90 days
pub const TTL_FINANCIALS: u64 = 7_776_000; // 90 days
pub const TTL_HOLDERS: u64 = 86_400; // 1 day
pub const TTL_NEWS: u64 = 21_600; // 6 hours
pub const TTL_ANALYSTS: u64 = 604_800; // 1 week

// Cache key generation helpers
pub fn earnings_transcript_key(symbol: &str, transcript_type: &str) -> String {
    format!("earnings_transcript:{}:{}", symbol.to_uppercase(), transcript_type)
}

pub fn financials_key(symbol: &str, statement: &str, frequency: &str) -> String {
    format!("financials:{}:{}:{}", symbol.to_uppercase(), statement, frequency)
}

pub fn holders_key(symbol: &str, holder_type: &str) -> String {
    format!("holders:{}:{}", symbol.to_uppercase(), holder_type)
}

pub fn news_key(symbol: Option<&str>) -> String {
    match symbol {
        Some(s) => format!("news:{}", s.to_uppercase()),
        None => "news:general".to_string(),
    }
}

pub fn analysts_key(symbol: &str, analysis_type: &str) -> String {
    format!("analysts:{}:{}", symbol.to_uppercase(), analysis_type)
}