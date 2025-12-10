use crate::client::FetchClient;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tokio::time::timeout;
use tracing::debug;

/// Simple circuit breaker to protect the external logo service.
#[derive(Debug)]
struct CircuitBreaker {
    failure_threshold: u32,
    timeout_duration: Duration,
    failure_count: u32,
    last_failure_time: Option<Instant>,
    state: CircuitState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    fn new(failure_threshold: u32, timeout_duration: Duration) -> Self {
        Self {
            failure_threshold,
            timeout_duration,
            failure_count: 0,
            last_failure_time: None,
            state: CircuitState::Closed,
        }
    }

    fn allow(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last) = self.last_failure_time {
                    if last.elapsed() > self.timeout_duration {
                        self.state = CircuitState::HalfOpen;
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    fn record_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitState::Closed;
        self.last_failure_time = None;
    }

    fn record_failure(&mut self) {
        self.failure_count = self.failure_count.saturating_add(1);
        self.last_failure_time = Some(Instant::now());
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }
}

#[derive(Debug, Clone)]
struct CacheEntry {
    value: String,
    expires_at: Instant,
}

/// Fetches company logos using logo.dev API with caching and a circuit breaker.
/// 
/// This fetcher uses logo.dev's API (https://logo.dev) which provides company logos
/// by ticker symbol. It constructs URLs like:
/// `https://img.logo.dev/ticker/AAPL?token=...&format=png&fallback=404&size=50&theme=dark`
/// 
/// The fetcher includes:
/// 1. In-memory caching with TTL
/// 2. Circuit breaker pattern to protect against service failures
/// 3. Configurable timeouts
#[derive(Debug, Clone)]
pub struct LogoFetcher {
    fetch_client: Arc<FetchClient>,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    circuit_breaker: Arc<Mutex<CircuitBreaker>>,
    timeout: Duration,
    cache_ttl: Duration,
    enabled: bool,
}

impl LogoFetcher {
    /// Create a new logo fetcher using the provided fetch client.
    ///
    /// Environment variables:
    /// - `DISABLE_LOGO_FETCHING`: when "true", skip all logo requests.
    /// - `LOGO_TIMEOUT_SECONDS`: per-request timeout (default 2s).
    /// - `LOGO_CIRCUIT_BREAKER_THRESHOLD`: failures before opening (default 5).
    /// - `LOGO_CIRCUIT_BREAKER_TIMEOUT`: cooldown in seconds (default 300s).
    pub fn new(fetch_client: Arc<FetchClient>) -> Self {
        let enabled = env::var("DISABLE_LOGO_FETCHING")
            .map(|v| v.to_lowercase() != "true")
            .unwrap_or(true);

        let timeout_secs = env::var("LOGO_TIMEOUT_SECONDS")
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(2.0)
            .max(0.1);

        let breaker_threshold = env::var("LOGO_CIRCUIT_BREAKER_THRESHOLD")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(5)
            .max(1);

        let breaker_timeout = env::var("LOGO_CIRCUIT_BREAKER_TIMEOUT")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(300);

        Self {
            fetch_client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            circuit_breaker: Arc::new(Mutex::new(CircuitBreaker::new(
                breaker_threshold,
                Duration::from_secs(breaker_timeout),
            ))),
            timeout: Duration::from_secs_f64(timeout_secs),
            cache_ttl: Duration::from_secs(60 * 60 * 24),
            enabled,
        }
    }

    /// Returns whether logo fetching is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Fetch a logo for the given ticker symbol using logo.dev API.
    /// 
    /// This uses the ticker symbol directly to fetch the logo from logo.dev.
    /// The website parameter is ignored but kept for backward compatibility.
    pub async fn fetch_logo(&self, symbol: &str, _website: Option<&str>) -> Option<String> {
        if !self.enabled {
            return None;
        }

        if symbol.is_empty() {
            return None;
        }

        if !self.allow_request().await {
            debug!("Logo circuit breaker is open; skipping fetch");
            return None;
        }

        let cache_key = format!("logo:{}", symbol.to_uppercase());
        
        if let Some(cached) = self.get_cached(&cache_key).await {
            return Some(cached);
        }

        // Construct logo.dev URL with hardcoded token
        let logo_url = format!(
            "https://img.logo.dev/ticker/{}?token=pk_NNp9abu9TMm9II6Z0666YA&format=png&fallback=404&size=50&theme=dark",
            symbol.to_uppercase()
        );

        match self.try_fetch_logo(&logo_url).await {
            Ok(url) => {
                self.cache_value(&cache_key, url.clone()).await;
                self.record_success().await;
                Some(url)
            }
            Err(err) => {
                debug!("Logo fetch failed for {}: {}", symbol, err);
                self.record_failure().await;
                None
            }
        }
    }

    async fn get_cached(&self, key: &str) -> Option<String> {
        let now = Instant::now();
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > now {
                return Some(entry.value.clone());
            }
        }
        cache.remove(key);
        None
    }

    async fn cache_value(&self, key: &str, value: String) {
        let expires_at = Instant::now() + self.cache_ttl;
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), CacheEntry { value, expires_at });
    }

    async fn allow_request(&self) -> bool {
        let mut breaker = self.circuit_breaker.lock().await;
        breaker.allow()
    }

    async fn record_success(&self) {
        let mut breaker = self.circuit_breaker.lock().await;
        breaker.record_success();
    }

    async fn record_failure(&self) {
        let mut breaker = self.circuit_breaker.lock().await;
        breaker.record_failure();
    }

    async fn try_fetch_logo(&self, url: &str) -> Result<String, String> {
        let fetch = self.fetch_client.fetch_response(url);
        match timeout(self.timeout, fetch).await {
            Ok(Ok(response)) => {
                let status = response.status();
                if status.is_success() {
                    Ok(response.url().to_string())
                } else {
                    Err(format!("HTTP {}", status))
                }
            }
            Ok(Err(err)) => Err(err.to_string()),
            Err(_) => Err(format!("Timed out after {:?}", self.timeout)),
        }
    }
}
