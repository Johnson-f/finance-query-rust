//! WebSocket message types for streaming data.
//!
//! This module defines the data structures used for WebSocket communication,
//! providing strongly-typed messages for various streaming endpoints.

use crate::models::{MarketMover, SimpleQuote};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Update containing multiple quotes with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesUpdate {
    pub quotes: Vec<SimpleQuote>,
    pub timestamp: DateTime<Utc>,
}

impl QuotesUpdate {
    pub fn new(quotes: Vec<SimpleQuote>) -> Self {
        Self {
            quotes,
            timestamp: Utc::now(),
        }
    }

    pub fn with_timestamp(quotes: Vec<SimpleQuote>, timestamp: DateTime<Utc>) -> Self {
        Self { quotes, timestamp }
    }
}

/// Update containing market movers (actives, gainers, losers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoversUpdate {
    pub actives: Vec<MarketMover>,
    pub gainers: Vec<MarketMover>,
    pub losers: Vec<MarketMover>,
    pub timestamp: DateTime<Utc>,
}

impl MoversUpdate {
    pub fn new(
        actives: Vec<MarketMover>,
        gainers: Vec<MarketMover>,
        losers: Vec<MarketMover>,
    ) -> Self {
        Self {
            actives,
            gainers,
            losers,
            timestamp: Utc::now(),
        }
    }

    pub fn with_timestamp(
        actives: Vec<MarketMover>,
        gainers: Vec<MarketMover>,
        losers: Vec<MarketMover>,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            actives,
            gainers,
            losers,
            timestamp,
        }
    }
}

/// Profile update (placeholder for future implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdate {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
}

/// Market hours information (placeholder for future implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHours {
    pub is_open: bool,
    pub timestamp: DateTime<Utc>,
}

/// Moving average update (placeholder for future implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverageUpdate {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
}
