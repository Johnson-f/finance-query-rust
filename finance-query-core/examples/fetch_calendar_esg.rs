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
    println!("Fetching calendar and ESG data for {}...\n", symbol);

    // Get calendar events
    println!("=== Calendar Events ===");
    match client.get_calendar(symbol).await {
        Ok(calendar) => {
            if let Some(date) = calendar.earnings_date {
                println!("Next Earnings Date: {}", date.format("%Y-%m-%d"));
            }
            if let Some(date) = calendar.ex_dividend_date {
                println!("Ex-Dividend Date: {}", date.format("%Y-%m-%d"));
            }
            if let Some(date) = calendar.dividend_date {
                println!("Dividend Date: {}", date.format("%Y-%m-%d"));
            }
        }
        Err(e) => println!("Error fetching calendar: {}", e),
    }

    // Get SEC filings
    println!("\n=== SEC Filings (last 5) ===");
    match client.get_sec_filings(symbol).await {
        Ok(filings) => {
            for filing in filings.filings.iter().take(5) {
                println!(
                    "{}: {} - {}",
                    filing.date.format("%Y-%m-%d"),
                    filing.filing_type,
                    filing.title
                );
            }
        }
        Err(e) => println!("Error fetching SEC filings: {}", e),
    }

    // Get ESG/Sustainability scores
    // Note: ESG data may not be available for all stocks or may require authentication
    println!("\n=== ESG/Sustainability Scores ===");
    match client.get_sustainability(symbol).await {
        Ok(esg) => {
            if esg.has_data() {
                if let Some(score) = esg.total_esg {
                    println!("Total ESG Score: {:.1}", score);
                }
                if let Some(rating) = esg.rating() {
                    println!("ESG Rating: {}", rating);
                }
                if let Some(env) = esg.environment_score {
                    println!("Environment Score: {:.1}", env);
                }
                if let Some(social) = esg.social_score {
                    println!("Social Score: {:.1}", social);
                }
                if let Some(gov) = esg.governance_score {
                    println!("Governance Score: {:.1}", gov);
                }
                if let Some(peer) = &esg.peer_group {
                    println!("Peer Group: {}", peer);
                }
            } else {
                println!("No ESG data available for {}", symbol);
            }
        }
        Err(e) => println!("ESG data not available: {} (Yahoo may have deprecated this endpoint)", e),
    }

    Ok(())
}
