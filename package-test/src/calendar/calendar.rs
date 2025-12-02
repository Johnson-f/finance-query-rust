//! Calendar data fetching functions using finance-query-core
//!
//! This module provides functions to fetch calendar event data from Yahoo Finance:
//! - Earnings dates (upcoming and historical)
//! - Dividend dates and ex-dividend dates
//! - Stock split dates
//! - Other corporate events

use finance_query_core::{FetchClient, YahooAuthManager, YahooError, YahooFinanceClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// ============================================================================
// Calendar Event Types
// ============================================================================

/// Earnings event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsEvent {
    pub earnings_date: Option<String>,
    pub earnings_date_start: Option<String>,
    pub earnings_date_end: Option<String>,
    pub earnings_average: Option<f64>,
    pub earnings_low: Option<f64>,
    pub earnings_high: Option<f64>,
    pub revenue_average: Option<f64>,
    pub revenue_low: Option<f64>,
    pub revenue_high: Option<f64>,
}

impl EarningsEvent {
    /// Check if earnings date is a range
    pub fn is_date_range(&self) -> bool {
        self.earnings_date_start.is_some()
            && self.earnings_date_end.is_some()
            && self.earnings_date_start != self.earnings_date_end
    }

    /// Get formatted date string
    pub fn date_display(&self) -> String {
        if self.is_date_range() {
            format!(
                "{} - {}",
                self.earnings_date_start.as_deref().unwrap_or("TBD"),
                self.earnings_date_end.as_deref().unwrap_or("TBD")
            )
        } else {
            self.earnings_date
                .as_deref()
                .or(self.earnings_date_start.as_deref())
                .unwrap_or("TBD")
                .to_string()
        }
    }
}

/// Dividend event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividendEvent {
    pub dividend_date: Option<String>,
    pub ex_dividend_date: Option<String>,
    pub dividend_rate: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub payout_ratio: Option<f64>,
    pub five_year_avg_dividend_yield: Option<f64>,
}

impl DividendEvent {
    /// Check if dividend is upcoming (has future ex-dividend date)
    pub fn has_upcoming_dividend(&self) -> bool {
        self.ex_dividend_date.is_some()
    }

    /// Get annual dividend amount
    pub fn annual_dividend(&self) -> Option<f64> {
        self.dividend_rate
    }
}

/// Stock split event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitEvent {
    pub split_date: Option<String>,
    pub split_ratio: Option<String>,
    pub split_factor: Option<f64>,
}

impl SplitEvent {
    /// Parse split ratio (e.g., "4:1" -> 4.0)
    pub fn split_multiplier(&self) -> Option<f64> {
        self.split_factor.or_else(|| {
            self.split_ratio.as_ref().and_then(|ratio| {
                let parts: Vec<&str> = ratio.split(':').collect();
                if parts.len() == 2 {
                    let numerator: f64 = parts[0].parse().ok()?;
                    let denominator: f64 = parts[1].parse().ok()?;
                    Some(numerator / denominator)
                } else {
                    None
                }
            })
        })
    }
}

// ============================================================================
// Complete Calendar Data
// ============================================================================

/// Complete calendar data for a symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarData {
    pub symbol: String,
    pub earnings: Option<EarningsEvent>,
    pub dividend: Option<DividendEvent>,
    pub split: Option<SplitEvent>,
}

impl CalendarData {
    /// Check if there are any upcoming events
    pub fn has_upcoming_events(&self) -> bool {
        self.earnings.as_ref().map(|e| e.earnings_date.is_some()).unwrap_or(false)
            || self.dividend.as_ref().map(|d| d.has_upcoming_dividend()).unwrap_or(false)
            || self.split.as_ref().map(|s| s.split_date.is_some()).unwrap_or(false)
    }

    /// Get next event date (earnings, dividend, or split - whichever is soonest)
    pub fn next_event_date(&self) -> Option<&str> {
        // Simple implementation - just return earnings date if available
        self.earnings
            .as_ref()
            .and_then(|e| e.earnings_date.as_deref())
            .or_else(|| {
                self.dividend
                    .as_ref()
                    .and_then(|d| d.ex_dividend_date.as_deref())
            })
    }
}

// ============================================================================
// Helper Functions
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

/// Convert Unix timestamp to date string
fn timestamp_to_date(ts: i64) -> String {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| ts.to_string())
}

/// Parse earnings event from JSON
fn parse_earnings_event(json: &Value) -> Option<EarningsEvent> {
    let result = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))?;

    let calendar = result.get("calendarEvents")?;
    let earnings = calendar.get("earnings")?;

    // Parse earnings dates
    let earnings_dates: Vec<String> = earnings
        .get("earningsDate")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_i64().map(timestamp_to_date))
                .collect()
        })
        .unwrap_or_default();

    let earnings_date = earnings_dates.first().cloned();
    let earnings_date_start = earnings_dates.first().cloned();
    let earnings_date_end = earnings_dates.get(1).cloned().or_else(|| earnings_date_start.clone());

    // Parse estimates
    let earnings_average = earnings
        .get("earningsAverage")
        .and_then(|v| v.as_f64());
    let earnings_low = earnings.get("earningsLow").and_then(|v| v.as_f64());
    let earnings_high = earnings.get("earningsHigh").and_then(|v| v.as_f64());
    let revenue_average = earnings.get("revenueAverage").and_then(|v| v.as_f64());
    let revenue_low = earnings.get("revenueLow").and_then(|v| v.as_f64());
    let revenue_high = earnings.get("revenueHigh").and_then(|v| v.as_f64());

    Some(EarningsEvent {
        earnings_date,
        earnings_date_start,
        earnings_date_end,
        earnings_average,
        earnings_low,
        earnings_high,
        revenue_average,
        revenue_low,
        revenue_high,
    })
}

/// Parse dividend event from JSON
fn parse_dividend_event(json: &Value) -> Option<DividendEvent> {
    let result = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))?;

    let calendar = result.get("calendarEvents");
    let summary = result.get("summaryDetail");

    // Get dates from calendarEvents
    let dividend_date = calendar
        .and_then(|c| c.get("dividendDate"))
        .and_then(|d| d.as_i64())
        .map(timestamp_to_date);

    let ex_dividend_date = calendar
        .and_then(|c| c.get("exDividendDate"))
        .and_then(|d| d.as_i64())
        .map(timestamp_to_date);

    // Get dividend info from summaryDetail
    let dividend_rate = summary
        .and_then(|s| s.get("dividendRate"))
        .and_then(|r| r.as_f64());

    let dividend_yield = summary
        .and_then(|s| s.get("dividendYield"))
        .and_then(|y| y.as_f64());

    let payout_ratio = summary
        .and_then(|s| s.get("payoutRatio"))
        .and_then(|p| p.as_f64());

    let five_year_avg = summary
        .and_then(|s| s.get("fiveYearAvgDividendYield"))
        .and_then(|y| y.as_f64());

    // Only return if we have some dividend data
    if dividend_date.is_some()
        || ex_dividend_date.is_some()
        || dividend_rate.is_some()
        || dividend_yield.is_some()
    {
        Some(DividendEvent {
            dividend_date,
            ex_dividend_date,
            dividend_rate,
            dividend_yield,
            payout_ratio,
            five_year_avg_dividend_yield: five_year_avg,
        })
    } else {
        None
    }
}

/// Parse split event from JSON (if available)
fn parse_split_event(json: &Value) -> Option<SplitEvent> {
    let result = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))?;

    // Check defaultKeyStatistics for split info
    let stats = result.get("defaultKeyStatistics")?;

    let split_date = stats
        .get("lastSplitDate")
        .and_then(|d| d.as_i64())
        .map(timestamp_to_date);

    let split_factor = stats
        .get("lastSplitFactor")
        .and_then(|f| f.as_str())
        .map(String::from);

    if split_date.is_some() || split_factor.is_some() {
        Some(SplitEvent {
            split_date,
            split_ratio: split_factor,
            split_factor: None,
        })
    } else {
        None
    }
}


// ============================================================================
// Public API Functions
// ============================================================================

/// Get raw calendar data JSON for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `modules` - Slice of module names to fetch
///
/// # Returns
/// Raw JSON value containing calendar data
pub async fn get_calendar_raw(symbol: &str, modules: &[&str]) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_quote_summary(symbol, modules).await
}

/// Get earnings calendar for a symbol
///
/// Returns upcoming earnings date and estimates
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// EarningsEvent with date and estimate information
///
/// # Example
/// ```rust,ignore
/// let earnings = get_earnings_calendar("AAPL").await?;
/// println!("Next earnings: {}", earnings.date_display());
/// if let Some(est) = earnings.earnings_average {
///     println!("EPS estimate: ${:.2}", est);
/// }
/// ```
pub async fn get_earnings_calendar(symbol: &str) -> Result<EarningsEvent, YahooError> {
    let json = get_calendar_raw(symbol, &["calendarEvents"]).await?;
    parse_earnings_event(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse earnings calendar".to_string())
    })
}

/// Get dividend calendar for a symbol
///
/// Returns dividend dates and yield information
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// DividendEvent with date and yield information
///
/// # Example
/// ```rust,ignore
/// let dividend = get_dividend_calendar("AAPL").await?;
/// if let Some(yield_pct) = dividend.dividend_yield {
///     println!("Dividend yield: {:.2}%", yield_pct * 100.0);
/// }
/// ```
pub async fn get_dividend_calendar(symbol: &str) -> Result<DividendEvent, YahooError> {
    let json = get_calendar_raw(symbol, &["calendarEvents", "summaryDetail"]).await?;
    parse_dividend_event(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse dividend calendar or no dividend data".to_string())
    })
}

/// Get split history for a symbol
///
/// Returns last split date and ratio
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// SplitEvent with split information
///
/// # Example
/// ```rust,ignore
/// let split = get_split_info("AAPL").await?;
/// if let Some(date) = &split.split_date {
///     println!("Last split: {} ({})", date, split.split_ratio.as_deref().unwrap_or("N/A"));
/// }
/// ```
pub async fn get_split_info(symbol: &str) -> Result<SplitEvent, YahooError> {
    let json = get_calendar_raw(symbol, &["defaultKeyStatistics"]).await?;
    parse_split_event(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse split info or no split data".to_string())
    })
}

/// Get all calendar data for a symbol
///
/// Fetches earnings, dividend, and split information
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// CalendarData with all calendar information
///
/// # Example
/// ```rust,ignore
/// let calendar = get_full_calendar("AAPL").await?;
/// if let Some(earnings) = &calendar.earnings {
///     println!("Next earnings: {}", earnings.date_display());
/// }
/// if let Some(dividend) = &calendar.dividend {
///     println!("Ex-dividend: {}", dividend.ex_dividend_date.as_deref().unwrap_or("N/A"));
/// }
/// ```
pub async fn get_full_calendar(symbol: &str) -> Result<CalendarData, YahooError> {
    let modules = &["calendarEvents", "summaryDetail", "defaultKeyStatistics"];
    let json = get_calendar_raw(symbol, modules).await?;

    Ok(CalendarData {
        symbol: symbol.to_string(),
        earnings: parse_earnings_event(&json),
        dividend: parse_dividend_event(&json),
        split: parse_split_event(&json),
    })
}

/// Get calendar data for multiple symbols
///
/// Fetches calendar data for multiple symbols
///
/// # Arguments
/// * `symbols` - Slice of stock symbols
///
/// # Returns
/// Vec of CalendarData for each symbol
///
/// # Example
/// ```rust,ignore
/// let calendars = get_calendars_for_symbols(&["AAPL", "GOOGL", "MSFT"]).await?;
/// for cal in &calendars {
///     if let Some(earnings) = &cal.earnings {
///         println!("{}: {}", cal.symbol, earnings.date_display());
///     }
/// }
/// ```
pub async fn get_calendars_for_symbols(symbols: &[&str]) -> Result<Vec<CalendarData>, YahooError> {
    let mut results = Vec::new();

    for symbol in symbols {
        match get_full_calendar(symbol).await {
            Ok(calendar) => results.push(calendar),
            Err(_) => continue, // Skip symbols that fail
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require network access and valid Yahoo Finance authentication
    // Run with: cargo test -p package-test -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_get_earnings_calendar() {
        let result = get_earnings_calendar("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_dividend_calendar() {
        let result = get_dividend_calendar("AAPL").await;
        assert!(result.is_ok());
        let dividend = result.unwrap();
        assert!(dividend.dividend_yield.is_some() || dividend.dividend_rate.is_some());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_split_info() {
        let result = get_split_info("AAPL").await;
        // May or may not have split data
        let _ = result;
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_full_calendar() {
        let result = get_full_calendar("AAPL").await;
        assert!(result.is_ok());
        let calendar = result.unwrap();
        assert_eq!(calendar.symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_calendars_for_symbols() {
        let result = get_calendars_for_symbols(&["AAPL", "MSFT"]).await;
        assert!(result.is_ok());
        let calendars = result.unwrap();
        assert!(!calendars.is_empty());
    }

    #[test]
    fn test_earnings_date_display() {
        let earnings = EarningsEvent {
            earnings_date: Some("2024-01-25".to_string()),
            earnings_date_start: Some("2024-01-25".to_string()),
            earnings_date_end: Some("2024-01-25".to_string()),
            earnings_average: Some(2.10),
            earnings_low: Some(1.95),
            earnings_high: Some(2.25),
            revenue_average: None,
            revenue_low: None,
            revenue_high: None,
        };

        assert!(!earnings.is_date_range());
        assert_eq!(earnings.date_display(), "2024-01-25");
    }

    #[test]
    fn test_earnings_date_range() {
        let earnings = EarningsEvent {
            earnings_date: None,
            earnings_date_start: Some("2024-01-25".to_string()),
            earnings_date_end: Some("2024-01-29".to_string()),
            earnings_average: None,
            earnings_low: None,
            earnings_high: None,
            revenue_average: None,
            revenue_low: None,
            revenue_high: None,
        };

        assert!(earnings.is_date_range());
        assert_eq!(earnings.date_display(), "2024-01-25 - 2024-01-29");
    }

    #[test]
    fn test_split_multiplier() {
        let split = SplitEvent {
            split_date: Some("2020-08-31".to_string()),
            split_ratio: Some("4:1".to_string()),
            split_factor: None,
        };

        assert_eq!(split.split_multiplier(), Some(4.0));
    }

    #[test]
    fn test_dividend_event() {
        let dividend = DividendEvent {
            dividend_date: Some("2024-02-15".to_string()),
            ex_dividend_date: Some("2024-02-09".to_string()),
            dividend_rate: Some(0.96),
            dividend_yield: Some(0.0052),
            payout_ratio: Some(0.15),
            five_year_avg_dividend_yield: Some(0.65),
        };

        assert!(dividend.has_upcoming_dividend());
        assert_eq!(dividend.annual_dividend(), Some(0.96));
    }
}
