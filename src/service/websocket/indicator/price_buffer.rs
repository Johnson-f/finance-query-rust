use chrono::{Datelike, TimeZone};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricePoint {
    pub price: f64,
    pub timestamp: i64,
}

/// Manages price buffers for symbols
pub struct PriceBufferManager {
    /// Map of symbol:timeframe to circular buffer of prices
    buffers: Arc<RwLock<HashMap<String, Vec<PricePoint>>>>,
    /// Map of symbol:timeframe to last date (YYYY-MM-DD) a price was added (for daily/weekly intervals)
    last_update_dates: Arc<RwLock<HashMap<String, String>>>,
    /// Maximum buffer size (e.g., 1000 for longest MA period)
    max_size: usize,
}

impl PriceBufferManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
            last_update_dates: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }

    /// Add a new price point for a symbol:timeframe
    /// For daily intervals: only adds if it's a new trading day and market is closed
    /// For weekly intervals: only adds if it's a new week (end of week, after market close)
    pub async fn add_price(
        &self,
        buffer_key: &str,
        price: f64,
        is_daily: bool,
        is_weekly: bool,
    ) -> bool {
        // For daily/weekly intervals, check if we should add
        if is_daily || is_weekly {
            let now_et = New_York.from_utc_datetime(&chrono::Utc::now().naive_utc());
            let current_date = now_et.date_naive();

            // Check if market is closed (after 4 PM ET)
            let market_close = chrono::NaiveTime::from_hms_opt(16, 0, 0).unwrap();
            let current_time = now_et.time();

            // Only add if market is closed (after 4 PM)
            if current_time < market_close {
                return false; // Market not closed yet
            }

            let mut last_dates = self.last_update_dates.write().await;

            if is_weekly {
                // For weekly: check if it's a new week (end of week = Friday)
                let weekday = now_et.weekday();
                if weekday != chrono::Weekday::Fri {
                    return false; // Not end of week yet
                }

                // Get the week identifier (year-week)
                let week_str = format!(
                    "{}-W{:02}",
                    current_date.year(),
                    current_date.iso_week().week()
                );

                if let Some(last_week) = last_dates.get(buffer_key)
                    && last_week == &week_str
                {
                    return false; // Already added this week
                }

                // Update last week
                last_dates.insert(buffer_key.to_string(), week_str);
            } else {
                // For daily: check if it's a new day
                let date_str = current_date.format("%Y-%m-%d").to_string();

                if let Some(last_date) = last_dates.get(buffer_key)
                    && last_date == &date_str
                {
                    return false; // Already added today
                }

                // Update last date
                last_dates.insert(buffer_key.to_string(), date_str);
            }
        }

        // Add the price
        let mut buffers = self.buffers.write().await;
        let buffer = buffers
            .entry(buffer_key.to_string())
            .or_insert_with(Vec::new);

        buffer.push(PricePoint {
            price,
            timestamp: chrono::Utc::now().timestamp(),
        });

        // Keep only last max_size prices
        if buffer.len() > self.max_size {
            buffer.remove(0);
        }

        true
    }

    /// Get price buffer for a symbol
    pub async fn get_prices(&self, symbol: &str) -> Vec<PricePoint> {
        let buffers = self.buffers.read().await;
        buffers.get(symbol).cloned().unwrap_or_default()
    }

    /// Initialize buffer with historical data
    pub async fn initialize_from_historical(&self, symbol: &str, prices: Vec<PricePoint>) {
        let mut buffers = self.buffers.write().await;
        // Only keep the last max_size prices
        let prices_len = prices.len();
        let prices_to_store: Vec<PricePoint> = if prices_len > self.max_size {
            prices
                .into_iter()
                .skip(prices_len - self.max_size)
                .collect()
        } else {
            prices
        };
        buffers.insert(symbol.to_string(), prices_to_store);
    }
}
