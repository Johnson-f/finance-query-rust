//! Historical data fetching functions using finance-query-core
//!
//! This module provides functions to fetch historical price data from Yahoo Finance:
//! - Chart data with range (e.g., "1d", "5d", "1mo", "1y", "max")
//! - Chart data with specific date periods (Unix timestamps)

use finance_query_core::{FetchClient, YahooAuthManager, YahooFinanceClient, YahooError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// ============================================================================
// Chart Response Types (from get_chart / chart endpoint)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartResponse {
    pub chart: ChartResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartResult {
    pub error: Option<Value>,
    pub result: Option<Vec<ChartData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    pub meta: ChartMeta,
    pub timestamp: Option<Vec<i64>>,
    pub indicators: Indicators,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartMeta {
    pub currency: Option<String>,
    pub symbol: Option<String>,
    pub exchange_name: Option<String>,
    pub full_exchange_name: Option<String>,
    pub instrument_type: Option<String>,
    pub first_trade_date: Option<i64>,
    pub regular_market_time: Option<i64>,
    pub gmtoffset: Option<i64>,
    pub timezone: Option<String>,
    pub exchange_timezone_name: Option<String>,
    pub regular_market_price: Option<f64>,
    pub chart_previous_close: Option<f64>,
    pub previous_close: Option<f64>,
    pub scale: Option<i32>,
    pub price_hint: Option<i32>,
    pub current_trading_period: Option<CurrentTradingPeriod>,
    pub trading_periods: Option<Vec<Vec<TradingPeriod>>>,
    pub data_granularity: Option<String>,
    pub range: Option<String>,
    pub valid_ranges: Option<Vec<String>>,
}


/// Current trading period - contains single TradingPeriod objects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTradingPeriod {
    pub pre: Option<TradingPeriod>,
    pub regular: Option<TradingPeriod>,
    pub post: Option<TradingPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPeriod {
    pub timezone: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub gmtoffset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicators {
    pub quote: Option<Vec<QuoteIndicator>>,
    pub adjclose: Option<Vec<AdjCloseIndicator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteIndicator {
    pub open: Option<Vec<Option<f64>>>,
    pub high: Option<Vec<Option<f64>>>,
    pub low: Option<Vec<Option<f64>>>,
    pub close: Option<Vec<Option<f64>>>,
    pub volume: Option<Vec<Option<i64>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjCloseIndicator {
    pub adjclose: Option<Vec<Option<f64>>>,
}

// ============================================================================
// Simplified OHLCV Data Structure
// ============================================================================

/// A single OHLCV (Open, High, Low, Close, Volume) data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcvBar {
    pub timestamp: i64,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<i64>,
    pub adj_close: Option<f64>,
}

impl OhlcvBar {
    /// Get the date as a formatted string (YYYY-MM-DD)
    pub fn date_string(&self) -> String {
        use chrono::{TimeZone, Utc};
        Utc.timestamp_opt(self.timestamp, 0)
            .single()
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "Invalid".to_string())
    }

    /// Get the datetime as a formatted string (YYYY-MM-DD HH:MM:SS)
    pub fn datetime_string(&self) -> String {
        use chrono::{TimeZone, Utc};
        Utc.timestamp_opt(self.timestamp, 0)
            .single()
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid".to_string())
    }
}

/// Historical price data with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalData {
    pub symbol: String,
    pub currency: Option<String>,
    pub exchange: Option<String>,
    pub interval: String,
    pub range: Option<String>,
    pub bars: Vec<OhlcvBar>,
}

impl HistoricalData {
    /// Create from ChartResponse
    pub fn from_chart_response(response: &ChartResponse, interval: &str) -> Option<Self> {
        let result = response.chart.result.as_ref()?.first()?;
        let meta = &result.meta;
        let timestamps = result.timestamp.as_ref()?;
        let quote = result.indicators.quote.as_ref()?.first()?;
        let adjclose = result.indicators.adjclose.as_ref().and_then(|a| a.first());

        let bars: Vec<OhlcvBar> = timestamps
            .iter()
            .enumerate()
            .map(|(i, &ts)| OhlcvBar {
                timestamp: ts,
                open: quote.open.as_ref().and_then(|v| v.get(i).copied().flatten()),
                high: quote.high.as_ref().and_then(|v| v.get(i).copied().flatten()),
                low: quote.low.as_ref().and_then(|v| v.get(i).copied().flatten()),
                close: quote.close.as_ref().and_then(|v| v.get(i).copied().flatten()),
                volume: quote.volume.as_ref().and_then(|v| v.get(i).copied().flatten()),
                adj_close: adjclose.and_then(|ac| ac.adjclose.as_ref().and_then(|v| v.get(i).copied().flatten())),
            })
            .collect();

        Some(Self {
            symbol: meta.symbol.clone().unwrap_or_default(),
            currency: meta.currency.clone(),
            exchange: meta.exchange_name.clone(),
            interval: interval.to_string(),
            range: meta.range.clone(),
            bars,
        })
    }

    /// Get the first bar (oldest)
    pub fn first(&self) -> Option<&OhlcvBar> {
        self.bars.first()
    }

    /// Get the last bar (most recent)
    pub fn last(&self) -> Option<&OhlcvBar> {
        self.bars.last()
    }

    /// Get the number of bars
    pub fn len(&self) -> usize {
        self.bars.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    /// Calculate the price change from first to last bar
    pub fn price_change(&self) -> Option<f64> {
        let first_close = self.first()?.close?;
        let last_close = self.last()?.close?;
        Some(last_close - first_close)
    }

    /// Calculate the percentage change from first to last bar
    pub fn percent_change(&self) -> Option<f64> {
        let first_close = self.first()?.close?;
        let last_close = self.last()?.close?;
        Some((last_close - first_close) / first_close * 100.0)
    }

    /// Get the highest price in the range
    pub fn high(&self) -> Option<f64> {
        self.bars.iter().filter_map(|b| b.high).max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Get the lowest price in the range
    pub fn low(&self) -> Option<f64> {
        self.bars.iter().filter_map(|b| b.low).min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Get total volume in the range
    pub fn total_volume(&self) -> i64 {
        self.bars.iter().filter_map(|b| b.volume).sum()
    }
}


// ============================================================================
// Client Helper
// ============================================================================

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

// ============================================================================
// Public API Functions
// ============================================================================

/// Get historical chart data for a symbol using a range string
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `interval` - Data interval (e.g., "1m", "5m", "15m", "1h", "1d", "1wk", "1mo")
/// * `range` - Time range (e.g., "1d", "5d", "1mo", "3mo", "6mo", "1y", "2y", "5y", "10y", "ytd", "max")
///
/// # Returns
/// Raw JSON value containing chart data
///
/// # Example
/// ```rust,ignore
/// let chart = get_chart("AAPL", "1d", "1mo").await?;
/// println!("{:?}", chart);
/// ```
pub async fn get_chart(symbol: &str, interval: &str, range: &str) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_chart(symbol, interval, range).await
}

/// Get historical chart data for a symbol using Unix timestamps
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `interval` - Data interval (e.g., "1m", "5m", "15m", "1h", "1d", "1wk", "1mo")
/// * `period1` - Start time as Unix timestamp
/// * `period2` - End time as Unix timestamp
///
/// # Returns
/// Raw JSON value containing chart data
///
/// # Example
/// ```rust,ignore
/// use chrono::Utc;
/// let now = Utc::now().timestamp();
/// let one_year_ago = now - 365 * 24 * 60 * 60;
/// let chart = get_chart_with_periods("AAPL", "1d", one_year_ago, now).await?;
/// println!("{:?}", chart);
/// ```
pub async fn get_chart_with_periods(
    symbol: &str,
    interval: &str,
    period1: i64,
    period2: i64,
) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_chart_with_periods(symbol, interval, period1, period2).await
}

/// Get historical data as a structured HistoricalData object
///
/// This is a convenience function that fetches chart data and parses it
/// into a more usable format with OHLCV bars.
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `interval` - Data interval (e.g., "1d", "1wk", "1mo")
/// * `range` - Time range (e.g., "1mo", "3mo", "1y", "max")
///
/// # Returns
/// HistoricalData struct with parsed OHLCV bars
///
/// # Example
/// ```rust,ignore
/// let data = get_historical_data("AAPL", "1d", "1mo").await?;
/// println!("Got {} bars for {}", data.len(), data.symbol);
/// if let Some(last) = data.last() {
///     println!("Latest close: ${:.2}", last.close.unwrap_or(0.0));
/// }
/// ```
pub async fn get_historical_data(
    symbol: &str,
    interval: &str,
    range: &str,
) -> Result<HistoricalData, YahooError> {
    let json = get_chart(symbol, interval, range).await?;
    let response: ChartResponse = serde_json::from_value(json)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse chart response: {}", e)))?;

    HistoricalData::from_chart_response(&response, interval)
        .ok_or_else(|| YahooError::ParseError("No chart data available".to_string()))
}

/// Get historical data using date strings (YYYY-MM-DD format)
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `interval` - Data interval (e.g., "1d", "1wk", "1mo")
/// * `start_date` - Start date as "YYYY-MM-DD"
/// * `end_date` - End date as "YYYY-MM-DD"
///
/// # Returns
/// HistoricalData struct with parsed OHLCV bars
///
/// # Example
/// ```rust,ignore
/// let data = get_historical_data_by_date("AAPL", "1d", "2024-01-01", "2024-12-01").await?;
/// println!("Got {} bars", data.len());
/// ```
pub async fn get_historical_data_by_date(
    symbol: &str,
    interval: &str,
    start_date: &str,
    end_date: &str,
) -> Result<HistoricalData, YahooError> {
    let period1 = date_to_timestamp(start_date)?;
    let period2 = date_to_timestamp(end_date)?;

    let json = get_chart_with_periods(symbol, interval, period1, period2).await?;
    let response: ChartResponse = serde_json::from_value(json)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse chart response: {}", e)))?;

    HistoricalData::from_chart_response(&response, interval)
        .ok_or_else(|| YahooError::ParseError("No chart data available".to_string()))
}

/// Convert a date string (YYYY-MM-DD) to Unix timestamp
fn date_to_timestamp(date: &str) -> Result<i64, YahooError> {
    use chrono::NaiveDate;
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
        .map_err(|e| YahooError::ParseError(format!("Invalid date format '{}': {}", date, e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require network access and valid Yahoo Finance authentication
    // Run with: cargo test -p package-test -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_get_chart() {
        let result = get_chart("AAPL", "1d", "1mo").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_chart_with_periods() {
        use chrono::Utc;
        let now = Utc::now().timestamp();
        let one_month_ago = now - 30 * 24 * 60 * 60;
        let result = get_chart_with_periods("AAPL", "1d", one_month_ago, now).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_historical_data() {
        let result = get_historical_data("AAPL", "1d", "1mo").await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(!data.is_empty());
        assert_eq!(data.symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_historical_data_by_date() {
        let result = get_historical_data_by_date("AAPL", "1d", "2024-01-01", "2024-01-31").await;
        assert!(result.is_ok());
    }
}
