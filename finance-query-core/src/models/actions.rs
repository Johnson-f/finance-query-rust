use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a dividend payment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dividend {
    pub date: DateTime<Utc>,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Represents a stock split
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockSplit {
    pub date: DateTime<Utc>,
    pub numerator: f64,
    pub denominator: f64,
    /// Split ratio as string (e.g., "2:1")
    pub split_ratio: String,
}

impl StockSplit {
    pub fn new(date: DateTime<Utc>, numerator: f64, denominator: f64) -> Self {
        let split_ratio = format!("{}:{}", numerator, denominator);
        Self {
            date,
            numerator,
            denominator,
            split_ratio,
        }
    }
}

/// Represents a capital gain distribution (for ETFs/Mutual Funds)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapitalGain {
    pub date: DateTime<Utc>,
    pub amount: f64,
}

/// Response containing all stock actions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionsResponse {
    pub symbol: String,
    pub dividends: Vec<Dividend>,
    pub splits: Vec<StockSplit>,
    pub capital_gains: Vec<CapitalGain>,
}

impl ActionsResponse {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            dividends: Vec::new(),
            splits: Vec::new(),
            capital_gains: Vec::new(),
        }
    }

    /// Check if there are any actions
    pub fn is_empty(&self) -> bool {
        self.dividends.is_empty() && self.splits.is_empty() && self.capital_gains.is_empty()
    }

    /// Get total dividend amount
    pub fn total_dividends(&self) -> f64 {
        self.dividends.iter().map(|d| d.amount).sum()
    }

    /// Parse Yahoo Finance events response
    pub(crate) fn from_yahoo_response(
        symbol: String,
        response: YahooEventsResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let mut actions = Self::new(symbol);

        if let Some(result) = response.chart.result.first() {
            if let Some(events) = &result.events {
                // Parse dividends
                if let Some(divs) = &events.dividends {
                    for div in divs.values() {
                        actions.dividends.push(Dividend {
                            date: Utc.timestamp_opt(div.date, 0).single().ok_or_else(|| {
                                crate::client::YahooError::ParseError(
                                    "Invalid dividend timestamp".to_string(),
                                )
                            })?,
                            amount: div.amount,
                            currency: None,
                        });
                    }
                }

                // Parse splits
                if let Some(splits) = &events.splits {
                    for split in splits.values() {
                        actions.splits.push(StockSplit::new(
                            Utc.timestamp_opt(split.date, 0).single().ok_or_else(|| {
                                crate::client::YahooError::ParseError(
                                    "Invalid split timestamp".to_string(),
                                )
                            })?,
                            split.numerator,
                            split.denominator,
                        ));
                    }
                }

                // Parse capital gains
                if let Some(gains) = &events.capital_gains {
                    for gain in gains.values() {
                        actions.capital_gains.push(CapitalGain {
                            date: Utc.timestamp_opt(gain.date, 0).single().ok_or_else(|| {
                                crate::client::YahooError::ParseError(
                                    "Invalid capital gain timestamp".to_string(),
                                )
                            })?,
                            amount: gain.amount,
                        });
                    }
                }
            }
        }

        // Sort by date (oldest first)
        actions.dividends.sort_by_key(|d| d.date);
        actions.splits.sort_by_key(|s| s.date);
        actions.capital_gains.sort_by_key(|g| g.date);

        Ok(actions)
    }
}

// Internal structures for parsing Yahoo response
#[derive(Debug, Deserialize)]
pub(crate) struct YahooEventsResponse {
    pub chart: ChartData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChartData {
    pub result: Vec<ChartResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChartResult {
    pub events: Option<Events>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Events {
    pub dividends: Option<HashMap<String, YahooDividend>>,
    pub splits: Option<HashMap<String, YahooSplit>>,
    #[serde(rename = "capitalGains")]
    pub capital_gains: Option<HashMap<String, YahooCapitalGain>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooDividend {
    pub amount: f64,
    pub date: i64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooSplit {
    pub date: i64,
    pub numerator: f64,
    pub denominator: f64,
    #[serde(rename = "splitRatio")]
    #[allow(dead_code)]
    pub split_ratio: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooCapitalGain {
    pub amount: f64,
    pub date: i64,
}
