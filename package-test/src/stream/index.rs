//! Market index streaming using finance-query-core
//!
//! Provides IndexStream for real-time market index updates.

use finance_query_core::{
    FetchClient, IndexStream, YahooAuthManager, YahooFinanceClient, YahooError,
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

/// Stream market indices at a specified interval.
///
/// # Arguments
/// * `symbols` - List of index symbols to track (e.g., "^GSPC", "^DJI", "^IXIC")
/// * `interval_secs` - Polling interval in seconds
/// * `max_updates` - Maximum number of updates to receive (None for infinite)
pub async fn stream_indices(
    symbols: Vec<&str>,
    interval_secs: u64,
    max_updates: Option<usize>,
) -> Result<(), YahooError> {
    let client = create_client().await?;
    
    let symbol_strings: Vec<String> = symbols.iter().map(|s| s.to_string()).collect();
    let mut stream = IndexStream::create(client, symbol_strings, Duration::from_secs(interval_secs));
    
    println!("📊 Starting market index stream");
    println!("   Indices: {:?}", symbols);
    println!("   Polling every {} seconds", interval_secs);
    if let Some(max) = max_updates {
        println!("   Will stop after {} updates", max);
    }
    println!();

    let mut update_count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(indices) => {
                println!(
                    "Update #{} at {}",
                    update_count + 1,
                    chrono::Utc::now().format("%H:%M:%S")
                );
                
                for index in &indices {
                    let emoji = if index.change.starts_with('+') { "📈" } else { "📉" };
                    println!(
                        "  {} {} - {:.2} {} ({})",
                        emoji,
                        index.name,
                        index.value,
                        index.change,
                        index.percent_change
                    );
                }
                println!();
            }
            Err(e) => {
                eprintln!("Error fetching indices: {}", e);
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