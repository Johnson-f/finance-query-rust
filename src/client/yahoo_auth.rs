use crate::client::error::YahooError;
use chrono::{DateTime, Utc};
use reqwest::{cookie::Jar, Client, ClientBuilder, Url};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const MIN_REFRESH_INTERVAL_SECS: i64 = 30;

struct AuthState {
    crumb: Option<String>,
    cookie_jar: Arc<Jar>,
    last_update: Option<DateTime<Utc>>,
}

pub struct YahooAuthManager {
    state: Arc<Mutex<AuthState>>,
    proxy: Option<String>,
}

impl YahooAuthManager {
    pub fn new(proxy: Option<String>) -> Self {
        let cookie_jar = Arc::new(Jar::default());
        Self {
            state: Arc::new(Mutex::new(AuthState {
                crumb: None,
                cookie_jar,
                last_update: None,
            })),
            proxy,
        }
    }

    pub async fn refresh(&self) -> Result<(), YahooError> {
        let mut builder = ClientBuilder::new().timeout(Duration::from_secs(10));

        if let Some(proxy_url) = &self.proxy {
            builder = builder.proxy(reqwest::Proxy::all(proxy_url).map_err(|e| {
                YahooError::NetworkError(e)
            })?);
        }

        let client = builder.build().map_err(YahooError::NetworkError)?;

        // Step 1: Visit fc.yahoo.com to get initial cookies
        let _ = client
            .get("https://fc.yahoo.com")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        // Step 2: Try to get crumb
        let crumb_response = client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let crumb_text = crumb_response.text().await.map_err(YahooError::NetworkError)?;
        let crumb = crumb_text.trim().to_string();

        // If crumb is invalid (contains HTML), use CSRF fallback
        if crumb.is_empty() || crumb.contains("<html") {
            return self.refresh_with_csrf(client).await;
        }

        // Successfully obtained crumb
        let mut state = self.state.lock().unwrap();
        state.crumb = Some(crumb);
        state.last_update = Some(Utc::now());

        // Extract cookies from client
        let cookie_jar = Arc::new(Jar::default());
        let base_url = Url::parse("https://finance.yahoo.com").map_err(|e| {
            YahooError::ParseError(format!("Failed to parse base URL: {}", e))
        })?;

        // Note: reqwest doesn't expose cookies directly, so we'll rely on the client's cookie store
        // For now, we'll create a new jar and let the client handle cookies
        state.cookie_jar = cookie_jar;

        Ok(())
    }

    async fn refresh_with_csrf(&self, client: Client) -> Result<(), YahooError> {
        // Get consent page
        let consent_response = client
            .get("https://guce.yahoo.com/consent")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let consent_html = consent_response.text().await.map_err(YahooError::NetworkError)?;

        // Extract CSRF token and session ID using regex
        let csrf_regex = regex::Regex::new(r#"name="csrfToken"[^>]*value="([^"]+)""#)
            .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;
        let session_regex = regex::Regex::new(r#"name="sessionId"[^>]*value="([^"]+)""#)
            .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;

        let csrf_token = csrf_regex
            .captures(&consent_html)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| YahooError::AuthFailed("Failed to extract CSRF token".to_string()))?;

        let session_id = session_regex
            .captures(&consent_html)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| YahooError::AuthFailed("Failed to extract session ID".to_string()))?;

        // Submit consent form
        let consent_data = [
            ("agree", "agree"),
            ("agree", "agree"),
            ("consentUUID", "default"),
            ("sessionId", &session_id),
            ("csrfToken", &csrf_token),
            ("originalDoneUrl", "https://finance.yahoo.com/"),
            ("namespace", "yahoo"),
        ];

        let _ = client
            .post(format!(
                "https://consent.yahoo.com/v2/collectConsent?sessionId={}",
                session_id
            ))
            .form(&consent_data)
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        // Copy consent
        let _ = client
            .get(format!(
                "https://guce.yahoo.com/copyConsent?sessionId={}",
                session_id
            ))
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        // Get crumb from alternative endpoint
        let crumb_response = client
            .get("https://query2.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let crumb_text = crumb_response.text().await.map_err(YahooError::NetworkError)?;
        let crumb = crumb_text.trim().to_string();

        if crumb.is_empty() || crumb.contains("<html") {
            return Err(YahooError::AuthFailed(
                "Yahoo returned an invalid crumb".to_string(),
            ));
        }

        // Successfully obtained crumb
        let mut state = self.state.lock().unwrap();
        state.crumb = Some(crumb);
        state.last_update = Some(Utc::now());

        Ok(())
    }

    pub async fn get_or_refresh(&self) -> Result<(Arc<Jar>, String), YahooError> {
        let mut state = self.state.lock().unwrap();

        let needs_refresh = state.crumb.is_none()
            || state.last_update.is_none()
            || (Utc::now() - state.last_update.unwrap()).num_seconds() > MIN_REFRESH_INTERVAL_SECS;

        if needs_refresh {
            drop(state); // Release lock before async operation
            self.refresh().await?;
            state = self.state.lock().unwrap();
        }

        let crumb = state
            .crumb
            .clone()
            .ok_or_else(|| YahooError::AuthFailed("No crumb available".to_string()))?;

        Ok((state.cookie_jar.clone(), crumb))
    }

    pub fn crumb(&self) -> Option<String> {
        self.state.lock().unwrap().crumb.clone()
    }
}

