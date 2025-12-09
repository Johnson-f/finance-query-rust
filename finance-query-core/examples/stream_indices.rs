use finance_query_core::{FetchClient, IndexStream, YahooAuthManager, YahooFinanceClient};
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

    println!("Starting real-time index stream...");
    println!("Streaming S&P 500, Dow Jones, and NASDAQ every 5 seconds");
    println!("Press Ctrl+C to stop\n");

    // Create stream for major US indices
    let mut stream = IndexStream::us_major_indices(client.clone(), Duration::from_secs(5));

    // Stream updates
    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(indices) => {
                count += 1;
                println!("=== Update #{} ===", count);

                for index in indices {
                    println!(
                        "{}: {:.2} ({}) {}",
                        index.name, index.value, index.change, index.percent_change
                    );
                }
                println!();

                // Stop after 10 updates for demo purposes
                if count >= 10 {
                    println!("Demo complete - received 10 updates");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error fetching indices: {}", e);
            }
        }
    }

    Ok(())
}
