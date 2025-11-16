use crate::client::{scraper, YahooFinanceClient};
use crate::client::error::YahooError;
use crate::client::FetchClient;
use serde_json::Value;
use std::sync::Arc;

pub async fn get_earnings_calls_list(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<Vec<Value>, YahooError> {
    // Try scraping first (as it's more reliable for this endpoint)
    match scraper::scrape_earnings_calls_list(fetch_client, symbol).await {
        Ok(calls) if !calls.is_empty() => Ok(calls),
        _ => {
            // Fallback: return empty list if scraping fails
            Ok(Vec::new())
        }
    }
}

pub async fn get_earnings_transcript(
    yahoo_client: &YahooFinanceClient,
    event_id: &str,
    company_id: &str,
) -> Result<Value, YahooError> {
    yahoo_client.get_earnings_transcript(event_id, company_id).await
}

