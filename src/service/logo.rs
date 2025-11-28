use crate::client::error::YahooError;
use crate::client::FetchClient;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};
use url::Url;

const LOGO_DEV_TOKEN: &str = "pk_Xd1Cdye3QYmCOXzcvxhxyw";
const LOGO_TIMEOUT_SECONDS: u64 = 1;

/// Fetches a company logo URL from logo.dev API.
/// 
/// Tries to fetch by ticker symbol first, then falls back to domain-based lookup.
/// Returns None if logo fetching is disabled, fails, or times out.
/// 
/// # Arguments
/// * `fetch_client` - The HTTP client to use for fetching
/// * `symbol` - Optional stock ticker symbol (e.g., "AAPL")
/// * `website_url` - Optional company website URL (e.g., "https://www.apple.com")
/// 
/// # Returns
/// * `Option<String>` - The logo URL if successful, None otherwise
pub async fn get_logo(
    fetch_client: &Arc<FetchClient>,
    symbol: Option<&str>,
    website_url: Option<&str>,
) -> Option<String> {
    // Check if logo fetching is disabled
    if std::env::var("DISABLE_LOGO_FETCHING")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true"
    {
        debug!("Logo fetching is disabled via DISABLE_LOGO_FETCHING");
        return None;
    }

    // Get timeout from environment or use default
    let timeout_secs = std::env::var("LOGO_TIMEOUT_SECONDS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(LOGO_TIMEOUT_SECONDS);
    let timeout = Duration::from_secs(timeout_secs);

    // Try fetching by ticker symbol first
    if let Some(sym) = symbol {
        let ticker_url = format!(
            "https://img.logo.dev/ticker/{}?token={}&retina=true",
            sym, LOGO_DEV_TOKEN
        );
        
        match tokio::time::timeout(timeout, fetch_logo_url(fetch_client, &ticker_url)).await {
            Ok(Ok(Some(logo_url))) => {
                debug!("Successfully fetched logo for symbol {}: {}", sym, logo_url);
                return Some(logo_url);
            }
            Ok(Ok(None)) => {
                debug!("Logo fetch returned None for symbol {}", sym);
            }
            Ok(Err(e)) => {
                debug!("Logo fetch failed for symbol {}: {}", sym, e);
            }
            Err(_) => {
                warn!("Logo fetch timeout for symbol {} after {}s", sym, timeout_secs);
            }
        }
    }

    // Fall back to domain-based lookup if website URL is provided
    if let Some(url_str) = website_url
        && let Ok(parsed_url) = Url::parse(url_str)
        && let Some(domain) = parsed_url.domain()
    {
        // Remove www. prefix if present
        let domain = domain.strip_prefix("www.").unwrap_or(domain);
        let domain_url = format!(
            "https://img.logo.dev/{}?token={}&retina=true",
            domain, LOGO_DEV_TOKEN
        );

        match tokio::time::timeout(timeout, fetch_logo_url(fetch_client, &domain_url)).await {
            Ok(Ok(Some(logo_url))) => {
                debug!("Successfully fetched logo for domain {}: {}", domain, logo_url);
                return Some(logo_url);
            }
            Ok(Ok(None)) => {
                debug!("Logo fetch returned None for domain {}", domain);
            }
            Ok(Err(e)) => {
                debug!("Logo fetch failed for domain {}: {}", domain, e);
            }
            Err(_) => {
                warn!("Logo fetch timeout for domain {} after {}s", domain, timeout_secs);
            }
        }
    }

    None
}

/// Fetches a logo URL from logo.dev and returns the final URL.
/// 
/// The logo.dev API redirects to the actual logo image URL, so we need to
/// follow the redirect and return the final URL.
async fn fetch_logo_url(
    fetch_client: &Arc<FetchClient>,
    url: &str,
) -> Result<Option<String>, YahooError> {
    let response = fetch_client.fetch_response(url).await?;
    
    let status = response.status();
    if status.is_success() {
        // Get the final URL after redirects
        let final_url = response.url().to_string();
        Ok(Some(final_url))
    } else {
        debug!("Logo fetch returned status {} for URL: {}", status, url);
        Ok(None)
    }
}

