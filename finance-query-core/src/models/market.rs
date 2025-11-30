use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market status information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketStatus {
    pub market: String,
    /// Current status: "open", "closed", "pre", "post"
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_offset: Option<i32>,
}

/// Market summary with index data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSummaryItem {
    pub exchange: String,
    pub short_name: String,
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub percent_change: f64,
}

/// Complete market summary response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSummaryResponse {
    pub market: String,
    pub status: Option<MarketStatus>,
    pub indices: Vec<MarketSummaryItem>,
}

impl MarketStatus {
    pub(crate) fn from_yahoo_response(
        market: String,
        response: YahooMarketTimeResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let market_time = response
            .finance
            .market_times
            .first()
            .and_then(|mt| mt.market_time.first())
            .ok_or_else(|| {
                crate::client::YahooError::ParseError("No market time data".to_string())
            })?;

        let timezone_info = market_time.timezone.first();

        let open_time = market_time
            .open
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let close_time = market_time
            .close
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Ok(Self {
            market,
            status: market_time.status.clone().unwrap_or_else(|| "unknown".to_string()),
            open_time,
            close_time,
            timezone: timezone_info.and_then(|tz| tz.long.clone()),
            timezone_short: timezone_info.and_then(|tz| tz.short.clone()),
            gmt_offset: timezone_info.and_then(|tz| tz.gmtoffset),
        })
    }

    /// Check if market is currently open
    pub fn is_open(&self) -> bool {
        self.status.to_lowercase() == "open"
    }

    /// Check if market is in pre-market hours
    pub fn is_pre_market(&self) -> bool {
        self.status.to_lowercase() == "pre"
    }

    /// Check if market is in after-hours
    pub fn is_after_hours(&self) -> bool {
        self.status.to_lowercase() == "post"
    }
}

impl MarketSummaryResponse {
    pub(crate) fn from_yahoo_response(
        market: String,
        summary_response: YahooMarketSummaryResponse,
        status: Option<MarketStatus>,
    ) -> Result<Self, crate::client::YahooError> {
        let indices = summary_response
            .market_summary_response
            .result
            .into_iter()
            .map(|item| MarketSummaryItem {
                exchange: item.exchange.unwrap_or_default(),
                short_name: item.short_name.unwrap_or_default(),
                symbol: item.symbol.unwrap_or_default(),
                price: item.regular_market_price.unwrap_or(0.0),
                change: item.regular_market_change.unwrap_or(0.0),
                percent_change: item.regular_market_change_percent.unwrap_or(0.0),
            })
            .collect();

        Ok(Self {
            market,
            status,
            indices,
        })
    }
}

// Internal Yahoo response structures
#[derive(Debug, Deserialize)]
pub(crate) struct YahooMarketTimeResponse {
    pub finance: FinanceData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FinanceData {
    #[serde(rename = "marketTimes")]
    pub market_times: Vec<MarketTimeWrapper>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MarketTimeWrapper {
    #[serde(rename = "marketTime")]
    pub market_time: Vec<MarketTimeData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MarketTimeData {
    pub status: Option<String>,
    pub open: Option<String>,
    pub close: Option<String>,
    pub timezone: Vec<TimezoneInfo>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TimezoneInfo {
    #[serde(deserialize_with = "deserialize_gmt_offset", default)]
    pub gmtoffset: Option<i32>,
    pub short: Option<String>,
    pub long: Option<String>,
}

/// Deserialize gmtoffset which can be either a string or integer
fn deserialize_gmt_offset<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum GmtOffset {
        Int(i32),
        String(String),
    }

    match Option::<GmtOffset>::deserialize(deserializer)? {
        Some(GmtOffset::Int(i)) => Ok(Some(i)),
        Some(GmtOffset::String(s)) => s.parse::<i32>().map(Some).map_err(D::Error::custom),
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooMarketSummaryResponse {
    #[serde(rename = "marketSummaryResponse")]
    pub market_summary_response: MarketSummaryData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MarketSummaryData {
    pub result: Vec<MarketSummaryResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MarketSummaryResult {
    pub exchange: Option<String>,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "regularMarketPrice")]
    pub regular_market_price: Option<f64>,
    #[serde(rename = "regularMarketChange")]
    pub regular_market_change: Option<f64>,
    #[serde(rename = "regularMarketChangePercent")]
    pub regular_market_change_percent: Option<f64>,
}
