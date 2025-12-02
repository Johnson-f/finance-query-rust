mod financial;
mod historical;
mod quotes;

use financial::{get_balance_sheet, get_cash_flow, get_income_statement, Frequency};
use historical::{get_chart, get_historical_data, get_historical_data_by_date, ChartResponse};
use quotes::{
    get_detailed_quote, get_logo_url, get_similar_quotes, get_simple_quotes,
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
    println!();

    // ========================================================================
    // Historical Data Tests
    // ========================================================================

    println!("📅 Fetching historical data for AAPL (1 month, daily)...");
    match get_historical_data("AAPL", "1d", "1mo").await {
        Ok(data) => {
            println!("Symbol: {}", data.symbol);
            println!("Currency: {}", data.currency.as_deref().unwrap_or("N/A"));
            println!("Exchange: {}", data.exchange.as_deref().unwrap_or("N/A"));
            println!("Bars: {}", data.len());
            
            if let Some(first) = data.first() {
                println!("First bar: {} - Open: ${:.2}", 
                    first.date_string(), 
                    first.open.unwrap_or(0.0));
            }
            if let Some(last) = data.last() {
                println!("Last bar: {} - Close: ${:.2}", 
                    last.date_string(), 
                    last.close.unwrap_or(0.0));
            }
            
            if let Some(change) = data.price_change() {
                println!("Price change: ${:.2}", change);
            }
            if let Some(pct) = data.percent_change() {
                println!("Percent change: {:.2}%", pct);
            }
            if let Some(high) = data.high() {
                println!("Period high: ${:.2}", high);
            }
            if let Some(low) = data.low() {
                println!("Period low: ${:.2}", low);
            }
            println!("Total volume: {}", format_large_number(data.total_volume()));
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Test raw chart data with typed parsing
    println!("📉 Fetching raw chart data for MSFT (5 days, hourly)...");
    match get_chart("MSFT", "1h", "5d").await {
        Ok(json) => {
            match serde_json::from_value::<ChartResponse>(json.clone()) {
                Ok(response) => {
                    if let Some(results) = &response.chart.result {
                        if let Some(chart) = results.first() {
                            println!("Symbol: {}", chart.meta.symbol.as_deref().unwrap_or("N/A"));
                            println!("Timezone: {}", chart.meta.timezone.as_deref().unwrap_or("N/A"));
                            println!("Granularity: {}", chart.meta.data_granularity.as_deref().unwrap_or("N/A"));
                            
                            if let Some(timestamps) = &chart.timestamp {
                                println!("Data points: {}", timestamps.len());
                            }
                            
                            if let Some(price) = chart.meta.regular_market_price {
                                println!("Current price: ${:.2}", price);
                            }
                            if let Some(prev) = chart.meta.chart_previous_close {
                                println!("Previous close: ${:.2}", prev);
                            }
                            
                            if let Some(ranges) = &chart.meta.valid_ranges {
                                println!("Valid ranges: {}", ranges.join(", "));
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

    // Test historical data by date range
    println!("📆 Fetching historical data by date range (GOOGL, 2024-06-01 to 2024-06-30)...");
    match get_historical_data_by_date("GOOGL", "1d", "2024-06-01", "2024-06-30").await {
        Ok(data) => {
            println!("Symbol: {}", data.symbol);
            println!("Bars: {}", data.len());
            
            // Show first 5 bars
            println!("First 5 bars:");
            for bar in data.bars.iter().take(5) {
                println!("  {} | O: ${:.2} H: ${:.2} L: ${:.2} C: ${:.2} V: {}",
                    bar.date_string(),
                    bar.open.unwrap_or(0.0),
                    bar.high.unwrap_or(0.0),
                    bar.low.unwrap_or(0.0),
                    bar.close.unwrap_or(0.0),
                    bar.volume.unwrap_or(0));
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // ========================================================================
    // Financial Statements Tests
    // ========================================================================

    println!("�  Fetching income statement for AAPL (5 years, annual)...");
    match get_income_statement("AAPL", Frequency::Annual, 5).await {
        Ok(income) => {
            println!("Symbol: {}", income.symbol);
            println!("Statement: {}", income.statement_type);
            println!("Frequency: {}", income.frequency);
            println!("Metrics available: {}", income.metrics.len());

            // Show key metrics
            if let Some(revenue) = income.latest("TotalRevenue") {
                println!("Latest Total Revenue: ${}", format_large_number(revenue as i64));
            }
            if let Some(net_income) = income.latest("NetIncome") {
                println!("Latest Net Income: ${}", format_large_number(net_income as i64));
            }
            if let Some(eps) = income.latest("BasicEPS") {
                println!("Latest Basic EPS: ${:.2}", eps);
            }
            if let Some(ebitda) = income.latest("EBITDA") {
                println!("Latest EBITDA: ${}", format_large_number(ebitda as i64));
            }

            // Show revenue history
            if let Some(revenue_history) = income.get_metric("TotalRevenue") {
                println!("Revenue history:");
                for point in revenue_history.iter().take(5) {
                    println!(
                        "  {} ({}): ${}",
                        point.date,
                        point.period_type,
                        format_large_number(point.value as i64)
                    );
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📋 Fetching balance sheet for AAPL (5 years, annual)...");
    match get_balance_sheet("AAPL", Frequency::Annual, 5).await {
        Ok(balance) => {
            println!("Symbol: {}", balance.symbol);
            println!("Metrics available: {}", balance.metrics.len());

            if let Some(assets) = balance.latest("TotalAssets") {
                println!("Latest Total Assets: ${}", format_large_number(assets as i64));
            }
            if let Some(liabilities) = balance.latest("TotalLiabilitiesNetMinorityInterest") {
                println!(
                    "Latest Total Liabilities: ${}",
                    format_large_number(liabilities as i64)
                );
            }
            if let Some(equity) = balance.latest("StockholdersEquity") {
                println!(
                    "Latest Stockholders Equity: ${}",
                    format_large_number(equity as i64)
                );
            }
            if let Some(cash) = balance.latest("CashAndCashEquivalents") {
                println!("Latest Cash: ${}", format_large_number(cash as i64));
            }
            if let Some(debt) = balance.latest("TotalDebt") {
                println!("Latest Total Debt: ${}", format_large_number(debt as i64));
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("💵 Fetching cash flow statement for AAPL (5 years, annual)...");
    match get_cash_flow("AAPL", Frequency::Annual, 5).await {
        Ok(cashflow) => {
            println!("Symbol: {}", cashflow.symbol);
            println!("Metrics available: {}", cashflow.metrics.len());

            if let Some(ocf) = cashflow.latest("OperatingCashFlow") {
                println!(
                    "Latest Operating Cash Flow: ${}",
                    format_large_number(ocf as i64)
                );
            }
            if let Some(fcf) = cashflow.latest("FreeCashFlow") {
                println!("Latest Free Cash Flow: ${}", format_large_number(fcf as i64));
            }
            if let Some(capex) = cashflow.latest("CapitalExpenditure") {
                println!("Latest CapEx: ${}", format_large_number(capex as i64));
            }
            if let Some(dividends) = cashflow.latest("CashDividendsPaid") {
                println!(
                    "Latest Dividends Paid: ${}",
                    format_large_number(dividends.abs() as i64)
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📈 Fetching quarterly income statement for AAPL (2 years)...");
    match get_income_statement("AAPL", Frequency::Quarterly, 2).await {
        Ok(income) => {
            println!("Symbol: {}", income.symbol);
            println!("Frequency: {}", income.frequency);

            if let Some(revenue_history) = income.get_metric("TotalRevenue") {
                println!("Quarterly revenue (last 8 quarters):");
                for point in revenue_history.iter().take(8) {
                    println!(
                        "  {} ({}): ${}",
                        point.date,
                        point.period_type,
                        format_large_number(point.value as i64)
                    );
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n✅ All functions tested!");
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
