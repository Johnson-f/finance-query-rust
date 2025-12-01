# Quote Model

The Quote model provides real-time and comprehensive stock quote data, including price, volume, fundamentals, and company information.

## Overview

The quote module contains three main structures:
- `SimpleQuote` - Lightweight quote with essential price information
- `Quote` - Complete quote with all available data fields
- `DetailedQuote` - Same as Quote but with camelCase JSON serialization

These models support various use cases from simple price tracking to comprehensive fundamental analysis.

## Data Structures

### SimpleQuote

Lightweight structure for basic price information.

```rust
pub struct SimpleQuote {
    pub symbol: String,
    pub name: String,
    pub price: String,
    pub pre_market_price: Option<String>,
    pub after_hours_price: Option<String>,
    pub change: String,
    pub percent_change: String,
    pub logo: Option<String>,
}
```

**Fields:**
- `symbol` - Stock ticker symbol
- `name` - Company or security name
- `price` - Current market price
- `pre_market_price` - Pre-market trading price (if available)
- `after_hours_price` - After-hours trading price (if available)
- `change` - Price change from previous close
- `percent_change` - Percentage change from previous close
- `logo` - URL to company logo

### Quote

Complete quote structure with all available data.

```rust
pub struct Quote {
    // Basic Information
    pub symbol: String,
    pub name: String,
    pub price: String,
    pub pre_market_price: Option<String>,
    pub after_hours_price: Option<String>,
    pub change: String,
    pub percent_change: String,
    
    // Daily Trading Data
    pub open: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    pub year_high: Option<String>,
    pub year_low: Option<String>,
    pub volume: Option<i64>,
    pub avg_volume: Option<i64>,
    
    // Fundamental Data
    pub market_cap: Option<String>,
    pub beta: Option<String>,
    pub pe: Option<String>,
    pub eps: Option<String>,
    
    // Dividend Information
    pub dividend: Option<String>,
    pub dividend_yield: Option<String>,
    pub ex_dividend: Option<String>,
    pub last_dividend: Option<String>,
    
    // Fund-Specific Data
    pub net_assets: Option<String>,
    pub nav: Option<String>,
    pub expense_ratio: Option<String>,
    pub category: Option<String>,
    pub last_capital_gain: Option<String>,
    pub morningstar_rating: Option<String>,
    pub morningstar_risk_rating: Option<String>,
    pub holdings_turnover: Option<String>,
    pub inception_date: Option<String>,
    
    // Company Information
    pub earnings_date: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub about: Option<String>,
    pub employees: Option<String>,
    
    // Performance Returns
    pub five_days_return: Option<String>,
    pub one_month_return: Option<String>,
    pub three_month_return: Option<String>,
    pub six_month_return: Option<String>,
    pub ytd_return: Option<String>,
    pub year_return: Option<String>,
    pub three_year_return: Option<String>,
    pub five_year_return: Option<String>,
    pub ten_year_return: Option<String>,
    pub max_return: Option<String>,
    
    pub logo: Option<String>,
}
```

### DetailedQuote

Same structure as `Quote` but serializes to camelCase JSON format for API compatibility.

## JSON Format

### SimpleQuote Example

```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "price": "178.50",
  "pre_market_price": "178.25",
  "after_hours_price": "178.75",
  "change": "2.35",
  "percent_change": "1.33%",
  "logo": "https://logo.clearbit.com/apple.com"
}
```

### Quote Example (Stock)

```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "price": "178.50",
  "pre_market_price": "178.25",
  "after_hours_price": "178.75",
  "change": "2.35",
  "percent_change": "1.33%",
  "open": "176.80",
  "high": "179.20",
  "low": "176.50",
  "year_high": "199.62",
  "year_low": "164.08",
  "volume": 52341890,
  "avg_volume": 58234567,
  "market_cap": "2.78T",
  "beta": "1.24",
  "pe": "29.45",
  "eps": "6.05",
  "dividend": "0.96",
  "dividend_yield": "0.54%",
  "ex_dividend": "2024-11-08",
  "earnings_date": "2024-01-25",
  "last_dividend": "0.24",
  "sector": "Technology",
  "industry": "Consumer Electronics",
  "about": "Apple Inc. designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.",
  "employees": "161,000",
  "five_days_return": "1.2%",
  "one_month_return": "5.8%",
  "three_month_return": "12.3%",
  "six_month_return": "18.7%",
  "ytd_return": "45.2%",
  "year_return": "48.9%",
  "three_year_return": "125.4%",
  "five_year_return": "287.6%",
  "logo": "https://logo.clearbit.com/apple.com"
}
```

### DetailedQuote Example (camelCase)

```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "price": "178.50",
  "preMarketPrice": "178.25",
  "afterHoursPrice": "178.75",
  "change": "2.35",
  "percentChange": "1.33%",
  "open": "176.80",
  "high": "179.20",
  "low": "176.50",
  "yearHigh": "199.62",
  "yearLow": "164.08",
  "volume": 52341890,
  "avgVolume": 58234567,
  "marketCap": "2.78T",
  "beta": "1.24",
  "pe": "29.45",
  "eps": "6.05",
  "dividend": "0.96",
  "yield": "0.54%",
  "exDividend": "2024-11-08",
  "earningsDate": "2024-01-25",
  "lastDividend": "0.24",
  "sector": "Technology",
  "industry": "Consumer Electronics",
  "logo": "https://logo.clearbit.com/apple.com"
}
```

### ETF/Fund Quote Example

```json
{
  "symbol": "SPY",
  "name": "SPDR S&P 500 ETF Trust",
  "price": "458.32",
  "change": "3.21",
  "percent_change": "0.71%",
  "open": "456.50",
  "high": "459.10",
  "low": "456.20",
  "year_high": "478.50",
  "year_low": "408.20",
  "volume": 45678901,
  "avg_volume": 52341890,
  "net_assets": "487.5B",
  "nav": "458.28",
  "expense_ratio": "0.09%",
  "category": "Large Blend",
  "morningstar_rating": "5",
  "morningstar_risk_rating": "Average",
  "holdings_turnover": "3%",
  "inception_date": "1993-01-22",
  "dividend": "5.89",
  "dividend_yield": "1.29%",
  "ytd_return": "24.5%",
  "year_return": "26.8%",
  "three_year_return": "38.2%",
  "five_year_return": "89.4%",
  "ten_year_return": "234.5%"
}
```

## Usage Examples

### Fetching a Single Quote

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let quote = client.get_quote("AAPL").await?;
    
    println!("{} ({})", quote.name, quote.symbol);
    println!("Price: ${}", quote.price);
    println!("Change: {} ({})", quote.change, quote.percent_change);
    
    if let Some(market_cap) = quote.market_cap {
        println!("Market Cap: {}", market_cap);
    }
    
    Ok(())
}
```

### Fetching Multiple Simple Quotes

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let symbols = vec!["AAPL", "GOOGL", "MSFT", "AMZN", "TSLA"];
    let quotes = client.get_simple_quotes(&symbols).await?;
    
    println!("Tech Stock Prices:");
    println!("{:<8} {:<30} {:>12} {:>12}", "Symbol", "Name", "Price", "Change");
    println!("{}", "-".repeat(70));
    
    for quote in quotes {
        println!("{:<8} {:<30} {:>12} {:>12}",
            quote.symbol,
            quote.name,
            format!("${}", quote.price),
            quote.percent_change
        );
    }
    
    Ok(())
}
```

### Monitoring Pre-Market and After-Hours

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let quote = client.get_quote("TSLA").await?;
    
    println!("{} Extended Hours Trading:", quote.symbol);
    
    if let Some(pre_market) = quote.pre_market_price {
        println!("Pre-Market: ${}", pre_market);
    }
    
    println!("Regular: ${}", quote.price);
    
    if let Some(after_hours) = quote.after_hours_price {
        println!("After-Hours: ${}", after_hours);
    }
    
    Ok(())
}
```

### Analyzing Fundamentals

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let quote = client.get_quote("NVDA").await?;
    
    println!("Fundamental Analysis: {}", quote.symbol);
    println!();
    
    if let Some(pe) = quote.pe {
        println!("P/E Ratio: {}", pe);
    }
    
    if let Some(eps) = quote.eps {
        println!("EPS: ${}", eps);
    }
    
    if let Some(beta) = quote.beta {
        println!("Beta: {}", beta);
    }
    
    if let Some(market_cap) = quote.market_cap {
        println!("Market Cap: {}", market_cap);
    }
    
    if let Some(sector) = quote.sector {
        println!("Sector: {}", sector);
    }
    
    if let Some(industry) = quote.industry {
        println!("Industry: {}", industry);
    }
    
    Ok(())
}
```

### Dividend Analysis

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["KO", "PEP", "JNJ", "PG", "T"];
    
    println!("Dividend Stock Analysis:");
    println!("{:<8} {:>12} {:>12} {:>15}", "Symbol", "Price", "Dividend", "Yield");
    println!("{}", "-".repeat(50));
    
    for symbol in symbols {
        let quote = client.get_quote(symbol).await?;
        
        let dividend = quote.dividend.unwrap_or_else(|| "N/A".to_string());
        let yield_val = quote.dividend_yield.unwrap_or_else(|| "N/A".to_string());
        
        println!("{:<8} {:>12} {:>12} {:>15}",
            quote.symbol,
            format!("${}", quote.price),
            format!("${}", dividend),
            yield_val
        );
    }
    
    Ok(())
}
```

### Performance Tracking

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let quote = client.get_quote("SPY").await?;
    
    println!("Performance Returns for {}:", quote.symbol);
    println!();
    
    if let Some(ret) = quote.five_days_return {
        println!("5 Days:     {}", ret);
    }
    if let Some(ret) = quote.one_month_return {
        println!("1 Month:    {}", ret);
    }
    if let Some(ret) = quote.three_month_return {
        println!("3 Months:   {}", ret);
    }
    if let Some(ret) = quote.ytd_return {
        println!("YTD:        {}", ret);
    }
    if let Some(ret) = quote.year_return {
        println!("1 Year:     {}", ret);
    }
    if let Some(ret) = quote.three_year_return {
        println!("3 Years:    {}", ret);
    }
    if let Some(ret) = quote.five_year_return {
        println!("5 Years:    {}", ret);
    }
    
    Ok(())
}
```

### ETF Analysis

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let quote = client.get_quote("VTI").await?;
    
    println!("ETF Analysis: {} - {}", quote.symbol, quote.name);
    println!();
    
    if let Some(nav) = quote.nav {
        println!("NAV: ${}", nav);
    }
    
    if let Some(net_assets) = quote.net_assets {
        println!("Net Assets: {}", net_assets);
    }
    
    if let Some(expense_ratio) = quote.expense_ratio {
        println!("Expense Ratio: {}", expense_ratio);
    }
    
    if let Some(category) = quote.category {
        println!("Category: {}", category);
    }
    
    if let Some(rating) = quote.morningstar_rating {
        println!("Morningstar Rating: {} stars", rating);
    }
    
    if let Some(turnover) = quote.holdings_turnover {
        println!("Holdings Turnover: {}", turnover);
    }
    
    Ok(())
}
```

### Building a Stock Screener

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN", "META", "TSLA", "NVDA"];
    
    println!("Value Stock Screener (P/E < 30, Dividend Yield > 0%):");
    println!("{:<8} {:>12} {:>10} {:>12}", "Symbol", "Price", "P/E", "Div Yield");
    println!("{}", "-".repeat(50));
    
    for symbol in symbols {
        let quote = client.get_quote(symbol).await?;
        
        // Parse P/E ratio
        let pe_value = quote.pe
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok());
        
        // Parse dividend yield
        let yield_value = quote.dividend_yield
            .as_ref()
            .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok());
        
        // Apply filters
        if let (Some(pe), Some(div_yield)) = (pe_value, yield_value) {
            if pe < 30.0 && div_yield > 0.0 {
                println!("{:<8} {:>12} {:>10.2} {:>12}",
                    quote.symbol,
                    format!("${}", quote.price),
                    pe,
                    quote.dividend_yield.unwrap()
                );
            }
        }
    }
    
    Ok(())
}
```

### Price Alert System

```rust
use finance_query_core::YahooClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Define price targets
    let mut targets: HashMap<&str, f64> = HashMap::new();
    targets.insert("AAPL", 180.0);
    targets.insert("TSLA", 250.0);
    targets.insert("NVDA", 500.0);
    
    println!("Price Alert Monitor:");
    println!();
    
    for (symbol, target) in targets {
        let quote = client.get_quote(symbol).await?;
        let current_price = quote.price.parse::<f64>().unwrap_or(0.0);
        
        if current_price >= target {
            println!("🚨 ALERT: {} reached target!", symbol);
            println!("   Current: ${:.2} | Target: ${:.2}", current_price, target);
        } else {
            let diff = target - current_price;
            let pct = (diff / current_price) * 100.0;
            println!("✓ {}: ${:.2} (${:.2} / {:.1}% to target)",
                symbol, current_price, diff, pct);
        }
    }
    
    Ok(())
}
```

### Portfolio Tracker

```rust
use finance_query_core::YahooClient;

struct Position {
    symbol: String,
    shares: f64,
    cost_basis: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let portfolio = vec![
        Position { symbol: "AAPL".to_string(), shares: 100.0, cost_basis: 150.0 },
        Position { symbol: "MSFT".to_string(), shares: 50.0, cost_basis: 300.0 },
        Position { symbol: "GOOGL".to_string(), shares: 25.0, cost_basis: 120.0 },
    ];
    
    println!("Portfolio Summary:");
    println!("{:<8} {:>10} {:>12} {:>12} {:>12} {:>12}",
        "Symbol", "Shares", "Cost", "Current", "Value", "Gain/Loss");
    println!("{}", "-".repeat(75));
    
    let mut total_value = 0.0;
    let mut total_cost = 0.0;
    
    for position in portfolio {
        let quote = client.get_quote(&position.symbol).await?;
        let current_price = quote.price.parse::<f64>().unwrap_or(0.0);
        let current_value = position.shares * current_price;
        let cost = position.shares * position.cost_basis;
        let gain_loss = current_value - cost;
        let gain_loss_pct = (gain_loss / cost) * 100.0;
        
        total_value += current_value;
        total_cost += cost;
        
        println!("{:<8} {:>10.0} {:>12.2} {:>12.2} {:>12.2} {:>11.2}%",
            position.symbol,
            position.shares,
            position.cost_basis,
            current_price,
            current_value,
            gain_loss_pct
        );
    }
    
    let total_gain = total_value - total_cost;
    let total_gain_pct = (total_gain / total_cost) * 100.0;
    
    println!("{}", "-".repeat(75));
    println!("Total Value: ${:.2} | Total Gain: ${:.2} ({:.2}%)",
        total_value, total_gain, total_gain_pct);
    
    Ok(())
}
```

### Exporting Quote Data

```rust
use finance_query_core::YahooClient;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let quote = client.get_quote("AAPL").await?;
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&quote)?;
    std::fs::write("aapl_quote.json", json)?;
    
    // Export to CSV
    let csv = format!(
        "Symbol,Name,Price,Change,PercentChange,Volume,MarketCap\n{},{},{},{},{},{},{}\n",
        quote.symbol,
        quote.name,
        quote.price,
        quote.change,
        quote.percent_change,
        quote.volume.unwrap_or(0),
        quote.market_cap.unwrap_or_else(|| "N/A".to_string())
    );
    std::fs::write("aapl_quote.csv", csv)?;
    
    println!("Quote data exported successfully");
    
    Ok(())
}
```

## Field Availability

Different security types have different field availability:

### Stocks
- All basic price fields
- Fundamental data (P/E, EPS, Beta)
- Dividend information
- Sector and industry
- Company information

### ETFs/Funds
- Basic price fields
- Fund-specific data (NAV, expense ratio, net assets)
- Morningstar ratings
- Performance returns
- Category and inception date

### Indices
- Basic price fields
- Limited fundamental data
- No dividend information

## Data Types

### String Fields
Most numeric fields are returned as strings to preserve formatting and precision:
- Prices: "178.50"
- Percentages: "1.33%" or "0.54%"
- Market cap: "2.78T", "487.5B", "12.3M"
- Ratios: "29.45", "1.24"

### Integer Fields
- `volume` - Trading volume as i64
- `avg_volume` - Average volume as i64

## Notes

- Quote data is typically delayed by 15-20 minutes for most exchanges
- Real-time data requires a premium subscription
- Pre-market and after-hours prices are only available during those trading sessions
- Not all fields are available for all securities
- Market cap uses abbreviations: T (trillion), B (billion), M (million)
- Percentage fields include the "%" symbol
- Fund-specific fields are only populated for ETFs and mutual funds
- Company information fields may be empty for smaller companies
- Performance returns are typically only available for funds and ETFs

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_quote("INVALID").await {
        Ok(quote) => {
            println!("Quote: {} - ${}", quote.symbol, quote.price);
        }
        Err(YahooError::NotFound) => {
            eprintln!("Symbol not found");
        }
        Err(YahooError::Network(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## See Also

- [Historical Model](./historical.md) - For historical price data
- [Options Model](./options.md) - For options chain data
- [News Model](./news.md) - For related news articles
- [Calendar Model](./calendar.md) - For earnings dates
- [Analysts Model](./analysts.md) - For analyst ratings
