//! Market movers streaming examples using finance-query-core
//!
//! Demonstrates how to use MoversStream for real-time market movers updates
//! (most active, top gainers, top losers).

use finance_query_core::{
    FetchClient, MoverCount, MoversStream, YahooAuthManager, YahooFinanceClient, YahooError,
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
    
    // Prime authentication
    auth_manager.refresh().await?;
    
    Ok(Arc::new(client))
}

/// Stream market movers at a specified interval.
///
/// # Arguments
/// * `count` - Number of movers to return (25, 50, or 100)
/// * `interval_secs` - Polling interval in seconds
/// * `max_updates` - Maximum number of updates to receive (None for infinite)
///
/// # Example
/// ```rust,ignore
/// stream_movers(MoverCount::Fifty, 10, Some(5)).await?;
/// ```
pub async fn stream_movers(
    count: MoverCount,
    interval_secs: u64,
    max_updates: Option<usize>,
) -> Result<(), YahooError> {
    let client = create_client().await?;
    
    let mut stream = MoversStream::create(client, count, Duration::from_secs(interval_secs));
    
    println!("🔥 Starting market movers stream");
    println!("   Count: {} movers per category", count.as_str());
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
                
                // Show top 5 actives
                println!("  📊 Most Active:");
                for mover in update.actives.iter().take(5) {
                    println!(
                        "    {} - ${} {} ({})",
                        mover.symbol,
                        mover.price,
                        mover.change,
                        mover.percent_change
                    );
                }
                
                // Show top 5 gainers
                println!("  📈 Top Gainers:");
                for mover in update.gainers.iter().take(5) {
                    println!(
                        "    {} - ${} {} ({})",
                        mover.symbol,
                        mover.price,
                        mover.change,
                        mover.percent_change
                    );
                }
                
                // Show top 5 losers
                println!("  📉 Top Losers:");
                for mover in update.losers.iter().take(5) {
                    println!(
                        "    {} - ${} {} ({})",
                        mover.symbol,
                        mover.price,
                        mover.change,
                        mover.percent_change
                    );
                }
                println!();
            }
            Err(e) => {
                eprintln!("Error fetching movers: {}", e);
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

/// Stream market movers with default settings (50 movers, 5-second interval).
///
/// # Arguments
/// * `max_updates` - Maximum number of updates to receive (None for infinite)
pub async fn stream_movers_default(max_updates: Option<usize>) -> Result<(), YahooError> {
    stream_movers(MoverCount::Fifty, 5, max_updates).await
}

/// Stream only the top gainers.
///
/// # Arguments
/// * `count` - Number of gainers to fetch
/// * `interval_secs` - Polling interval in seconds
/// * `max_updates` - Maximum number of updates to receive
pub async fn stream_top_gainers(
    count: MoverCount,
    interval_secs: u64,
    max_updates: Option<usize>,
) -> Result<(), YahooError> {
    let client = create_client().await?;
    
    let mut stream = MoversStream::create(client, count, Duration::from_secs(interval_secs));
    
    println!("📈 Starting top gainers stream");
    println!("   Polling every {} seconds", interval_secs);
    println!();
    
    let mut update_count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                println!(
                    "Top Gainers at {}:",
                    update.timestamp.format("%H:%M:%S")
                );
                for (i, mover) in update.gainers.iter().take(10).enumerate() {
                    println!(
                        "  {}. {} ({}) - ${} {} ({})",
                        i + 1,
                        mover.symbol,
                        mover.name,
                        mover.price,
                        mover.change,
                        mover.percent_change
                    );
                }
                println!();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        
        update_count += 1;
        if let Some(max) = max_updates {
            if update_count >= max {
                break;
            }
        }
    }
    
    Ok(())
}

/// Stream only the top losers.
///
/// # Arguments
/// * `count` - Number of losers to fetch
/// * `interval_secs` - Polling interval in seconds
/// * `max_updates` - Maximum number of updates to receive
pub async fn stream_top_losers(
    count: MoverCount,
    interval_secs: u64,
    max_updates: Option<usize>,
) -> Result<(), YahooError> {
    let client = create_client().await?;
    
    let mut stream = MoversStream::create(client, count, Duration::from_secs(interval_secs));
    
    println!("📉 Starting top losers stream");
    println!("   Polling every {} seconds", interval_secs);
    println!();
    
    let mut update_count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                println!(
                    "Top Losers at {}:",
                    update.timestamp.format("%H:%M:%S")
                );
                for (i, mover) in update.losers.iter().take(10).enumerate() {
                    println!(
                        "  {}. {} ({}) - ${} {} ({})",
                        i + 1,
                        mover.symbol,
                        mover.name,
                        mover.price,
                        mover.change,
                        mover.percent_change
                    );
                }
                println!();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        
        update_count += 1;
        if let Some(max) = max_updates {
            if update_count >= max {
                break;
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_stream_movers() {
        let result = stream_movers(MoverCount::TwentyFive, 5, Some(2)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_stream_top_gainers() {
        let result = stream_top_gainers(MoverCount::TwentyFive, 5, Some(2)).await;
        assert!(result.is_ok());
    }
}
