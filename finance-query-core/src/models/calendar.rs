use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Calendar events for a stock (earnings, dividends, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub symbol: String,
    /// Earnings date (single date or start of range)
    pub earnings_date: Option<DateTime<Utc>>,
    /// Earnings date range start
    pub earnings_date_start: Option<DateTime<Utc>>,
    /// Earnings date range end
    pub earnings_date_end: Option<DateTime<Utc>>,
    /// Dividend date
    pub dividend_date: Option<DateTime<Utc>>,
    /// Ex-dividend date
    pub ex_dividend_date: Option<DateTime<Utc>>,
    /// Dividend rate
    pub dividend_rate: Option<f64>,
    /// Dividend yield
    pub dividend_yield: Option<f64>,
}

impl Calendar {
    pub(crate) fn from_yahoo_response(
        symbol: String,
        response: YahooCalendarResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let result = response.quote_summary.result.first().ok_or_else(|| {
            crate::client::YahooError::ParseError("No calendar data in response".to_string())
        })?;

        let events = &result.calendar_events;

        // Parse earnings dates (can be raw timestamps or objects)
        let earnings_dates: Vec<DateTime<Utc>> = events
            .earnings
            .as_ref()
            .and_then(|e| e.earnings_date.as_ref())
            .map(|dates| {
                dates
                    .iter()
                    .filter_map(|ts| Utc.timestamp_opt(*ts, 0).single())
                    .collect()
            })
            .unwrap_or_default();

        let earnings_date = earnings_dates.first().copied();
        let earnings_date_start = earnings_dates.first().copied();
        let earnings_date_end = earnings_dates.get(1).copied().or(earnings_date_start);

        // Parse dividend date (raw timestamp)
        let dividend_date = events
            .dividend_date
            .and_then(|ts| Utc.timestamp_opt(ts, 0).single());

        // Parse ex-dividend date (raw timestamp)
        let ex_dividend_date = events
            .ex_dividend_date
            .and_then(|ts| Utc.timestamp_opt(ts, 0).single());

        Ok(Self {
            symbol,
            earnings_date,
            earnings_date_start,
            earnings_date_end,
            dividend_date,
            ex_dividend_date,
            dividend_rate: None,
            dividend_yield: None,
        })
    }
}

// Internal Yahoo response structures
#[derive(Debug, Deserialize)]
pub(crate) struct YahooCalendarResponse {
    #[serde(rename = "quoteSummary")]
    pub quote_summary: QuoteSummaryData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QuoteSummaryData {
    pub result: Vec<QuoteSummaryResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QuoteSummaryResult {
    #[serde(rename = "calendarEvents")]
    pub calendar_events: CalendarEventsData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CalendarEventsData {
    pub earnings: Option<EarningsData>,
    #[serde(rename = "dividendDate")]
    pub dividend_date: Option<i64>,
    #[serde(rename = "exDividendDate")]
    pub ex_dividend_date: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EarningsData {
    #[serde(rename = "earningsDate")]
    pub earnings_date: Option<Vec<i64>>,
}
