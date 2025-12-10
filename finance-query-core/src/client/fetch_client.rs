//! HTTP fetch client for Yahoo Finance requests.
//!
//! This module provides a low-level HTTP client with proxy support,
//! cookie management, and timeout handling.

use crate::client::error::YahooError;
use reqwest::{cookie::Jar, Client, ClientBuilder};
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// HTTP client for fetching data from Yahoo Finance.
///
/// Provides cookie management, proxy support, and various fetch methods
/// for different content types (HTML, JSON).
#[derive(Debug)]
pub struct FetchClient {
    client: Client,
    cookie_jar: Arc<Jar>,
    #[allow(dead_code)]
    proxy: Option<String>,
    /// Separate proxy URL only for Yahoo auth requests (to save bandwidth)
    auth_proxy: Option<String>,
}

impl FetchClient {
    /// Create a new FetchClient with optional proxy support.
    ///
    /// # Arguments
    /// * `proxy` - Optional proxy URL for general requests
    ///
    /// # Environment Variables
    /// * `AUTH_PROXY_URL` - If set, used for authentication requests instead of the general proxy
    pub fn new(proxy: Option<String>) -> Result<Self, YahooError> {
        // Check for auth-only proxy first, fall back to general proxy
        let auth_proxy = std::env::var("AUTH_PROXY_URL")
            .ok()
            .or_else(|| proxy.clone());

        let cookie_jar = Arc::new(Jar::default());

        let mut builder = ClientBuilder::new()
            .timeout(DEFAULT_TIMEOUT)
            .cookie_store(true)
            .cookie_provider(cookie_jar.clone())
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36");

        // Only use proxy for general client if PROXY_URL is set (not AUTH_PROXY_URL)
        if let Some(proxy_url) = &proxy {
            builder =
                builder.proxy(reqwest::Proxy::all(proxy_url).map_err(YahooError::NetworkError)?);
        }

        let client = builder.build().map_err(YahooError::NetworkError)?;

        Ok(Self {
            client,
            cookie_jar,
            proxy,
            auth_proxy,
        })
    }

    /// Get the proxy URL to use for auth requests only.
    pub fn auth_proxy(&self) -> Option<&String> {
        self.auth_proxy.as_ref()
    }

    /// Get a reference to the underlying reqwest Client.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a reference to the cookie jar.
    pub fn cookie_jar(&self) -> &Arc<Jar> {
        &self.cookie_jar
    }

    /// Fetch a URL and return the response body as a string.
    pub async fn fetch(&self, url: &str) -> Result<String, YahooError> {
        self.fetch_with_timeout(url, DEFAULT_TIMEOUT).await
    }

    /// Fetch a URL expecting JSON response with proper Accept header.
    pub async fn fetch_json(&self, url: &str) -> Result<String, YahooError> {
        self.fetch_json_with_timeout(url, DEFAULT_TIMEOUT).await
    }

    /// Fetch a URL expecting JSON response with timeout and proper Accept header.
    /// Note: Accept-Encoding is not set to avoid compression issues with JSON parsing.
    pub async fn fetch_json_with_timeout(
        &self,
        url: &str,
        timeout: Duration,
    ) -> Result<String, YahooError> {
        let response = match tokio::time::timeout(
            timeout,
            self.client
                .get(url)
                .timeout(timeout)
                .header("Accept", "application/json")
                .header("Accept-Language", "en-US,en;q=0.9")
                // Don't request compression for JSON - it's usually small and compression can cause parsing issues
                .header(
                    "sec-ch-ua",
                    r#""Chromium";v="122", "Google Chrome";v="122""#,
                )
                .header("sec-ch-ua-mobile", "?0")
                .header("sec-ch-ua-platform", r#""Windows""#)
                .send(),
        )
        .await
        {
            Ok(Ok(resp)) => resp,
            Ok(Err(e)) => return Err(YahooError::NetworkError(e)),
            Err(_) => {
                return Err(YahooError::ParseError(format!(
                    "Request to {} timed out after {:?}",
                    url, timeout
                )));
            }
        };

        let status = response.status();
        if !status.is_success() {
            return Err(YahooError::HttpError(
                status.as_u16(),
                format!(
                    "HTTP {}: {}",
                    status,
                    response.status().canonical_reason().unwrap_or("Unknown")
                ),
            ));
        }

        // Check Content-Encoding header to see if response is compressed
        let content_encoding = response
            .headers()
            .get("content-encoding")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        let bytes = response.bytes().await.map_err(YahooError::NetworkError)?;

        // If response is compressed, decompress it
        let text = if content_encoding.contains("gzip") || content_encoding.contains("deflate") {
            // Try to decompress gzip/deflate
            let mut decoder = flate2::read::GzDecoder::new(&bytes[..]);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed).map_err(|e| {
                YahooError::ParseError(format!("Failed to decompress gzip response: {}", e))
            })?;
            decompressed
        } else if content_encoding.contains("br") {
            // Brotli compression - reqwest should handle this automatically, but if not, return error
            return Err(YahooError::ParseError(
                "Brotli compression detected but not automatically decompressed. This should not happen.".to_string()
            ));
        } else {
            // Try to convert bytes to string
            match String::from_utf8(bytes.to_vec()) {
                Ok(text) => text,
                Err(_) => {
                    // If not valid UTF-8, might be compressed without Content-Encoding header
                    // Try to decompress as gzip
                    let mut decoder = flate2::read::GzDecoder::new(&bytes[..]);
                    let mut decompressed = String::new();
                    match decoder.read_to_string(&mut decompressed) {
                        Ok(_) => decompressed,
                        Err(_) => {
                            // Not gzip either, return original error
                            return Err(YahooError::ParseError(format!(
                                "Response is not valid UTF-8 and not gzip compressed (length: {} bytes)",
                                bytes.len()
                            )));
                        }
                    }
                }
            }
        };

        Ok(text)
    }

    /// Fetch a URL with a custom timeout.
    pub async fn fetch_with_timeout(
        &self,
        url: &str,
        timeout: Duration,
    ) -> Result<String, YahooError> {
        // Create a request builder with timeout override
        // Use tokio::time::timeout to ensure the request doesn't exceed the specified timeout
        let response = match tokio::time::timeout(
            timeout,
            self.client
                .get(url)
                .timeout(timeout) // Explicitly set timeout on the request
                .header(
                    "Accept",
                    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8",
                )
                .header("Accept-Language", "en-US,en;q=0.9")
                .header("Accept-Encoding", "gzip, deflate, br")
                .header(
                    "sec-ch-ua",
                    r#""Chromium";v="122", "Google Chrome";v="122""#,
                )
                .header("sec-ch-ua-mobile", "?0")
                .header("sec-ch-ua-platform", r#""Windows""#)
                .send(),
        )
        .await
        {
            Ok(Ok(resp)) => resp,
            Ok(Err(e)) => return Err(YahooError::NetworkError(e)),
            Err(_) => {
                // Timeout occurred - return a parse error with timeout message
                return Err(YahooError::ParseError(format!(
                    "Request to {} timed out after {:?}",
                    url, timeout
                )));
            }
        };

        let status = response.status();
        if !status.is_success() {
            return Err(YahooError::HttpError(
                status.as_u16(),
                format!(
                    "HTTP {}: {}",
                    status,
                    response.status().canonical_reason().unwrap_or("Unknown")
                ),
            ));
        }

        response.text().await.map_err(YahooError::NetworkError)
    }

    /// Fetch a URL and return the raw response.
    pub async fn fetch_response(&self, url: &str) -> Result<reqwest::Response, YahooError> {
        let response = self
            .client
            .get(url)
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header(
                "sec-ch-ua",
                r#""Chromium";v="122", "Google Chrome";v="122""#,
            )
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", r#""Windows""#)
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        Ok(response)
    }
}
