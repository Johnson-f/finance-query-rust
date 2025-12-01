use finance_query_core::{FetchClient, MoverCount, MoversStream, YahooAuthManager, YahooFinanceClient};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = Arc::new(YahooFinanceClient::new(auth_manager.clone(), fetch_client));

    // Authenticate
    auth_manager.refresh().await?;

    println!("Starting real-time market movers stream...");
    println!("Streaming top 50 actives, gainers, and losers every 5 seconds");
    println!("Press Ctrl+C to stop\n");

    // Create stream for market movers
    let mut stream = MoversStream::new(client.clone(), MoverCount::Fifty, Duration::from_secs(5));

    // Stream updates
    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                count += 1;
                println!("=== Update #{} ===", count);
                
                // Show top 5 from each category
                println!("\n📈 Top 5 Gainers:");
                for (i, mover) in update.gainers.iter().take(5).enumerate() {
                    println!(
                        "  {}. {} ({}): {} {}",
                        i + 1,
                        mover.symbol,
                        mover.name,
                        mover.price,
                        mover.percent_change
                    );
                }

                println!("\n📉 Top 5 Losers:");
                for (i, mover) in update.losers.iter().take(5).enumerate() {
                    println!(
                        "  {}. {} ({}): {} {}",
                        i + 1,
                        mover.symbol,
                        mover.name,
                        mover.price,
                        mover.percent_change
                    );
                }

                println!("\n🔥 Top 5 Most Active:");
                for (i, mover) in update.actives.iter().take(5).enumerate() {
                    println!(
                        "  {}. {} ({}): {} {}",
                        i + 1,
                        mover.symbol,
                        mover.name,
                        mover.price,
                        mover.percent_change
                    );
                }
                
                println!("\nTimestamp: {}", update.timestamp);
                println!();
                
                // Stop after 5 updates for demo purposes
                if count >= 5 {
                    println!("Demo complete - received 5 updates");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error fetching movers: {}", e);
            }
        }
    }

    Ok(())
}