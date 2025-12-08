//! Yahoo Finance authentication manager.
//!
//! This module handles cookie/crumb authentication for Yahoo Finance API requests.

use crate::client::error::YahooError;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{cookie::Jar, ClientBuilder};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

const MIN_REFRESH_INTERVAL_SECS: i64 = 30;

/// Auth acquisition strategy. Mirrors yfinance’s dual-path approach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CookieStrategy {
    Basic,
    Csrf,
}

impl Default for CookieStrategy {
    fn default() -> Self {
        CookieStrategy::Basic
    }
}

static CSRF_TOKEN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"name="csrfToken"[^>]*value="([^"]+)""#).unwrap());
static SESSION_ID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"name="sessionId"[^>]*value="([^"]+)""#).unwrap());

struct AuthState {
    crumb: Option<String>,
    cookie_jar: Arc<Jar>,
    last_update: Option<DateTime<Utc>>,
    strategy: CookieStrategy,
}

/// Manages Yahoo Finance authentication (cookies and crumb).
///
/// Yahoo Finance requires a valid crumb token for API requests.
/// This manager handles obtaining and refreshing the crumb automatically.
pub struct YahooAuthManager {
    state: Arc<Mutex<AuthState>>,
    proxy: Option<String>,
}

impl YahooAuthManager {
    /// Create a new YahooAuthManager.
    ///
    /// # Arguments
    /// * `proxy` - Optional proxy URL for authentication requests
    /// * `cookie_jar` - Shared cookie jar for storing session cookies
    pub fn new(proxy: Option<String>, cookie_jar: Arc<Jar>) -> Self {
        Self {
            state: Arc::new(Mutex::new(AuthState {
                crumb: None,
                cookie_jar,
                last_update: None,
                strategy: CookieStrategy::default(),
            })),
            proxy,
        }
    }

    /// Force refresh the authentication credentials.
    pub async fn refresh(&self) -> Result<(), YahooError> {
        info!("Refreshing Yahoo authentication...");
        let state = self.state.lock().await;
        let cookie_jar = state.cookie_jar.clone();
        let mut strategy = state.strategy;
        drop(state);

        let mut builder = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .cookie_provider(cookie_jar.clone())
            .redirect(reqwest::redirect::Policy::limited(10));

        if let Some(proxy_url) = &self.proxy {
            info!(
                "Using proxy for Yahoo auth: {}...",
                &proxy_url.chars().take(30).collect::<String>()
            );
            builder = builder
                .proxy(
                    reqwest::Proxy::all(proxy_url).map_err(YahooError::NetworkError)?,
                )
                // Accept proxy's SSL certificate (Bright Data and similar proxies use self-signed certs)
                .danger_accept_invalid_certs(true);
        }

        let client = builder.build().map_err(YahooError::NetworkError)?;

        // Try current strategy, then flip once if it fails (mirrors yfinance dual-path logic)
        let mut last_err: Option<YahooError> = None;
        for _ in 0..2 {
            let result = match strategy {
                CookieStrategy::Basic => {
                    info!("Attempting Yahoo auth (basic crumb)");
                    Self::fetch_basic(&client).await
                }
                CookieStrategy::Csrf => {
                    info!("Attempting Yahoo auth (consent/CSRF crumb)");
                    Self::fetch_csrf(&client).await
                }
            };

            match result {
                Ok(crumb) => {
                    info!(
                        "Successfully obtained crumb via {:?} (length: {})",
                        strategy,
                        crumb.len()
                    );
                    let mut state = self.state.lock().await;
                    state.crumb = Some(crumb);
                    state.last_update = Some(Utc::now());
                    state.strategy = strategy;
                    return Ok(());
                }
                Err(err) => {
                    warn!(
                        "Auth attempt with {:?} failed: {}. Switching strategy.",
                        strategy, err
                    );
                    last_err = Some(err);
                    strategy = match strategy {
                        CookieStrategy::Basic => CookieStrategy::Csrf,
                        CookieStrategy::Csrf => CookieStrategy::Basic,
                    };
                }
            }
        }

        Err(last_err.unwrap_or_else(|| {
            YahooError::AuthFailed("Failed to obtain Yahoo crumb with all strategies".to_string())
        }))
    }

    /// Get the current crumb, refreshing if necessary.
    ///
    /// Returns a tuple of (cookie_jar, crumb) for use in API requests.
    pub async fn get_or_refresh(&self) -> Result<(Arc<Jar>, String), YahooError> {
        let mut state = self.state.lock().await;

        let needs_refresh = state.crumb.is_none()
            || state.last_update.is_none()
            || (Utc::now() - state.last_update.unwrap()).num_seconds() > MIN_REFRESH_INTERVAL_SECS;

        if needs_refresh {
            debug!(
                "Auth refresh needed. Crumb exists: {}, Last update: {:?}",
                state.crumb.is_some(),
                state.last_update
            );
            drop(state); // Release lock before async operation
            self.refresh().await?;
            state = self.state.lock().await;
        } else {
            debug!(
                "Using cached crumb (age: {}s)",
                state
                    .last_update
                    .map(|t| (Utc::now() - t).num_seconds())
                    .unwrap_or(0)
            );
        }

        let crumb = state.crumb.clone().ok_or_else(|| {
            error!("No crumb available after refresh attempt");
            YahooError::AuthFailed("No crumb available".to_string())
        })?;

        debug!("Returning crumb (length: {})", crumb.len());
        Ok((state.cookie_jar.clone(), crumb))
    }

    /// Get the current crumb without refreshing.
    pub async fn crumb(&self) -> Option<String> {
        self.state.lock().await.crumb.clone()
    }

    /// Force a strategy flip and refresh credentials. Used when Yahoo responds with generic 4xx indicating
    /// a potentially bad crumb, mirroring yfinance's "retry with other cookie strategy" behavior.
    pub async fn switch_strategy_and_refresh(&self) -> Result<(), YahooError> {
        {
            let mut state = self.state.lock().await;
            state.strategy = match state.strategy {
                CookieStrategy::Basic => CookieStrategy::Csrf,
                CookieStrategy::Csrf => CookieStrategy::Basic,
            };
            state.crumb = None;
            state.last_update = None;
            info!("Switched Yahoo auth strategy to {:?}", state.strategy);
        }
        self.refresh().await
    }

    /// Basic path: hit fc.yahoo.com then query1 getcrumb.
    async fn fetch_basic(client: &reqwest::Client) -> Result<String, YahooError> {
        client
            .get("https://fc.yahoo.com")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header("Accept", "*/*")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let crumb_response = client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header("Accept", "*/*")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let crumb_text = crumb_response
            .text()
            .await
            .map_err(YahooError::NetworkError)?
            .trim()
            .to_string();

        if Self::is_valid_crumb(&crumb_text) {
            Ok(crumb_text)
        } else {
            Err(YahooError::AuthFailed(
                "Invalid crumb from query1 endpoint".to_string(),
            ))
        }
    }

    /// Consent/CSRF path: visit consent page, post consent, then query2 getcrumb.
    async fn fetch_csrf(client: &reqwest::Client) -> Result<String, YahooError> {
        let consent_html = client
            .get("https://guce.yahoo.com/consent")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header("Accept", "text/html,*/*;q=0.8")
            .send()
            .await
            .map_err(YahooError::NetworkError)?
            .text()
            .await
            .map_err(YahooError::NetworkError)?;

        let csrf_token = CSRF_TOKEN_RE
            .captures(&consent_html)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string());
        let session_id = SESSION_ID_RE
            .captures(&consent_html)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string());

        let (csrf_token, session_id) = match (csrf_token, session_id) {
            (Some(c), Some(s)) => (c, s),
            _ => {
                return Err(YahooError::AuthFailed(
                    "Failed to extract CSRF token or session id from consent page".to_string(),
                ))
            }
        };

        let data = [
            ("agree", "agree"),
            ("consentUUID", "default"),
            ("sessionId", session_id.as_str()),
            ("csrfToken", csrf_token.as_str()),
            ("originalDoneUrl", "https://finance.yahoo.com/"),
            ("namespace", "yahoo"),
        ];

        client
            .post(format!(
                "https://consent.yahoo.com/v2/collectConsent?sessionId={}",
                session_id
            ))
            .form(&data)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        client
            .get(format!(
                "https://guce.yahoo.com/copyConsent?sessionId={}",
                session_id
            ))
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .query(&data)
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let crumb_response = client
            .get("https://query2.finance.yahoo.com/v1/test/getcrumb")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header("Accept", "*/*")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let crumb_text = crumb_response
            .text()
            .await
            .map_err(YahooError::NetworkError)?
            .trim()
            .to_string();

        if Self::is_valid_crumb(&crumb_text) {
            Ok(crumb_text)
        } else {
            Err(YahooError::AuthFailed(
                "Invalid crumb from query2 endpoint".to_string(),
            ))
        }
    }

    fn is_valid_crumb(crumb: &str) -> bool {
        !crumb.is_empty()
            && !crumb.contains("<html")
            && !crumb.contains("Unauthorized")
            && !crumb.contains("Too Many Requests")
    }
}
