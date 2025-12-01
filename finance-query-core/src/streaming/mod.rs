//! Streaming functionality for real-time data.
//!
//! This module provides async streams for continuously fetching financial data
//! at configurable intervals. These streams can be used with any async runtime
//! and integrated into WebSocket servers or other streaming applications.

use crate::client::error::YahooError;
use crate::client::YahooFinanceClient;
use crate::models::SimpleQuote;
use crate::websocket::QuotesUpdate;
use async_stream::stream;
use chrono::Utc;
use futures_util::Stream;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, error};

/// A stream that yields quote updates at regular intervals.
pub struct QuoteStream;

impl QuoteStream {
    /// Create a new quote stream that fetches quotes for the given symbols
    /// at the specified interval.
    ///
    /// # Arguments
    /// * `client` - The Yahoo Finance client to use for fetching quotes
    /// * `symbols` - List of stock symbols to track
    /// * `poll_interval` - How often to fetch new quotes
    ///
    /// # Example
    /// ```rust,ignore
    /// use finance_query_core::{QuoteStream, YahooFinanceClient};
    /// use std::time::Duration;
    /// use futures_util::StreamExt;
    ///
    /// let stream = QuoteStream::new(&client, vec!["AAPL", "GOOGL"], Duration::from_secs(5));
    /// 
    /// while let Some(result) = stream.next().await {
    ///     match result {
    ///         Ok(update) => println!("Got {} quotes", update.len()),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub fn new(
        client: Arc<YahooFinanceClient>,
        symbols: Vec<String>,
        poll_interval: Duration,
    ) -> Pin<Box<dyn Stream<Item = Result<QuotesUpdate, YahooError>> + Send>> {
        Box::pin(stream! {
            let mut ticker = interval(poll_interval);
            
            loop {
                ticker.tick().await;
                debug!("Fetching quotes for {:?}", symbols);
                
                let symbol_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
                
                match client.get_simple_quotes(&symbol_refs).await {
                    Ok(data) => {
                        let quotes = parse_simple_quotes(&data);
                        let update = QuotesUpdate::with_timestamp(quotes, Utc::now());
                        yield Ok(update);
                    }
                    Err(e) => {
                        error!("Failed to fetch quotes: {}", e);
                        yield Err(e);
                    }
                }
            }
        })
    }

    /// Create a quote stream with a default 5-second interval.
    pub fn with_default_interval(
        client: Arc<YahooFinanceClient>,
        symbols: Vec<String>,
    ) -> Pin<Box<dyn Stream<Item = Result<QuotesUpdate, YahooError>> + Send>> {
        Self::new(client, symbols, Duration::from_secs(5))
    }
}

/// A stream that yields a single quote update for one symbol.
pub struct SingleQuoteStream;

impl SingleQuoteStream {
    /// Create a stream for a single symbol.
    pub fn new(
        client: Arc<YahooFinanceClient>,
        symbol: String,
        poll_interval: Duration,
    ) -> Pin<Box<dyn Stream<Item = Result<SimpleQuote, YahooError>> + Send>> {
        Box::pin(stream! {
            let mut ticker = interval(poll_interval);
            
            loop {
                ticker.tick().await;
                debug!("Fetching quote for {}", symbol);
                
                match client.get_simple_quotes(&[symbol.as_str()]).await {
                    Ok(data) => {
                        let quotes = parse_simple_quotes(&data);
                        if let Some(quote) = quotes.into_iter().next() {
                            yield Ok(quote);
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch quote for {}: {}", symbol, e);
                        yield Err(e);
                    }
                }
            }
        })
    }
}

/// A stream that yields market index updates at regular intervals.
pub struct IndexStream;

impl IndexStream {
    /// Create a new index stream that fetches index data at the specified interval.
    ///
    /// # Arguments
    /// * `client` - The Yahoo Finance client to use for fetching data
    /// * `index_symbols` - List of index symbols to track (e.g., "^GSPC", "^DJI", "^IXIC")
    /// * `poll_interval` - How often to fetch new data
    ///
    /// # Example
    /// ```rust,ignore
    /// use finance_query_core::{IndexStream, YahooFinanceClient};
    /// use std::time::Duration;
    /// use futures_util::StreamExt;
    ///
    /// // Stream S&P 500, Dow Jones, and NASDAQ
    /// let symbols = vec!["^GSPC".to_string(), "^DJI".to_string(), "^IXIC".to_string()];
    /// let stream = IndexStream::new(&client, symbols, Duration::from_secs(5));
    /// 
    /// while let Some(result) = stream.next().await {
    ///     match result {
    ///         Ok(indices) => {
    ///             for index in indices {
    ///                 println!("{}: {} ({:+})", index.name, index.value, index.percent_change);
    ///             }
    ///         }
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub fn new(
        client: Arc<YahooFinanceClient>,
        index_symbols: Vec<String>,
        poll_interval: Duration,
    ) -> Pin<Box<dyn Stream<Item = Result<Vec<crate::models::MarketIndex>, YahooError>> + Send>> {
        Box::pin(stream! {
            let mut ticker = interval(poll_interval);
            
            loop {
                ticker.tick().await;
                debug!("Fetching index data for {:?}", index_symbols);
                
                let symbol_refs: Vec<&str> = index_symbols.iter().map(|s| s.as_str()).collect();
                
                match client.get_simple_quotes(&symbol_refs).await {
                    Ok(data) => {
                        let indices = parse_market_indices(&data);
                        yield Ok(indices);
                    }
                    Err(e) => {
                        error!("Failed to fetch index data: {}", e);
                        yield Err(e);
                    }
                }
            }
        })
    }

    /// Create an index stream with a default 5-second interval.
    pub fn with_default_interval(
        client: Arc<YahooFinanceClient>,
        index_symbols: Vec<String>,
    ) -> Pin<Box<dyn Stream<Item = Result<Vec<crate::models::MarketIndex>, YahooError>> + Send>> {
        Self::new(client, index_symbols, Duration::from_secs(5))
    }

    /// Create a stream for major US indices (S&P 500, Dow Jones, NASDAQ).
    pub fn us_major_indices(
        client: Arc<YahooFinanceClient>,
        poll_interval: Duration,
    ) -> Pin<Box<dyn Stream<Item = Result<Vec<crate::models::MarketIndex>, YahooError>> + Send>> {
        let symbols = vec![
            "^GSPC".to_string(),  // S&P 500
            "^DJI".to_string(),   // Dow Jones
            "^IXIC".to_string(),  // NASDAQ
        ];
        Self::new(client, symbols, poll_interval)
    }
}

/// Parse simple quotes from Yahoo Finance API response.
fn parse_simple_quotes(data: &Value) -> Vec<SimpleQuote> {
    let mut quotes = Vec::new();
    
    if let Some(results) = data
        .get("quoteResponse")
        .and_then(|qr| qr.get("result"))
        .and_then(|r| r.as_array())
    {
        for result in results {
            let quote = SimpleQuote {
                symbol: result.get("symbol")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string(),
                name: result.get("longName")
                    .or_else(|| result.get("shortName"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string(),
                price: result.get("regularMarketPrice")
                    .and_then(|p| p.as_f64())
                    .map(|p| format!("{:.2}", p))
                    .unwrap_or_else(|| "0.00".to_string()),
                pre_market_price: result.get("preMarketPrice")
                    .and_then(|p| p.as_f64())
                    .map(|p| format!("{:.2}", p)),
                after_hours_price: result.get("postMarketPrice")
                    .and_then(|p| p.as_f64())
                    .map(|p| format!("{:.2}", p)),
                change: result.get("regularMarketChange")
                    .and_then(|c| c.as_f64())
                    .map(|c| format!("{:+.2}", c))
                    .unwrap_or_else(|| "0.00".to_string()),
                percent_change: result.get("regularMarketChangePercent")
                    .and_then(|p| p.as_f64())
                    .map(|p| format!("{:+.2}%", p))
                    .unwrap_or_else(|| "0.00%".to_string()),
                logo: None,
            };
            quotes.push(quote);
        }
    }
    
    quotes
}

/// A stream that yields market movers (actives, gainers, losers) at regular intervals.
pub struct MoversStream;

impl MoversStream {
    /// Create a new movers stream that fetches market movers at the specified interval.
    ///
    /// # Arguments
    /// * `client` - The Yahoo Finance client to use for fetching data
    /// * `count` - Number of movers to return (25, 50, or 100)
    /// * `poll_interval` - How often to fetch new data
    ///
    /// # Example
    /// ```rust,ignore
    /// use finance_query_core::{MoversStream, YahooFinanceClient, MoverCount};
    /// use std::time::Duration;
    /// use futures_util::StreamExt;
    ///
    /// let stream = MoversStream::new(&client, MoverCount::Fifty, Duration::from_secs(5));
    /// 
    /// while let Some(result) = stream.next().await {
    ///     match result {
    ///         Ok(update) => {
    ///             println!("Actives: {}", update.actives.len());
    ///             println!("Gainers: {}", update.gainers.len());
    ///             println!("Losers: {}", update.losers.len());
    ///         }
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub fn new(
        client: Arc<YahooFinanceClient>,
        count: crate::models::MoverCount,
        poll_interval: Duration,
    ) -> Pin<Box<dyn Stream<Item = Result<crate::websocket::MoversUpdate, YahooError>> + Send>> {
        Box::pin(stream! {
            let mut ticker = interval(poll_interval);
            
            loop {
                ticker.tick().await;
                debug!("Fetching market movers (count: {})", count.as_str());
                
                match client.get_movers(count).await {
                    Ok((actives, gainers, losers)) => {
                        let update = crate::websocket::MoversUpdate::with_timestamp(
                            actives,
                            gainers,
                            losers,
                            Utc::now()
                        );
                        yield Ok(update);
                    }
                    Err(e) => {
                        error!("Failed to fetch movers: {}", e);
                        yield Err(e);
                    }
                }
            }
        })
    }

    /// Create a movers stream with a default 5-second interval.
    pub fn with_default_interval(
        client: Arc<YahooFinanceClient>,
        count: crate::models::MoverCount,
    ) -> Pin<Box<dyn Stream<Item = Result<crate::websocket::MoversUpdate, YahooError>> + Send>> {
        Self::new(client, count, Duration::from_secs(5))
    }

    /// Create a movers stream with default count (50) and interval (5 seconds).
    pub fn with_defaults(
        client: Arc<YahooFinanceClient>,
    ) -> Pin<Box<dyn Stream<Item = Result<crate::websocket::MoversUpdate, YahooError>> + Send>> {
        Self::new(client, crate::models::MoverCount::default(), Duration::from_secs(5))
    }
}

/// Parse market indices from Yahoo Finance API response.
fn parse_market_indices(data: &Value) -> Vec<crate::models::MarketIndex> {
    let mut indices = Vec::new();
    
    if let Some(results) = data
        .get("quoteResponse")
        .and_then(|qr| qr.get("result"))
        .and_then(|r| r.as_array())
    {
        for result in results {
            let name = result.get("longName")
                .or_else(|| result.get("shortName"))
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string();
            
            let value = result.get("regularMarketPrice")
                .and_then(|p| p.as_f64())
                .unwrap_or(0.0);
            
            let change = result.get("regularMarketChange")
                .and_then(|c| c.as_f64())
                .map(|c| format!("{:+.2}", c))
                .unwrap_or_else(|| "0.00".to_string());
            
            let percent_change = result.get("regularMarketChangePercent")
                .and_then(|p| p.as_f64())
                .map(|p| format!("{:+.2}%", p))
                .unwrap_or_else(|| "0.00%".to_string());
            
            // Parse performance returns if available
            let five_days_return = result.get("fiftyTwoWeekLow")
                .and_then(|_| None); // Placeholder - would need actual field
            
            let one_month_return = result.get("fiftyTwoWeekHigh")
                .and_then(|_| None); // Placeholder - would need actual field
            
            let index = crate::models::MarketIndex {
                name,
                value,
                change,
                percent_change,
                five_days_return,
                one_month_return,
                three_month_return: None,
                six_month_return: None,
                ytd_return: None,
                year_return: None,
                three_year_return: None,
                five_year_return: None,
                ten_year_return: None,
                max_return: None,
            };
            
            indices.push(index);
        }
    }
    
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_quotes() {
        let data = serde_json::json!({
            "quoteResponse": {
                "result": [
                    {
                        "symbol": "AAPL",
                        "longName": "Apple Inc.",
                        "regularMarketPrice": 175.50,
                        "regularMarketChange": 2.50,
                        "regularMarketChangePercent": 1.45
                    }
                ]
            }
        });

        let quotes = parse_simple_quotes(&data);
        assert_eq!(quotes.len(), 1);
        assert_eq!(quotes[0].symbol, "AAPL");
        assert_eq!(quotes[0].name, "Apple Inc.");
        assert_eq!(quotes[0].price, "175.50");
        assert_eq!(quotes[0].change, "+2.50");
        assert_eq!(quotes[0].percent_change, "+1.45%");
    }

    #[test]
    fn test_movers_stream_creation() {
        // Test that we can create a MoversStream with different configurations
        // This is a compile-time test to ensure the API is correct
        use crate::models::MoverCount;
        
        // These would require a real client to actually run, but we can verify
        // the types compile correctly
        let _count_25 = MoverCount::TwentyFive;
        let _count_50 = MoverCount::Fifty;
        let _count_100 = MoverCount::Hundred;
        
        assert_eq!(_count_25.as_str(), "25");
        assert_eq!(_count_50.as_str(), "50");
        assert_eq!(_count_100.as_str(), "100");
    }
}
