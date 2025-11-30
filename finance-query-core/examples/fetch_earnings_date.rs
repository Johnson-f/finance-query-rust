use finance_query_core::{FetchClient, YahooAuthManager, YahooFinanceClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    auth_manager.refresh().await?;

    // Fetch for multiple symbols
    for symbol in &["AAPL", "MSFT", "GOOGL"] {
        match client.get_calendar(symbol).await {
            Ok(calendar) => {
                println!("\n{} Calendar:", symbol);
                
                if let Some(date) = calendar.earnings_date {
                    println!("  Earnings: {}", date.format("%Y-%m-%d %H:%M UTC"));
                } else {
                    println!("  Earnings: Not scheduled");
                }
                
                if let Some(date) = calendar.ex_dividend_date {
                    println!("  Ex-Dividend: {}", date.format("%Y-%m-%d"));
                }
            }
            Err(e) => println!("{}: Error - {}", symbol, e),
        }
    }

    Ok(())
}
