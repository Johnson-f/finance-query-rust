use finance_query_core::{FetchClient, YahooAuthManager, YahooFinanceClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    auth_manager.refresh().await?;

    println!("Fetching market data...\n");

    // Get market status
    println!("=== US Market Status ===");
    match client.get_market_status("us_market").await {
        Ok(status) => {
            println!("Status: {}", status.status);
            println!("Is Open: {}", status.is_open());
            if let Some(tz) = &status.timezone_short {
                println!("Timezone: {}", tz);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Get market summary
    // Note: This endpoint may require specific parameters or may be deprecated
    println!("\n=== Market Summary ===");
    match client.get_market_summary("us_market").await {
        Ok(summary) => {
            if summary.indices.is_empty() {
                println!("No indices data available");
            } else {
                for index in summary.indices.iter().take(5) {
                    println!(
                        "{}: {:.2} ({:+.2}%)",
                        index.short_name, index.price, index.percent_change
                    );
                }
            }
        }
        Err(e) => println!("Market summary not available: {}", e),
    }

    // Get industry data
    // Note: Industry keys may vary - try different keys like "semiconductors", "software-application"
    println!("\n=== Industry Data ===");
    for industry_key in &["semiconductors", "software-application", "consumer-electronics"] {
        match client.get_industry(industry_key).await {
            Ok(industry) => {
                println!("\nIndustry: {} (key: {})", industry.name, industry_key);
                if let Some(sector) = &industry.sector_name {
                    println!("Sector: {}", sector);
                }
                
                if !industry.top_performing_companies.is_empty() {
                    println!("Top Performing Companies:");
                    for company in industry.top_performing_companies.iter().take(3) {
                        let ytd = company.ytd_return.map(|r| format!("{:+.2}%", r * 100.0)).unwrap_or_default();
                        println!("  {} ({}) - YTD: {}", company.name, company.symbol, ytd);
                    }
                }
                break; // Found one that works
            }
            Err(_) => continue, // Try next industry key
        }
    }
    println!("\nNote: Some market endpoints may require different authentication or may be deprecated.");

    Ok(())
}
