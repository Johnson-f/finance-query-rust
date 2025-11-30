//! Example: Using WebSocket types for real-time streaming
//!
//! This demonstrates how to use the WebSocket types from finance-query-core
//! to build a streaming quote service.
//!
//! Run with: cargo run --example websocket_types -p finance-query-core

use finance_query_core::{
    FetchClient, YahooAuthManager, YahooFinanceClient,
    QuotesUpdate, SimpleQuote, MarketHours, MovingAverageUpdate,
};
use chrono::Utc;
use serde_json::Value;
use std::sync::Arc;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WebSocket Types Demo - Simulating Real-Time Streaming\n");

    // Set up the client
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client.clone());

    // Prime auth
    println!("Authenticating...");
    auth_manager.refresh().await?;
    println!("Ready!\n");

    // Simulate streaming quotes every 3 seconds (3 updates)
    println!("Simulating quote stream for AAPL, GOOGL, MSFT...\n");
    
    let symbols = vec!["AAPL", "GOOGL", "MSFT"];
    let mut ticker = interval(Duration::from_secs(3));
    
    for i in 1..=3 {
        ticker.tick().await;
        
        // Fetch fresh quotes
        let response = client.get_simple_quotes(&symbols).await?;
        
        // Parse into SimpleQuote structs
        let quotes: Vec<SimpleQuote> = parse_simple_quotes(&response);
        
        // Create a QuotesUpdate (this is what you'd send over WebSocket)
        let update = QuotesUpdate::multiple(quotes);
        
        println!("--- Update #{} ({} quotes) ---", i, update.len());
        println!("Timestamp: {}", update.timestamp);
        
        for quote in &update.quotes {
            println!("  {} ({}): ${} | {} ({})", 
                quote.symbol, 
                quote.name,
                quote.price, 
                quote.change,
                quote.percent_change
            );
        }
        
        // Show JSON that would be sent over WebSocket
        println!("\nJSON payload size: {} bytes", serde_json::to_string(&update)?.len());
        println!();
    }

    // Demo: MarketHours update
    println!("--- MarketHours Update Demo ---");
    let market_hours = MarketHours {
        status: "closed".to_string(),
        reason: Some("Weekend".to_string()),
        timestamp: Utc::now(),
    };
    println!("Market Status: {} ({})", market_hours.status, market_hours.reason.as_deref().unwrap_or(""));
    println!("JSON: {}\n", serde_json::to_string(&market_hours)?);

    // Demo: MovingAverageUpdate
    println!("--- MovingAverageUpdate Demo ---");
    let ma_update = MovingAverageUpdate {
        symbol: "AAPL".to_string(),
        indicator_type: "SMA".to_string(),
        period: 20,
        value: 275.50,
        timestamp: Utc::now(),
    };
    println!("{} {} ({}): {}", ma_update.symbol, ma_update.indicator_type, ma_update.period, ma_update.value);
    println!("JSON: {}\n", serde_json::to_string(&ma_update)?);

    // Demo: QuotesUpdate helper methods
    println!("--- QuotesUpdate Helper Methods ---");
    let response = client.get_simple_quotes(&["AAPL", "TSLA"]).await?;
    let quotes = parse_simple_quotes(&response);
    let update = QuotesUpdate::multiple(quotes);
    
    println!("Contains AAPL: {}", update.contains_symbol("AAPL"));
    println!("Contains MSFT: {}", update.contains_symbol("MSFT"));
    if let Some(aapl) = update.get_quote("AAPL") {
        println!("AAPL price: ${}", aapl.price);
    }
    println!("Total quotes: {}", update.len());
    println!("Is empty: {}", update.is_empty());

    println!("\nDone!");
    Ok(())
}

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
