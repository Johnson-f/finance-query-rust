//! Example: Fetch real stock quotes using finance-query-core
//!
//! Run with: cargo run --example fetch_quote -p finance-query-core

use finance_query_core::{FetchClient, YahooAuthManager, YahooFinanceClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing finance-query-core with real Yahoo Finance data...\n");

    // Optional proxy for Yahoo (set PROXY_URL to enable)
    let proxy = std::env::var("PROXY_URL").ok();

    // 1. Create the fetch client (with optional proxy)
    let fetch_client = Arc::new(FetchClient::new(proxy.clone())?);
    
    // 2. Create auth manager (shares the same proxy)
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(proxy, cookie_jar));
    
    // 3. Create Yahoo Finance client
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client.clone());

    // 4. Prime authentication (dual-path: basic crumb then consent/CSRF fallback)
    println!("📡 Authenticating with Yahoo Finance...");
    auth_manager.refresh().await?;
    println!("✅ Authentication successful!\n");

    // 5. Fetch a quote
    println!("📈 Fetching AAPL quote...");
    let quote = client.get_quote("AAPL").await?;
    println!("Raw response:\n{}\n", serde_json::to_string_pretty(&quote)?);

    // 6. Fetch simple quotes for multiple symbols
    println!("📊 Fetching simple quotes for AAPL, GOOGL, MSFT...");
    let symbols = vec!["AAPL", "GOOGL", "MSFT"];
    let quotes = client.get_simple_quotes(&symbols).await?;
    println!("Simple quotes response:\n{}\n", serde_json::to_string_pretty(&quotes)?);

    // 7. Search for a symbol
    println!("🔍 Searching for 'Tesla'...");
    let search_results = client.search("Tesla", 5).await?;
    println!("Search results:\n{}\n", serde_json::to_string_pretty(&search_results)?);

    // 8. Get historical data
    println!("📉 Fetching 1-month historical data for AAPL...");
    let chart = client.get_chart("AAPL", "1d", "1mo").await?;
    println!("Chart data (first 500 chars):\n{}...\n", 
        serde_json::to_string_pretty(&chart)?
            .chars()
            .take(500)
            .collect::<String>()
    );

    println!("✅ All tests passed! The crate is working correctly.");
    Ok(())
}
