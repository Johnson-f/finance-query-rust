//! Quote streaming using finance-query-core
//!
//! Provides QuoteStream and SingleQuoteStream for real-time stock quote updates.

use finance_query_core::{
    FetchClient, QuoteStream, SingleQuoteStream, YahooAuthManager, YahooFinanceClient, YahooError,
};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;

/// Creates a configured YahooFinanceClient wrapped in Arc for streaming
async fn create_client() -> Result<Arc<YahooFinanceClient>, YahooError> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);
    
    auth_manager.refresh().await?;
    
    Ok(Arc::new(client))
}

/// Stream quotes for multiple symbols at a specified interval.
///
/// # Arguments
/// * `symbols` - List of stock symbols to track
/// * `interval_secs` - Polling interval in seconds
/// * `max_updates` - Maximum number of updates to receive (None for infinite)
pub async fn stream_quotes(
    symbols: Vec<&str>,
    interval_secs: u64,
    max_updates: Option<usize>,
) -> Result<(), YahooError> {
    let client = create_client().await?;
    
    let symbol_strings: Vec<String> = symbols.iter().map(|s| s.to_string()).collect();
    let mut stream = QuoteStream::create(client, symbol_strings, Duration::from_secs(interval_secs));
    
    println!("📈 Starting quote stream");
    println!("   Symbols: {:?}", symbols);
    println!("   Polling every {} seconds", interval_secs);
    if let Some(max) = max_updates {
        println!("   Will stop after {} updates", max);
    }
    println!();

    let mut update_count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                println!(
                    "Update #{} at {}",
                    update_count + 1,
                    update.timestamp.format("%H:%M:%S")
                );
                
                for quote in &update.quotes {
                    println!(
                        "  {} ({}) - ${} {} ({})",
                        quote.symbol,
                        quote.name,
                        quote.price,
                        quote.change,
                        quote.percent_change
                    );
                    
                    if let Some(pre) = &quote.pre_market_price {
                        println!("    Pre-market: ${}", pre);
                    }
                    if let Some(after) = &quote.after_hours_price {
                        println!("    After-hours: ${}", after);
                    }
                }
                println!();
            }
            Err(e) => {
                eprintln!("Error fetching quotes: {}", e);
            }
        }
        
        update_count += 1;
        if let Some(max) = max_updates {
            if update_count >= max {
                println!("Reached maximum updates ({}), stopping stream.", max);
                break;
            }
        }
    }
    
    Ok(())
}

/// Stream a single stock quote at a specified interval.
///
/// # Arguments
/// * `symbol` - Stock symbol to track
/// * `interval_secs` - Polling interval in seconds
/// * `max_updates` - Maximum number of updates to receive (None for infinite)
pub async fn stream_single_quote(
    symbol: &str,
    interval_secs: u64,
    max_updates: Option<usize>,
) -> Result<(), YahooError> {
    let client = create_client().await?;
    
    let mut stream = SingleQuoteStream::create(
        client,
        symbol.to_string(),
        Duration::from_secs(interval_secs),
    );
    
    println!("📊 Starting single quote stream for {}", symbol);
    println!("   Polling every {} seconds", interval_secs);
    if let Some(max) = max_updates {
        println!("   Will stop after {} updates", max);
    }
    println!();
    
    let mut update_count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(quote) => {
                println!(
                    "[{}] {} ({}) - ${} {} ({})",
                    chrono::Utc::now().format("%H:%M:%S"),
                    quote.symbol,
                    quote.name,
                    quote.price,
                    quote.change,
                    quote.percent_change
                );
                
                if let Some(pre) = &quote.pre_market_price {
                    println!("  Pre-market: ${}", pre);
                }
                if let Some(after) = &quote.after_hours_price {
                    println!("  After-hours: ${}", after);
                }
            }
            Err(e) => {
                eprintln!("Error fetching quote: {}", e);
            }
        }
        
        update_count += 1;
        if let Some(max) = max_updates {
            if update_count >= max {
                println!("\nReached maximum updates ({}), stopping stream.", max);
                break;
            }
        }
    }
    
    Ok(())
}
