//! WebSocket support types for real-time data streaming.
//!
//! This module provides framework-agnostic data structures for WebSocket
//! subscriptions. These types can be used with any WebSocket framework.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{Quote, SimpleQuote, News};
use crate::models::movers::MarketMover;
use crate::models::sectors::MarketSector;

/// Real-time quote update for streaming stock quotes.
///
/// This type supports streaming a single quote or multiple quotes at once.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesUpdate {
    /// List of updated quotes
    pub quotes: Vec<SimpleQuote>,
    /// Timestamp of the update
    pub timestamp: DateTime<Utc>,
}

impl QuotesUpdate {
    /// Create a new QuotesUpdate with a single quote.
    pub fn single(quote: SimpleQuote) -> Self {
        Self {
            quotes: vec![quote],
            timestamp: Utc::now(),
        }
    }

    /// Create a new QuotesUpdate with multiple quotes.
    pub fn multiple(quotes: Vec<SimpleQuote>) -> Self {
        Self {
            quotes,
            timestamp: Utc::now(),
        }
    }

    /// Create a new QuotesUpdate with a specific timestamp.
    pub fn with_timestamp(quotes: Vec<SimpleQuote>, timestamp: DateTime<Utc>) -> Self {
        Self { quotes, timestamp }
    }

    /// Check if this update contains a specific symbol.
    pub fn contains_symbol(&self, symbol: &str) -> bool {
        self.quotes.iter().any(|q| q.symbol == symbol)
    }

    /// Get a quote by symbol if present.
    pub fn get_quote(&self, symbol: &str) -> Option<&SimpleQuote> {
        self.quotes.iter().find(|q| q.symbol == symbol)
    }

    /// Returns true if this update contains no quotes.
    pub fn is_empty(&self) -> bool {
        self.quotes.is_empty()
    }

    /// Returns the number of quotes in this update.
    pub fn len(&self) -> usize {
        self.quotes.len()
    }
}

/// Profile update containing quote, similar stocks, sector performance, and news.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdate {
    /// Current quote data for the symbol
    pub quote: Option<Quote>,
    /// Similar stocks to the symbol
    pub similar: Option<Vec<SimpleQuote>>,
    /// Sector performance data
    pub sector_performance: Option<MarketSector>,
    /// Recent news for the symbol
    pub news: Option<Vec<News>>,
}

/// Market movers update containing actives, gainers, and losers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoversUpdate {
    /// Most active stocks
    pub actives: Option<Vec<MarketMover>>,
    /// Top gaining stocks
    pub gainers: Option<Vec<MarketMover>>,
    /// Top losing stocks
    pub losers: Option<Vec<MarketMover>>,
}

/// Market hours status update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHours {
    /// Current market status (e.g., "open", "closed", "pre-market", "after-hours")
    pub status: String,
    /// Optional reason for the status (e.g., holiday name)
    pub reason: Option<String>,
    /// Timestamp of the status update
    pub timestamp: DateTime<Utc>,
}

/// Moving average update for real-time indicator streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverageUpdate {
    /// Stock symbol
    pub symbol: String,
    /// Indicator type (e.g., "SMA", "EMA")
    pub indicator_type: String,
    /// Period for the moving average
    pub period: i32,
    /// Calculated value
    pub value: f64,
    /// Timestamp of the calculation
    pub timestamp: DateTime<Utc>,
}
