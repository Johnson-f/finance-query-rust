//! Example: Real-time quote streaming
//!
//! This demonstrates how to use QuoteStream for continuous quote updates.
//!
//! Run with: cargo run --example quote_stream -p finance-query-core

use finance_query_core::{
    FetchClient, QuoteStream, SingleQuoteStream, YahooAuthManager, YahooFinanceClient,
};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Quote Streaming Demo\n");

    // Set up the client
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = Arc::new(YahooFinanceClient::new(auth_manager.clone(), fetch_client));

    // Prime auth
    println!("Authenticating...");
    auth_manager.refresh().await?;
    println!("Ready!\n");

    // Demo 1: Stream multiple quotes
    println!("=== Multi-Quote Stream (3 updates, 2-second interval) ===\n");

    let symbols = vec!["AAPL".to_string(), "GOOGL".to_string(), "MSFT".to_string()];
    let mut stream = QuoteStream::create(client.clone(), symbols, Duration::from_secs(2));

    let mut count = 0;
    while let Some(result) = stream.next().await {
        count += 1;
        match result {
            Ok(update) => {
                println!(
                    "Update #{} - {} quotes at {}",
                    count,
                    update.len(),
                    update.timestamp.format("%H:%M:%S")
                );
                for quote in &update.quotes {
                    println!(
                        "  {} ({}): ${} | {} ({})",
                        quote.symbol,
                        truncate(&quote.name, 15),
                        quote.price,
                        quote.change,
                        quote.percent_change
                    );
                }
                println!();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        if count >= 3 {
            break;
        }
    }

    // Demo 2: Stream single quote
    println!("=== Single Quote Stream (TSLA, 3 updates) ===\n");

    let mut single_stream =
        SingleQuoteStream::create(client.clone(), "TSLA".to_string(), Duration::from_secs(2));

    let mut count = 0;
    while let Some(result) = single_stream.next().await {
        count += 1;
        match result {
            Ok(quote) => {
                println!(
                    "#{} {} - ${} | {} ({})",
                    count, quote.symbol, quote.price, quote.change, quote.percent_change
                );
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        if count >= 3 {
            break;
        }
    }

    println!("\nDone!");
    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
