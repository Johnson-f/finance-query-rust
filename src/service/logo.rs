use finance_query_core::client::FetchClient;
use finance_query_core::client::error::YahooError;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};
const LOGO_TIMEOUT_SECONDS: u64 = 1;

/// Fetches a company logo URL from Financial Modeling Prep's image-stock endpoint.
///
/// Uses the provided ticker symbol to build the logo URL. Returns None if logo
/// fetching is disabled, fails, or times out.
///
/// # Arguments
/// * `fetch_client` - The HTTP client to use for fetching
/// * `symbol` - Optional stock ticker symbol (e.g., "AAPL")
/// * `website_url` - Unused; retained for backward compatibility
///
/// # Returns
/// * `Option<String>` - The logo URL if successful, None otherwise
pub async fn get_logo(
    fetch_client: &Arc<FetchClient>,
    symbol: Option<&str>,
    _website_url: Option<&str>,
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

    // Try fetching by ticker symbol
    if let Some(sym) = symbol {
        let ticker_url = format!(
            "https://financialmodelingprep.com/image-stock/{}.png",
            sym.to_uppercase()
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
                warn!(
                    "Logo fetch timeout for symbol {} after {}s",
                    sym, timeout_secs
                );
            }
        }
    }

    None
}

/// Fetches a logo URL and returns the final URL.
///
/// The Financial Modeling Prep endpoint returns the actual logo image URL, so
/// we simply check for success and return the final URL.
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
