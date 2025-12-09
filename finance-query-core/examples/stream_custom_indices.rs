use finance_query_core::{FetchClient, IndexStream, YahooAuthManager, YahooFinanceClient};
use futures_util::StreamExt;
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get index symbols from command line arguments or use defaults
    let args: Vec<String> = env::args().collect();

    let index_symbols = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        // Default to major US indices
        vec![
            "^GSPC".to_string(), // S&P 500
            "^DJI".to_string(),  // Dow Jones
            "^IXIC".to_string(), // NASDAQ
        ]
    };

    println!("Streaming indices: {:?}", index_symbols);
    println!("Usage: cargo run --example stream_custom_indices [symbols...]");
    println!("Example: cargo run --example stream_custom_indices ^GSPC ^DJI ^IXIC ^FTSE ^N225\n");

    // Initialize client
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = Arc::new(YahooFinanceClient::new(auth_manager.clone(), fetch_client));

    // Authenticate
    auth_manager.refresh().await?;

    println!("Starting real-time stream (updates every 5 seconds)...");
    println!("Press Ctrl+C to stop\n");

    // Create custom index stream
    let mut stream = IndexStream::create(client.clone(), index_symbols, Duration::from_secs(5));

    // Stream updates
    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(indices) => {
                count += 1;
                let timestamp = chrono::Utc::now().format("%H:%M:%S");
                println!("=== Update #{} at {} UTC ===", count, timestamp);

                for index in indices {
                    // Color code based on change
                    let indicator = if index.change.starts_with('+') {
                        "📈"
                    } else if index.change.starts_with('-') {
                        "📉"
                    } else {
                        "➡️"
                    };

                    println!(
                        "{} {}: {:.2} ({}) {}",
                        indicator, index.name, index.value, index.change, index.percent_change
                    );
                }
                println!();
            }
            Err(e) => {
                eprintln!("❌ Error fetching indices: {}", e);
            }
        }
    }

    Ok(())
}
