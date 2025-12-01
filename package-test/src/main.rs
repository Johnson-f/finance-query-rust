mod quotes;

use quotes::{
    get_detailed_quote, get_simple_quotes, get_similar_quotes, get_logo_url,
    DetailedQuoteResponse, SimpleQuotesResponse, SimilarQuotesResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing quote functions with typed responses...\n");

    // Test detailed quote with typed parsing
    println!("📈 Fetching detailed quote for AAPL...");
    match get_detailed_quote("AAPL").await {
        Ok(json) => {
            // Parse into typed struct
            match serde_json::from_value::<DetailedQuoteResponse>(json.clone()) {
                Ok(detailed) => {
                    if let Some(results) = &detailed.quote_summary.result {
                        if let Some(result) = results.first() {
                            // Price info
                            if let Some(price) = &result.price {
                                println!("Symbol: {}", price.symbol.as_deref().unwrap_or("N/A"));
                                println!("Name: {}", price.long_name.as_deref().unwrap_or("N/A"));
                                if let Some(p) = &price.regular_market_price {
                                    println!("Price: {}", p.formatted().unwrap_or("N/A"));
                                }
                                if let Some(c) = &price.regular_market_change {
                                    println!("Change: {}", c.formatted().unwrap_or("N/A"));
                                }
                                if let Some(cp) = &price.regular_market_change_percent {
                                    println!("Change %: {}", cp.formatted().unwrap_or("N/A"));
                                }
                                if let Some(mc) = &price.market_cap {
                                    println!("Market Cap: {}", mc.formatted().unwrap_or("N/A"));
                                }
                            }
                            // Asset profile
                            if let Some(profile) = &result.asset_profile {
                                println!("Sector: {}", profile.sector.as_deref().unwrap_or("N/A"));
                                println!("Industry: {}", profile.industry.as_deref().unwrap_or("N/A"));
                                if let Some(employees) = profile.full_time_employees {
                                    println!("Employees: {}", employees);
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("Parse error: {} - Raw JSON available", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Test simple quotes with typed parsing
    println!("📊 Fetching simple quotes for AAPL, GOOGL, MSFT...");
    match get_simple_quotes(&["AAPL", "GOOGL", "MSFT"]).await {
        Ok(json) => {
            match serde_json::from_value::<SimpleQuotesResponse>(json.clone()) {
                Ok(response) => {
                    if let Some(results) = &response.quote_response.result {
                        for quote in results {
                            println!("---");
                            println!("Symbol: {}", quote.symbol);
                            println!("Name: {}", quote.long_name.as_deref().unwrap_or("N/A"));
                            if let Some(price) = quote.regular_market_price {
                                println!("Price: ${:.2}", price);
                            }
                            if let Some(change) = quote.regular_market_change {
                                let pct = quote.regular_market_change_percent.unwrap_or(0.0);
                                println!("Change: {:.2} ({:.2}%)", change, pct);
                            }
                            if let Some(mc) = quote.market_cap {
                                println!("Market Cap: ${}", format_large_number(mc));
                            }
                        }
                    }
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Test similar quotes with typed parsing
    println!("🔗 Fetching similar quotes to AAPL...");
    match get_similar_quotes("AAPL", 5).await {
        Ok(json) => {
            match serde_json::from_value::<SimilarQuotesResponse>(json.clone()) {
                Ok(response) => {
                    if let Some(results) = &response.finance.result {
                        for rec in results {
                            println!("Similar to {}:", rec.symbol);
                            for sym in &rec.recommended_symbols {
                                println!("  - {} (score: {:.2})", sym.symbol, sym.score);
                            }
                        }
                    }
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Test logo URL
    println!("🖼️ Fetching logo URL for AAPL...");
    match get_logo_url("AAPL").await {
        Ok(Some(url)) => println!("Logo URL: {}", url),
        Ok(None) => println!("No logo URL found"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n✅ All quote functions tested!");
    Ok(())
}

fn format_large_number(n: i64) -> String {
    if n >= 1_000_000_000_000 {
        format!("{:.2}T", n as f64 / 1_000_000_000_000.0)
    } else if n >= 1_000_000_000 {
        format!("{:.2}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.2}M", n as f64 / 1_000_000.0)
    } else {
        format!("{}", n)
    }
}
