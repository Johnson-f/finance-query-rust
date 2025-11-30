use finance_query_core::{FetchClient, YahooAuthManager, YahooFinanceClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    auth_manager.refresh().await?;

    let symbol = "AAPL";
    println!("Fetching option data for {}...\n", symbol);

    // Get available expirations
    let expirations = client.get_option_expirations(symbol).await?;
    println!("=== Available Expirations ===");
    for (i, exp) in expirations.expirations.iter().enumerate().take(5) {
        println!("{}. {}", i + 1, exp);
    }

    // Get option chain for nearest expiration
    if let Some(first_exp) = expirations.expirations.first() {
        println!("\n=== Option Chain for {} ===", first_exp);
        let chain = client.get_option_chain(symbol, Some(first_exp)).await?;

        if let Some(price) = chain.underlying_price {
            println!("Underlying Price: ${:.2}", price);
        }

        println!("\n--- Calls (first 5) ---");
        for call in chain.calls.iter().take(5) {
            println!(
                "Strike: ${:.2} | Last: ${:.2} | Bid: ${:.2} | Ask: ${:.2} | IV: {:.2}%",
                call.strike,
                call.last_price,
                call.bid,
                call.ask,
                call.implied_volatility * 100.0
            );
        }

        println!("\n--- Puts (first 5) ---");
        for put in chain.puts.iter().take(5) {
            println!(
                "Strike: ${:.2} | Last: ${:.2} | Bid: ${:.2} | Ask: ${:.2} | IV: {:.2}%",
                put.strike,
                put.last_price,
                put.bid,
                put.ask,
                put.implied_volatility * 100.0
            );
        }
    }

    Ok(())
}
