//! Yahoo Finance authentication manager.
//!
//! This module handles cookie/crumb authentication for Yahoo Finance API requests.

use crate::client::error::YahooError;
use chrono::{DateTime, Utc};
use reqwest::{cookie::Jar, ClientBuilder};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

const MIN_REFRESH_INTERVAL_SECS: i64 = 30;

struct AuthState {
    crumb: Option<String>,
    cookie_jar: Arc<Jar>,
    last_update: Option<DateTime<Utc>>,
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
            })),
            proxy,
        }
    }

    /// Force refresh the authentication credentials.
    pub async fn refresh(&self) -> Result<(), YahooError> {
        info!("Refreshing Yahoo authentication...");
        let state = self.state.lock().await;
        let cookie_jar = state.cookie_jar.clone();
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
                    reqwest::Proxy::all(proxy_url).map_err(|e| YahooError::NetworkError(e))?,
                )
                // Accept proxy's SSL certificate (Bright Data and similar proxies use self-signed certs)
                .danger_accept_invalid_certs(true);
        }

        let client = builder.build().map_err(YahooError::NetworkError)?;

        // Step 1: Visit Yahoo Finance homepage to establish session
        info!("Step 1: Visiting Yahoo Finance homepage");
        let _ = client
            .get("https://finance.yahoo.com/")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Connection", "keep-alive")
            .header("Upgrade-Insecure-Requests", "1")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        debug!("Finance homepage visited");

        // Step 2: Visit a quote page to get more cookies
        info!("Step 2: Visiting quote page");
        let _ = client
            .get("https://finance.yahoo.com/quote/AAPL")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Referer", "https://finance.yahoo.com/")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        debug!("Quote page visited");

        // Step 3: Try to get crumb from query1
        info!("Step 3: Attempting to get crumb from query1 endpoint");
        let crumb_response = client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header("Accept", "*/*")
            .header("Referer", "https://finance.yahoo.com/")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let status = crumb_response.status();
        debug!("getcrumb response status: {}", status);

        let crumb_text = crumb_response
            .text()
            .await
            .map_err(YahooError::NetworkError)?;
        let crumb = crumb_text.trim().to_string();
        debug!(
            "Crumb received (length: {}): {}",
            crumb.len(),
            if crumb.len() < 50 { &crumb } else { "..." }
        );


        // If crumb is invalid (contains HTML or is empty), try alternative method
        if crumb.is_empty() || crumb.contains("<html") || crumb.contains("Unauthorized") {
            warn!("Crumb from query1 is invalid, trying query2");

            let crumb_response2 = client
                .get("https://query2.finance.yahoo.com/v1/test/getcrumb")
                .header(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
                )
                .header("Accept", "*/*")
                .header("Referer", "https://finance.yahoo.com/")
                .send()
                .await
                .map_err(YahooError::NetworkError)?;

            let crumb_text2 = crumb_response2
                .text()
                .await
                .map_err(YahooError::NetworkError)?;
            let crumb2 = crumb_text2.trim().to_string();

            if crumb2.is_empty() || crumb2.contains("<html") || crumb2.contains("Unauthorized") {
                error!("Failed to get valid crumb from both endpoints");
                return Err(YahooError::AuthFailed(
                    "Could not obtain valid crumb from Yahoo".to_string(),
                ));
            }

            info!(
                "Successfully obtained crumb from query2 (length: {})",
                crumb2.len()
            );
            let mut state = self.state.lock().await;
            state.crumb = Some(crumb2);
            state.last_update = Some(Utc::now());
            return Ok(());
        }

        // Successfully obtained crumb from query1
        info!("Successfully obtained crumb (length: {})", crumb.len());
        let mut state = self.state.lock().await;
        state.crumb = Some(crumb);
        state.last_update = Some(Utc::now());

        Ok(())
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
}
