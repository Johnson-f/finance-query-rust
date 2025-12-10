//! Example: Fetch real stock quotes using finance-query-core
//!
//! Run with: cargo run --example fetch_quote -p finance-query-core

use finance_query_core::{
    FetchClient, LogoFetcher, QuoteStream, YahooAuthManager, YahooFinanceClient,
};
use futures_util::StreamExt;
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

    // 3. Create Yahoo Finance client (wrap in Arc for sharing)
    let client = Arc::new(YahooFinanceClient::new(auth_manager.clone(), fetch_client.clone()));

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
    println!(
        "Simple quotes response:\n{}\n",
        serde_json::to_string_pretty(&quotes)?
    );

    // 7. Fetch logos for symbols
    println!("🎨 Fetching logos (logo.dev) for AAPL, GOOGL, MSFT...");
    let logo_fetcher = Arc::new(LogoFetcher::new(fetch_client.clone()));

    for symbol in &symbols {
        match logo_fetcher.fetch_logo(symbol, None).await {
            Some(logo_url) => println!("✅ {} logo (logo.dev): {}", symbol, logo_url),
            None => println!("❌ {} logo not found via logo.dev", symbol),
        }
    }

    println!();

    // 8. Use streaming quotes with automatic logo enrichment
    println!("🌊 Fetching streaming quotes with automatic logo enrichment...");

    // Create a quote stream and get the first batch
    let mut stream = QuoteStream::create(
        client.clone(),
        symbols.iter().map(|s| s.to_string()).collect(),
        std::time::Duration::from_secs(30), // Long interval since we only want one batch
    );

    if let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                println!("📈 Received {} quotes with logos:", update.quotes.len());
                for quote in update.quotes {
                    let logo_status = if quote.logo.is_some() { "✅" } else { "❌" };
                    println!(
                        "  {}: {} ({}) - logo: {}",
                        quote.symbol,
                        quote.price,
                        quote.change,
                        logo_status
                    );
                }
                println!();
            }
            Err(e) => println!("❌ Stream error: {}", e),
        }
    }

    // 9. Search for a symbol
    println!("🔍 Searching for 'Tesla'...");
    let search_results = client.search("Tesla", 5).await?;
    println!(
        "Search results:\n{}\n",
        serde_json::to_string_pretty(&search_results)?
    );

    // 10. Get historical data
    println!("📉 Fetching 1-month historical data for AAPL...");
    let chart = client.get_chart("AAPL", "1d", "1mo").await?;
    println!(
        "Chart data (first 500 chars):\n{}...\n",
        serde_json::to_string_pretty(&chart)?
            .chars()
            .take(500)
            .collect::<String>()
    );

    println!("✅ All tests passed! The crate is working correctly.");
    Ok(())
}