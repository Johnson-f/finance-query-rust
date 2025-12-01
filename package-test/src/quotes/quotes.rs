//! Quote fetching functions using finance-query-core
//!
//! This module provides functions to fetch quote data from Yahoo Finance:
//! - Detailed quotes for symbols
//! - Simple quotes (summary data) for symbols
//! - Similar quotes to a queried symbol
//! - Logo URL for a single symbol

use finance_query_core::{
    FetchClient, YahooAuthManager, YahooFinanceClient, YahooError,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// ============================================================================
// Detailed Quote Response Types (from get_quote / quoteSummary endpoint)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedQuoteResponse {
    pub quote_summary: QuoteSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSummary {
    pub error: Option<Value>,
    pub result: Option<Vec<QuoteSummaryResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSummaryResult {
    pub asset_profile: Option<AssetProfile>,
    pub calendar_events: Option<CalendarEvents>,
    pub default_key_statistics: Option<DefaultKeyStatistics>,
    pub price: Option<PriceData>,
    pub summary_detail: Option<SummaryDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetProfile {
    pub address1: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub industry_disp: Option<String>,
    pub sector: Option<String>,
    pub sector_disp: Option<String>,
    pub long_business_summary: Option<String>,
    pub full_time_employees: Option<i64>,
    pub company_officers: Option<Vec<CompanyOfficer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyOfficer {
    pub name: Option<String>,
    pub title: Option<String>,
    pub age: Option<i32>,
    pub year_born: Option<i32>,
    pub total_pay: Option<FormattedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvents {
    pub dividend_date: Option<FormattedValue>,
    pub ex_dividend_date: Option<FormattedValue>,
    pub earnings: Option<EarningsCalendar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsCalendar {
    pub earnings_date: Option<Vec<FormattedValue>>,
    pub earnings_average: Option<FormattedValue>,
    pub earnings_high: Option<FormattedValue>,
    pub earnings_low: Option<FormattedValue>,
    pub revenue_average: Option<FormattedValue>,
    pub revenue_high: Option<FormattedValue>,
    pub revenue_low: Option<FormattedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultKeyStatistics {
    pub enterprise_value: Option<FormattedValue>,
    pub forward_pe: Option<FormattedValue>,
    pub profit_margins: Option<FormattedValue>,
    pub float_shares: Option<FormattedValue>,
    pub shares_outstanding: Option<FormattedValue>,
    pub shares_short: Option<FormattedValue>,
    pub short_ratio: Option<FormattedValue>,
    pub beta: Option<FormattedValue>,
    pub book_value: Option<FormattedValue>,
    pub price_to_book: Option<FormattedValue>,
    pub trailing_eps: Option<FormattedValue>,
    pub forward_eps: Option<FormattedValue>,
    pub peg_ratio: Option<FormattedValue>,
    pub last_split_factor: Option<String>,
    pub last_split_date: Option<FormattedValue>,
    pub last_dividend_value: Option<FormattedValue>,
    pub last_dividend_date: Option<FormattedValue>,
    #[serde(rename = "52WeekChange")]
    pub fifty_two_week_change: Option<FormattedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceData {
    pub symbol: Option<String>,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub currency: Option<String>,
    pub exchange: Option<String>,
    pub exchange_name: Option<String>,
    pub market_state: Option<String>,
    pub regular_market_price: Option<FormattedValue>,
    pub regular_market_change: Option<FormattedValue>,
    pub regular_market_change_percent: Option<FormattedValue>,
    pub regular_market_open: Option<FormattedValue>,
    pub regular_market_day_high: Option<FormattedValue>,
    pub regular_market_day_low: Option<FormattedValue>,
    pub regular_market_volume: Option<FormattedValue>,
    pub regular_market_previous_close: Option<FormattedValue>,
    pub regular_market_time: Option<i64>,
    pub pre_market_price: Option<FormattedValue>,
    pub pre_market_change: Option<FormattedValue>,
    pub pre_market_change_percent: Option<FormattedValue>,
    pub post_market_price: Option<FormattedValue>,
    pub post_market_change: Option<FormattedValue>,
    pub post_market_change_percent: Option<FormattedValue>,
    pub market_cap: Option<FormattedValue>,
    pub average_daily_volume_10_day: Option<FormattedValue>,
    pub average_daily_volume_3_month: Option<FormattedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryDetail {
    pub previous_close: Option<FormattedValue>,
    pub open: Option<FormattedValue>,
    pub day_low: Option<FormattedValue>,
    pub day_high: Option<FormattedValue>,
    pub regular_market_previous_close: Option<FormattedValue>,
    pub regular_market_open: Option<FormattedValue>,
    pub regular_market_day_low: Option<FormattedValue>,
    pub regular_market_day_high: Option<FormattedValue>,
    pub dividend_rate: Option<FormattedValue>,
    pub dividend_yield: Option<FormattedValue>,
    pub ex_dividend_date: Option<FormattedValue>,
    pub payout_ratio: Option<FormattedValue>,
    pub five_year_avg_dividend_yield: Option<FormattedValue>,
    pub beta: Option<FormattedValue>,
    pub trailing_pe: Option<FormattedValue>,
    pub forward_pe: Option<FormattedValue>,
    pub volume: Option<FormattedValue>,
    pub regular_market_volume: Option<FormattedValue>,
    pub average_volume: Option<FormattedValue>,
    pub average_volume_10days: Option<FormattedValue>,
    pub bid: Option<FormattedValue>,
    pub ask: Option<FormattedValue>,
    pub bid_size: Option<FormattedValue>,
    pub ask_size: Option<FormattedValue>,
    pub market_cap: Option<FormattedValue>,
    pub fifty_two_week_low: Option<FormattedValue>,
    pub fifty_two_week_high: Option<FormattedValue>,
    pub price_to_sales_trailing_12_months: Option<FormattedValue>,
    pub fifty_day_average: Option<FormattedValue>,
    pub two_hundred_day_average: Option<FormattedValue>,
    pub trailing_annual_dividend_rate: Option<FormattedValue>,
    pub trailing_annual_dividend_yield: Option<FormattedValue>,
}

// ============================================================================
// Simple Quote Response Types (from get_simple_quotes / quoteResponse endpoint)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleQuotesResponse {
    pub quote_response: QuoteResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub error: Option<Value>,
    pub result: Option<Vec<SimpleQuoteResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleQuoteResult {
    pub symbol: String,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub display_name: Option<String>,
    pub quote_type: Option<String>,
    pub currency: Option<String>,
    pub exchange: Option<String>,
    pub full_exchange_name: Option<String>,
    pub market: Option<String>,
    pub market_state: Option<String>,
    pub regular_market_price: Option<f64>,
    pub regular_market_change: Option<f64>,
    pub regular_market_change_percent: Option<f64>,
    pub regular_market_open: Option<f64>,
    pub regular_market_day_high: Option<f64>,
    pub regular_market_day_low: Option<f64>,
    pub regular_market_volume: Option<i64>,
    pub regular_market_previous_close: Option<f64>,
    pub regular_market_time: Option<i64>,
    pub pre_market_price: Option<f64>,
    pub pre_market_change: Option<f64>,
    pub pre_market_change_percent: Option<f64>,
    pub post_market_price: Option<f64>,
    pub post_market_change: Option<f64>,
    pub post_market_change_percent: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub bid_size: Option<i64>,
    pub ask_size: Option<i64>,
    pub market_cap: Option<i64>,
    pub fifty_two_week_low: Option<f64>,
    pub fifty_two_week_high: Option<f64>,
    pub fifty_two_week_change_percent: Option<f64>,
    pub fifty_day_average: Option<f64>,
    pub fifty_day_average_change: Option<f64>,
    pub fifty_day_average_change_percent: Option<f64>,
    pub two_hundred_day_average: Option<f64>,
    pub two_hundred_day_average_change: Option<f64>,
    pub two_hundred_day_average_change_percent: Option<f64>,
    pub average_daily_volume_10_day: Option<i64>,
    pub average_daily_volume_3_month: Option<i64>,
    pub trailing_pe: Option<f64>,
    pub forward_pe: Option<f64>,
    pub price_to_book: Option<f64>,
    pub book_value: Option<f64>,
    pub eps_trailing_twelve_months: Option<f64>,
    pub eps_forward: Option<f64>,
    pub eps_current_year: Option<f64>,
    pub dividend_rate: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub dividend_date: Option<i64>,
    pub trailing_annual_dividend_rate: Option<f64>,
    pub trailing_annual_dividend_yield: Option<f64>,
    pub earnings_timestamp: Option<i64>,
    pub earnings_timestamp_start: Option<i64>,
    pub earnings_timestamp_end: Option<i64>,
    pub average_analyst_rating: Option<String>,
}

// ============================================================================
// Similar Quotes Response Types (from get_similar_quotes / finance endpoint)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarQuotesResponse {
    pub finance: FinanceResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinanceResult {
    pub error: Option<Value>,
    pub result: Option<Vec<RecommendationResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationResult {
    pub symbol: String,
    pub recommended_symbols: Vec<RecommendedSymbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedSymbol {
    pub symbol: String,
    pub score: f64,
}

// ============================================================================
// Common Types
// ============================================================================

/// Yahoo Finance formatted value with raw and formatted representations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormattedValue {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
    pub long_fmt: Option<String>,
}

impl FormattedValue {
    pub fn value(&self) -> Option<f64> {
        self.raw
    }
    
    pub fn formatted(&self) -> Option<&str> {
        self.fmt.as_deref()
    }
}

/// Creates a configured YahooFinanceClient
async fn create_client() -> Result<(Arc<YahooAuthManager>, YahooFinanceClient), YahooError> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);
    
    // Prime authentication
    auth_manager.refresh().await?;
    
    Ok((auth_manager, client))
}

/// Get detailed quotes for symbols
///
/// Fetches comprehensive quote data including price, volume, market cap,
/// PE ratio, earnings, dividends, and more.
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Raw JSON value containing detailed quote data
///
/// # Example
/// ```rust,ignore
/// let quote = get_detailed_quote("AAPL").await?;
/// println!("{:?}", quote);
/// ```
pub async fn get_detailed_quote(symbol: &str) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_quote(symbol).await
}

/// Get simple quotes for symbols (summary data)
///
/// Fetches basic quote data for multiple symbols including price,
/// change, and percent change.
///
/// # Arguments
/// * `symbols` - Slice of stock symbols (e.g., &["AAPL", "GOOGL", "MSFT"])
///
/// # Returns
/// Raw JSON value containing simple quote data for all symbols
///
/// # Example
/// ```rust,ignore
/// let quotes = get_simple_quotes(&["AAPL", "GOOGL", "MSFT"]).await?;
/// println!("{:?}", quotes);
/// ```
pub async fn get_simple_quotes(symbols: &[&str]) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_simple_quotes(symbols).await
}

/// Get similar quotes to a queried symbol
///
/// Fetches recommended/similar symbols based on the provided symbol.
///
/// # Arguments
/// * `symbol` - The stock symbol to find similar quotes for
/// * `limit` - Maximum number of similar quotes to return
///
/// # Returns
/// Raw JSON value containing similar/recommended symbols
///
/// # Example
/// ```rust,ignore
/// let similar = get_similar_quotes("AAPL", 5).await?;
/// println!("{:?}", similar);
/// ```
pub async fn get_similar_quotes(symbol: &str, limit: usize) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_similar_quotes(symbol, limit).await
}

/// Get only the logo URL for a single symbol
///
/// Fetches the logo URL from the quote summary data.
/// The logo URL is extracted from the asset profile if available.
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Option containing the logo URL if available
///
/// # Example
/// ```rust,ignore
/// if let Some(logo_url) = get_logo_url("AAPL").await? {
///     println!("Logo: {}", logo_url);
/// }
/// ```
pub async fn get_logo_url(symbol: &str) -> Result<Option<String>, YahooError> {
    let (_, client) = create_client().await?;
    
    // Fetch quote summary with assetProfile module which contains logo
    let summary = client.get_quote_summary(symbol, &["assetProfile"]).await?;
    
    // Extract logo URL from the response
    let logo_url = summary
        .get("quoteSummary")
        .and_then(|qs| qs.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("assetProfile"))
        .and_then(|ap| ap.get("website"))
        .and_then(|w| w.as_str())
        .map(|website| {
            // Generate logo URL using Clearbit or similar service
            // Yahoo doesn't directly provide logo URLs, so we construct one
            let domain = website
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_start_matches("www.")
                .split('/')
                .next()
                .unwrap_or("");
            format!("https://logo.clearbit.com/{}", domain)
        });
    
    Ok(logo_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require network access and valid Yahoo Finance authentication
    // Run with: cargo test -p package-test -- --ignored
    
    #[tokio::test]
    #[ignore]
    async fn test_get_detailed_quote() {
        let result = get_detailed_quote("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_simple_quotes() {
        let result = get_simple_quotes(&["AAPL", "GOOGL"]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_similar_quotes() {
        let result = get_similar_quotes("AAPL", 5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_logo_url() {
        let result = get_logo_url("AAPL").await;
        assert!(result.is_ok());
    }
}
