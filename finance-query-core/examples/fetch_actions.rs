use finance_query_core::{FetchClient, YahooAuthManager, YahooFinanceClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Optional proxy for Yahoo (set PROXY_URL to enable)
    let proxy = std::env::var("PROXY_URL").ok();

    // Initialize client with shared cookie jar
    let fetch_client = Arc::new(FetchClient::new(proxy.clone())?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    // Pass the same proxy to the auth manager so both auth and data requests share config
    let auth_manager = Arc::new(YahooAuthManager::new(proxy, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    // Authenticate (uses new dual-path auth: basic crumb, then consent/CSRF fallback)
    auth_manager.refresh().await?;

    // Fetch all actions
    println!("Fetching stock actions for AAPL...\n");
    let actions = client.get_actions("AAPL", "5y").await?;

    // Display dividends
    println!("=== Dividends (last 10) ===");
    for dividend in actions.dividends.iter().rev().take(10) {
        println!(
            "{}: ${:.4}",
            dividend.date.format("%Y-%m-%d"),
            dividend.amount
        );
    }
    println!("\nTotal dividends: ${:.2}", actions.total_dividends());

    // Display splits
    println!("\n=== Stock Splits ===");
    if actions.splits.is_empty() {
        println!("No splits in the last 5 years");
    } else {
        for split in &actions.splits {
            println!(
                "{}: {} ({}:{})",
                split.date.format("%Y-%m-%d"),
                split.split_ratio,
                split.numerator,
                split.denominator
            );
        }
    }

    // Display capital gains
    println!("\n=== Capital Gains ===");
    if actions.capital_gains.is_empty() {
        println!("No capital gains (not an ETF/Mutual Fund)");
    } else {
        for gain in &actions.capital_gains {
            println!("{}: ${:.4}", gain.date.format("%Y-%m-%d"), gain.amount);
        }
    }

    Ok(())
}
