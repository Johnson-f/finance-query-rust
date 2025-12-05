mod analysts;
mod calendar;
mod financial;
mod historical;
mod holder;
mod news;
mod quotes;
mod sector;
mod stream;

use analysts::{
    get_all_analyst_data, get_earnings_history, get_price_targets, get_recommendations,
    get_upgrades_downgrades,
};
use calendar::{get_dividend_calendar, get_earnings_calendar, get_full_calendar};
use financial::{get_balance_sheet, get_cash_flow, get_income_statement, Frequency};
use historical::{get_chart, get_historical_data, get_historical_data_by_date, ChartResponse};
use holder::{
    get_all_holders, get_institutional_holders, get_insider_roster, get_insider_transactions,
    get_major_holders, get_mutual_fund_holders,
};
use news::{get_market_news, get_news_for_symbol, search_news};
use quotes::{
    get_detailed_quote, get_logo_url, get_similar_quotes, get_simple_quotes,
    DetailedQuoteResponse, SimpleQuotesResponse, SimilarQuotesResponse,
};
use sector::{get_all_sectors, get_all_sectors_performance, get_sector_performance, Sector};
use stream::{stream_indices, stream_movers, stream_quotes, stream_single_quote, MoverCount};

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
    println!();

    // ========================================================================
    // Holder Data Tests
    // ========================================================================

    println!("🏛️ Fetching major holders breakdown for AAPL...");
    match get_major_holders("AAPL").await {
        Ok(breakdown) => {
            if let Some(insiders) = breakdown.insiders_percent_held {
                println!("Insiders: {:.2}%", insiders * 100.0);
            }
            if let Some(institutions) = breakdown.institutions_percent_held {
                println!("Institutions: {:.2}%", institutions * 100.0);
            }
            if let Some(float_held) = breakdown.institutions_float_percent_held {
                println!("Float held by institutions: {:.2}%", float_held * 100.0);
            }
            if let Some(count) = breakdown.institutions_count {
                println!("Number of institutions: {}", count);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("🏦 Fetching top institutional holders for AAPL...");
    match get_institutional_holders("AAPL").await {
        Ok(holders) => {
            println!("Top 5 institutional holders:");
            for holder in holders.iter().take(5) {
                println!(
                    "  {} - {} shares (${}) - {:.2}%",
                    holder.organization,
                    format_large_number(holder.shares),
                    holder.value.map(|v| format_large_number(v)).unwrap_or("N/A".to_string()),
                    holder.percent_held.unwrap_or(0.0) * 100.0
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📊 Fetching top mutual fund holders for AAPL...");
    match get_mutual_fund_holders("AAPL").await {
        Ok(holders) => {
            println!("Top 5 mutual fund holders:");
            for holder in holders.iter().take(5) {
                println!(
                    "  {} - {} shares",
                    holder.organization,
                    format_large_number(holder.shares)
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("👤 Fetching insider transactions for AAPL...");
    match get_insider_transactions("AAPL").await {
        Ok(transactions) => {
            println!("Recent insider transactions:");
            for txn in transactions.iter().take(5) {
                println!(
                    "  {} ({}) - {} - {} shares",
                    txn.filer_name,
                    txn.filer_relation.as_deref().unwrap_or("N/A"),
                    txn.transaction_text.as_deref().unwrap_or("N/A"),
                    txn.shares.map(|s| format_large_number(s)).unwrap_or("N/A".to_string())
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📋 Fetching insider roster for AAPL...");
    match get_insider_roster("AAPL").await {
        Ok(roster) => {
            println!("Company insiders:");
            for insider in roster.iter().take(5) {
                println!(
                    "  {} ({}) - {} shares",
                    insider.name,
                    insider.relation.as_deref().unwrap_or("N/A"),
                    format_large_number(insider.total_shares())
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📦 Fetching all holder data for AAPL...");
    match get_all_holders("AAPL").await {
        Ok(data) => {
            println!("Symbol: {}", data.symbol);
            println!("Major holders: {}", if data.major_holders.is_some() { "✓" } else { "✗" });
            println!("Institutional holders: {}", data.institutional_holders.as_ref().map(|h| h.len()).unwrap_or(0));
            println!("Mutual fund holders: {}", data.mutual_fund_holders.as_ref().map(|h| h.len()).unwrap_or(0));
            println!("Insider transactions: {}", data.insider_transactions.as_ref().map(|t| t.len()).unwrap_or(0));
            println!("Insider roster: {}", data.insider_roster.as_ref().map(|r| r.len()).unwrap_or(0));
            println!("Insider buys: {}, sells: {}", data.insider_buy_count(), data.insider_sell_count());
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // ========================================================================
    // News Tests
    // ========================================================================

    println!("📰 Fetching news for AAPL...");
    match get_news_for_symbol("AAPL", 5).await {
        Ok(news) => {
            println!("Found {} articles for {}", news.count, news.symbol.as_deref().unwrap_or("N/A"));
            for article in news.articles.iter().take(5) {
                println!(
                    "  {} - {} ({})",
                    article.title,
                    article.publisher.as_deref().unwrap_or("Unknown"),
                    article.relative_time()
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("🔍 Searching news for 'technology stocks'...");
    match search_news("technology stocks", 5).await {
        Ok(news) => {
            println!("Found {} articles", news.count);
            for article in news.articles.iter().take(5) {
                println!(
                    "  {} - {}",
                    article.title,
                    article.publisher.as_deref().unwrap_or("Unknown")
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("🌐 Fetching market news...");
    match get_market_news(5).await {
        Ok(news) => {
            println!("Found {} market news articles", news.count);
            for article in news.articles.iter().take(5) {
                println!("  {} ({})", article.title, article.relative_time());
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // ========================================================================
    // Analyst Data Tests
    // ========================================================================

    println!("📊 Fetching analyst recommendations for AAPL...");
    match get_recommendations("AAPL").await {
        Ok(recs) => {
            if let Some(latest) = recs.first() {
                println!(
                    "Period: {} | Consensus: {} ({} analysts)",
                    latest.period,
                    latest.consensus(),
                    latest.total_analysts()
                );
                println!(
                    "  Strong Buy: {}, Buy: {}, Hold: {}, Sell: {}, Strong Sell: {}",
                    latest.strong_buy, latest.buy, latest.hold, latest.sell, latest.strong_sell
                );
                println!("  Bullish: {:.1}%", latest.bullish_percent());
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📈 Fetching price targets for AAPL...");
    match get_price_targets("AAPL").await {
        Ok(targets) => {
            println!(
                "Current: ${:.2}",
                targets.current_price.unwrap_or(0.0)
            );
            println!(
                "Target Mean: ${:.2}, Median: ${:.2}",
                targets.target_mean.unwrap_or(0.0),
                targets.target_median.unwrap_or(0.0)
            );
            println!(
                "Range: ${:.2} - ${:.2}",
                targets.target_low.unwrap_or(0.0),
                targets.target_high.unwrap_or(0.0)
            );
            if let Some(upside) = targets.upside_percent() {
                println!("Upside potential: {:.1}%", upside);
            }
            println!(
                "Analysts: {}",
                targets.number_of_analysts.unwrap_or(0)
            );
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("⬆️ Fetching upgrades/downgrades for AAPL...");
    match get_upgrades_downgrades("AAPL").await {
        Ok(changes) => {
            println!("Recent rating changes:");
            for change in changes.iter().take(5) {
                let action = change.action.as_deref().unwrap_or("N/A");
                let from = change.from_grade.as_deref().unwrap_or("N/A");
                let to = change.to_grade.as_deref().unwrap_or("N/A");
                let date = change.date.as_deref().unwrap_or("N/A");
                println!("  {} - {} -> {} ({}) [{}]", change.firm, from, to, action, date);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📉 Fetching earnings history for AAPL...");
    match get_earnings_history("AAPL").await {
        Ok(history) => {
            println!("Recent earnings:");
            for item in history.iter().take(4) {
                let beat = if item.beat_estimate() == Some(true) {
                    "✓ Beat"
                } else {
                    "✗ Miss"
                };
                println!(
                    "  {}: Actual ${:.2} vs Est ${:.2} ({}) - {:.1}% surprise",
                    item.quarter.as_deref().unwrap_or("N/A"),
                    item.eps_actual.unwrap_or(0.0),
                    item.eps_estimate.unwrap_or(0.0),
                    beat,
                    item.surprise_percent.unwrap_or(0.0)
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📦 Fetching all analyst data for AAPL...");
    match get_all_analyst_data("AAPL").await {
        Ok(data) => {
            println!("Symbol: {}", data.symbol);
            println!(
                "Recommendations: {}",
                data.recommendations.as_ref().map(|r| r.len()).unwrap_or(0)
            );
            println!(
                "Upgrades/Downgrades: {} (↑{} ↓{})",
                data.upgrades_downgrades
                    .as_ref()
                    .map(|u| u.len())
                    .unwrap_or(0),
                data.upgrade_count(),
                data.downgrade_count()
            );
            println!(
                "Price Target: {}",
                if data.price_target.is_some() {
                    "✓"
                } else {
                    "✗"
                }
            );
            println!(
                "Earnings History: {}",
                data.earnings_history.as_ref().map(|h| h.len()).unwrap_or(0)
            );
            if let Some(beat_rate) = data.earnings_beat_rate() {
                println!("Earnings Beat Rate: {:.1}%", beat_rate);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // ========================================================================
    // Calendar Data Tests
    // ========================================================================

    println!("📅 Fetching earnings calendar for AAPL...");
    match get_earnings_calendar("AAPL").await {
        Ok(earnings) => {
            println!("Next earnings: {}", earnings.date_display());
            if let Some(avg) = earnings.earnings_average {
                println!("EPS estimate: ${:.2}", avg);
            }
            if let (Some(low), Some(high)) = (earnings.earnings_low, earnings.earnings_high) {
                println!("EPS range: ${:.2} - ${:.2}", low, high);
            }
            if let Some(rev) = earnings.revenue_average {
                println!("Revenue estimate: ${}", format_large_number(rev as i64));
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("💰 Fetching dividend calendar for AAPL...");
    match get_dividend_calendar("AAPL").await {
        Ok(dividend) => {
            if let Some(ex_date) = &dividend.ex_dividend_date {
                println!("Ex-dividend date: {}", ex_date);
            }
            if let Some(div_date) = &dividend.dividend_date {
                println!("Dividend date: {}", div_date);
            }
            if let Some(rate) = dividend.dividend_rate {
                println!("Annual dividend: ${:.2}", rate);
            }
            if let Some(yield_pct) = dividend.dividend_yield {
                println!("Dividend yield: {:.2}%", yield_pct * 100.0);
            }
            if let Some(payout) = dividend.payout_ratio {
                println!("Payout ratio: {:.1}%", payout * 100.0);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📆 Fetching full calendar for AAPL...");
    match get_full_calendar("AAPL").await {
        Ok(calendar) => {
            println!("Symbol: {}", calendar.symbol);
            println!(
                "Earnings: {}",
                if calendar.earnings.is_some() { "✓" } else { "✗" }
            );
            println!(
                "Dividend: {}",
                if calendar.dividend.is_some() { "✓" } else { "✗" }
            );
            println!(
                "Split info: {}",
                if calendar.split.is_some() { "✓" } else { "✗" }
            );
            println!(
                "Has upcoming events: {}",
                if calendar.has_upcoming_events() { "Yes" } else { "No" }
            );
            if let Some(next) = calendar.next_event_date() {
                println!("Next event: {}", next);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // ========================================================================
    // Sector Data Tests
    // ========================================================================

    println!("🏭 Fetching all available sectors...");
    let sectors = get_all_sectors();
    println!("Available sectors ({}):", sectors.len());
    for sector in &sectors {
        println!("  - {}", sector.as_str());
    }
    println!();

    println!("📊 Fetching sector performance for Technology...");
    match get_sector_performance(Sector::Technology).await {
        Ok(perf) => {
            println!("Sector: {}", perf.sector);
            println!("Day Return: {}", perf.day_return_formatted());
            println!("YTD Return: {}", perf.ytd_return_formatted());
            println!("1Y Return: {:+.2}%", perf.year_return);
            println!("Positive day: {}", if perf.is_positive_day() { "Yes" } else { "No" });
            println!("Positive YTD: {}", if perf.is_positive_ytd() { "Yes" } else { "No" });
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("📈 Fetching all sectors performance...");
    match get_all_sectors_performance().await {
        Ok(overview) => {
            println!("Best performer: {}", overview.best_performer.as_deref().unwrap_or("N/A"));
            println!("Worst performer: {}", overview.worst_performer.as_deref().unwrap_or("N/A"));
            println!("\nSectors by day return:");
            for sector in overview.sorted_by_day_return() {
                println!("  {} - {}", sector.sector, sector.day_return_formatted());
            }
            println!("\nPositive sectors: {}", overview.positive_sectors().len());
            println!("Negative sectors: {}", overview.negative_sectors().len());
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // ========================================================================
    // Streaming Tests (limited to 2 updates each for testing)
    // ========================================================================

    println!("🔄 Testing streaming functionality...\n");

    // Test quote streaming
    println!("📈 Testing quote stream (2 updates)...");
    match stream_quotes(vec!["AAPL", "GOOGL"], 3, Some(2)).await {
        Ok(_) => println!("Quote stream completed successfully"),
        Err(e) => println!("Quote stream error: {}", e),
    }
    println!();

    // Test single quote streaming
    println!("📊 Testing single quote stream for NVDA (2 updates)...");
    match stream_single_quote("NVDA", 3, Some(2)).await {
        Ok(_) => println!("Single quote stream completed successfully"),
        Err(e) => println!("Single quote stream error: {}", e),
    }
    println!();

    // Test index streaming
    println!("📊 Testing index stream (2 updates)...");
    match stream_indices(vec!["^GSPC", "^DJI", "^IXIC"], 3, Some(2)).await {
        Ok(_) => println!("Index stream completed successfully"),
        Err(e) => println!("Index stream error: {}", e),
    }
    println!();

    // Test movers streaming
    println!("🔥 Testing market movers stream (2 updates)...");
    match stream_movers(MoverCount::TwentyFive, 5, Some(2)).await {
        Ok(_) => println!("Movers stream completed successfully"),
        Err(e) => println!("Movers stream error: {}", e),
    }
    println!();

    println!("\n✅ All functions tested (including streaming)!");
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
