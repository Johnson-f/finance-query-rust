# finance-query-core

A Rust client library for Yahoo Finance API. Fetch quotes, historical data, financials, news, and more.

## Features

- Framework-agnostic design - use with any Rust application
- Automatic authentication handling with cookie/crumb management
- Strongly-typed data models for all Yahoo Finance responses
- Comprehensive error handling
- WebSocket support for real-time data
- Async streaming for continuous quote updates

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
finance-query-core = "0.1"
```

## Quick Start

```rust
use finance_query_core::{YahooFinanceClient, YahooAuthManager, FetchClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    auth_manager.refresh().await?;
    let quote = client.get_quote("AAPL").await?;
    println!("Price: {}", quote["regularMarketPrice"]);

    Ok(())
}
```

## Models & JSON Formats

### Quote Models

#### SimpleQuote
Basic quote data for streaming and lists.

```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "price": "278.85",
  "pre_market_price": "279.50",
  "after_hours_price": "278.37",
  "change": "+1.30",
  "percent_change": "+0.47%",
  "logo": "https://logo.clearbit.com/apple.com"
}
```

#### Quote
Full quote data with all available fields.

```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "price": "278.85",
  "change": "+1.30",
  "percent_change": "+0.47%",
  "open": "277.26",
  "high": "279.00",
  "low": "275.99",
  "year_high": "280.38",
  "year_low": "169.21",
  "volume": 16129268,
  "avg_volume": 51432647,
  "market_cap": "4.14T",
  "beta": "1.11",
  "pe": "37.33",
  "eps": "7.47",
  "dividend": "1.04",
  "dividend_yield": "0.37%",
  "ex_dividend": "2025-11-10",
  "sector": "Technology",
  "industry": "Consumer Electronics",
  "about": "Apple Inc. designs, manufactures...",
  "employees": "164000",
  "earnings_date": "2025-01-30",
  "five_days_return": "4.73%",
  "one_month_return": "3.66%",
  "ytd_return": "11.35%",
  "year_return": "18.69%",
  "logo": "https://logo.clearbit.com/apple.com"
}
```

### Historical Models

#### HistoricalData
OHLCV price data for a single time period.

```json
{
  "open": 277.26,
  "high": 279.00,
  "low": 275.99,
  "close": 278.85,
  "volume": 16129268,
  "adj_close": 278.85,
  "sma": { "20": 262.64, "50": 245.30 },
  "ema": { "12": 270.50, "26": 258.20 }
}
```

#### HistoricalResponse
```json
{
  "data": {
    "2025-11-28": { "open": 277.26, "high": 279.00, "low": 275.99, "close": 278.85, "volume": 16129268 },
    "2025-11-27": { "open": 275.50, "high": 278.00, "low": 274.80, "close": 277.55, "volume": 18234567 }
  }
}
```

#### TimeRange
Options: `"1d"`, `"5d"`, `"1mo"`, `"3mo"`, `"6mo"`, `"1y"`, `"2y"`, `"5y"`, `"10y"`, `"ytd"`, `"max"`

#### Interval
Options: `"1m"`, `"3m"`, `"5m"`, `"10m"`, `"15m"`, `"20m"`, `"30m"`, `"65m"`, `"95m"`, `"1h"`, `"1d"`, `"1wk"`, `"1mo"`

### News

```json
{
  "title": "Apple Reports Record Q4 Earnings",
  "link": "https://finance.yahoo.com/news/apple-q4-earnings",
  "source": "Reuters",
  "img": "https://s.yimg.com/news/apple-earnings.jpg",
  "time": "2h ago"
}
```

### Search

#### SearchResult
```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "exchange": "NASDAQ",
  "quote_type": "EQUITY"
}
```

#### SearchResponse
```json
{
  "results": [
    { "symbol": "AAPL", "name": "Apple Inc.", "exchange": "NASDAQ", "quote_type": "EQUITY" },
    { "symbol": "AAPL.MX", "name": "Apple Inc.", "exchange": "Mexico", "quote_type": "EQUITY" }
  ]
}
```

### Financial Models

#### FinancialStatement
```json
{
  "symbol": "AAPL",
  "statement_type": "income",
  "frequency": "annual",
  "statement": {
    "2024": {
      "TotalRevenue": 383285000000,
      "GrossProfit": 169148000000,
      "NetIncome": 96995000000
    },
    "2023": {
      "TotalRevenue": 394328000000,
      "GrossProfit": 169148000000,
      "NetIncome": 96995000000
    }
  }
}
```

#### StatementType
Options: `"income"`, `"balance"`, `"cashflow"`

#### Frequency
Options: `"annual"`, `"quarterly"`

### Market Movers

#### MarketMover
```json
{
  "symbol": "NVDA",
  "name": "NVIDIA Corporation",
  "price": "145.50",
  "change": "+8.25",
  "percentChange": "+6.01%"
}
```

#### MoverCount
Options: `"25"`, `"50"`, `"100"`

### Market Indices

#### MarketIndex
```json
{
  "name": "S&P 500",
  "value": 6032.38,
  "change": "+33.64",
  "percentChange": "+0.56%",
  "fiveDaysReturn": "1.06%",
  "oneMonthReturn": "5.73%",
  "ytdReturn": "26.47%",
  "yearReturn": "32.06%"
}
```

#### Region
Options: `"US"`, `"NA"`, `"SA"`, `"EU"`, `"AS"`, `"AF"`, `"ME"`, `"OCE"`, `"global"`

#### Index
Major indices: `"snp"`, `"djia"`, `"nasdaq"`, `"ftse-100"`, `"dax"`, `"nikkei-225"`, `"hang-seng"`, and 60+ more.

### Sectors

#### MarketSector
```json
{
  "sector": "Technology",
  "dayReturn": "+0.85%",
  "ytdReturn": "+32.45%",
  "yearReturn": "+45.67%",
  "threeYearReturn": "+78.90%",
  "fiveYearReturn": "+156.78%"
}
```

#### MarketSectorDetails
```json
{
  "sector": "Technology",
  "dayReturn": "+0.85%",
  "ytdReturn": "+32.45%",
  "yearReturn": "+45.67%",
  "threeYearReturn": "+78.90%",
  "fiveYearReturn": "+156.78%",
  "marketCap": "18.5T",
  "marketWeight": "32.5%",
  "industries": 8,
  "companies": 450,
  "topIndustries": ["Semiconductors", "Software", "Hardware"],
  "topCompanies": ["AAPL", "MSFT", "NVDA"]
}
```

### Holders

#### InstitutionalHolder
```json
{
  "holder": "Vanguard Group Inc",
  "shares": 1234567890,
  "dateReported": "2025-09-30T00:00:00Z",
  "percentOut": 8.45,
  "value": 344234567890
}
```

#### MutualFundHolder
```json
{
  "holder": "Vanguard Total Stock Market Index Fund",
  "shares": 234567890,
  "dateReported": "2025-09-30T00:00:00Z",
  "percentOut": 1.58,
  "value": 65234567890
}
```

#### InsiderTransaction
```json
{
  "startDate": "2025-11-15T00:00:00Z",
  "insider": "Tim Cook",
  "position": "CEO",
  "transaction": "Sale",
  "shares": 50000,
  "value": 13942500,
  "ownership": "D"
}
```

#### InsiderPurchase
```json
{
  "period": "Last 6 Months",
  "purchasesShares": 125000,
  "purchasesTransactions": 15,
  "salesShares": 450000,
  "salesTransactions": 25,
  "netShares": -325000,
  "netTransactions": -10
}
```

### Analysts

#### RecommendationData
```json
{
  "period": "2025-11",
  "strongBuy": 15,
  "buy": 25,
  "hold": 8,
  "sell": 2,
  "strongSell": 0
}
```

#### UpgradeDowngrade
```json
{
  "firm": "Morgan Stanley",
  "toGrade": "Overweight",
  "fromGrade": "Equal-Weight",
  "action": "upgrade",
  "date": "2025-11-20T00:00:00Z"
}
```

#### PriceTarget
```json
{
  "current": 278.85,
  "mean": 245.50,
  "median": 250.00,
  "low": 180.00,
  "high": 300.00
}
```

#### EarningsHistoryItem
```json
{
  "date": "2025-10-31T00:00:00Z",
  "epsActual": 1.64,
  "epsEstimate": 1.60,
  "surprise": 0.04,
  "surprisePercent": 2.5
}
```

### Earnings Transcripts

#### EarningsCallListing
```json
{
  "event_id": "abc123",
  "quarter": "Q4",
  "year": 2024,
  "title": "Apple Inc. Q4 2024 Earnings Call",
  "url": "https://finance.yahoo.com/transcript/aapl-q4-2024"
}
```

#### EarningsTranscript
```json
{
  "symbol": "AAPL",
  "quarter": "Q4",
  "year": 2024,
  "date": "2024-10-31T21:00:00Z",
  "title": "Apple Inc. Q4 2024 Earnings Call",
  "speakers": [
    { "name": "Tim Cook", "role": "CEO", "company": "Apple Inc." },
    { "name": "Luca Maestri", "role": "CFO", "company": "Apple Inc." }
  ],
  "paragraphs": [
    { "speaker": "Tim Cook", "text": "Good afternoon and thank you for joining us..." }
  ],
  "metadata": {}
}
```

### WebSocket Types

#### QuotesUpdate
```json
{
  "quotes": [
    { "symbol": "AAPL", "name": "Apple Inc.", "price": "278.85", "change": "+1.30", "percent_change": "+0.47%" }
  ],
  "timestamp": "2025-11-30T12:39:37.556245Z"
}
```

#### ProfileUpdate
```json
{
  "quote": { "symbol": "AAPL", "name": "Apple Inc.", "price": "278.85" },
  "similar": [{ "symbol": "MSFT", "name": "Microsoft", "price": "492.01" }],
  "sector_performance": { "sector": "Technology", "dayReturn": "+0.85%" },
  "news": [{ "title": "Apple News", "link": "...", "source": "Reuters" }]
}
```

#### MoversUpdate
```json
{
  "actives": [{ "symbol": "NVDA", "name": "NVIDIA", "price": "145.50", "percentChange": "+6.01%" }],
  "gainers": [{ "symbol": "XYZ", "name": "XYZ Corp", "price": "50.00", "percentChange": "+15.00%" }],
  "losers": [{ "symbol": "ABC", "name": "ABC Inc", "price": "25.00", "percentChange": "-10.00%" }]
}
```

#### MarketHours
```json
{
  "status": "open",
  "reason": null,
  "timestamp": "2025-11-30T14:30:00Z"
}
```

#### MovingAverageUpdate
```json
{
  "symbol": "AAPL",
  "indicator_type": "SMA",
  "period": 20,
  "value": 262.64,
  "timestamp": "2025-11-30T14:30:00Z"
}
```

## Streaming

### Quote Streaming

Create async streams for continuous quote updates:

```rust
use finance_query_core::QuoteStream;
use futures_util::StreamExt;
use std::time::Duration;

let symbols = vec!["AAPL".to_string(), "GOOGL".to_string()];
let mut stream = QuoteStream::new(client, symbols, Duration::from_secs(5));

while let Some(Ok(update)) = stream.next().await {
    for quote in &update.quotes {
        println!("{}: ${}", quote.symbol, quote.price);
    }
}
```

### Index Streaming

Stream real-time market index data:

```rust
use finance_query_core::IndexStream;
use futures_util::StreamExt;
use std::time::Duration;

// Stream major US indices (S&P 500, Dow Jones, NASDAQ)
let mut stream = IndexStream::us_major_indices(client.clone(), Duration::from_secs(5));

while let Some(Ok(indices)) = stream.next().await {
    for index in indices {
        println!("{}: {:.2} ({:+})", index.name, index.value, index.percent_change);
    }
}
```

Or stream custom indices:

```rust
// Stream any indices by symbol
let symbols = vec!["^GSPC".to_string(), "^FTSE".to_string(), "^N225".to_string()];
let mut stream = IndexStream::new(client, symbols, Duration::from_secs(5));

while let Some(Ok(indices)) = stream.next().await {
    for index in indices {
        println!("{}: {:.2}", index.name, index.value);
    }
}
```

### Movers Streaming

Stream real-time market movers (most active, top gainers, top losers):

```rust
use finance_query_core::{MoversStream, MoverCount};
use futures_util::StreamExt;
use std::time::Duration;

// Stream top 50 movers in each category
let mut stream = MoversStream::new(client, MoverCount::Fifty, Duration::from_secs(5));

while let Some(Ok(update)) = stream.next().await {
    println!("\n📈 Top 5 Gainers:");
    for (i, mover) in update.gainers.iter().take(5).enumerate() {
        println!("  {}. {} ({}): {} {}", 
            i + 1, mover.symbol, mover.name, mover.price, mover.percent_change);
    }
    
    println!("\n📉 Top 5 Losers:");
    for (i, mover) in update.losers.iter().take(5).enumerate() {
        println!("  {}. {} ({}): {} {}", 
            i + 1, mover.symbol, mover.name, mover.price, mover.percent_change);
    }
    
    println!("\n🔥 Top 5 Most Active:");
    for (i, mover) in update.actives.iter().take(5).enumerate() {
        println!("  {}. {} ({}): {} {}", 
            i + 1, mover.symbol, mover.name, mover.price, mover.percent_change);
    }
}
```

Or use defaults (50 movers, 5-second interval):

```rust
let mut stream = MoversStream::with_defaults(client);

while let Some(Ok(update)) = stream.next().await {
    println!("Actives: {}, Gainers: {}, Losers: {}", 
        update.actives.len(), update.gainers.len(), update.losers.len());
}
```

## Error Handling

```rust
use finance_query_core::YahooError;

match client.get_quote("INVALID").await {
    Ok(quote) => println!("Got quote"),
    Err(YahooError::NotFound(msg)) => println!("Not found: {}", msg),
    Err(YahooError::AuthFailed(msg)) => println!("Auth failed: {}", msg),
    Err(YahooError::RateLimited) => println!("Rate limited"),
    Err(YahooError::HttpError(code, msg)) => println!("HTTP {}: {}", code, msg),
    Err(YahooError::ParseError(msg)) => println!("Parse error: {}", msg),
    Err(YahooError::NetworkError(e)) => println!("Network error: {}", e),
}
```

## New Features (v0.2)

### Stock Actions (Dividends, Splits, Capital Gains)

```rust
// Get all stock actions
let actions = client.get_actions("AAPL", "5y").await?;
println!("Total dividends: ${:.2}", actions.total_dividends());

// Get just dividends
let dividends = client.get_dividends("AAPL", "1y").await?;
for div in &dividends {
    println!("{}: ${:.4}", div.date.format("%Y-%m-%d"), div.amount);
}

// Get stock splits
let splits = client.get_splits("AAPL", "max").await?;
```

### Options Data

```rust
// Get option expirations
let expirations = client.get_option_expirations("AAPL").await?;

// Get option chain for specific expiration
let chain = client.get_option_chain("AAPL", Some("2025-12-19")).await?;
for call in &chain.calls {
    println!("Strike: ${:.2}, IV: {:.2}%", call.strike, call.implied_volatility * 100.0);
}
```

### Calendar Events

```rust
let calendar = client.get_calendar("AAPL").await?;
if let Some(date) = calendar.earnings_date {
    println!("Next earnings: {}", date.format("%Y-%m-%d"));
}
```

### SEC Filings

```rust
let filings = client.get_sec_filings("AAPL").await?;
for filing in &filings.filings {
    println!("{}: {} - {}", filing.date.format("%Y-%m-%d"), filing.filing_type, filing.title);
}
```

### ESG/Sustainability Scores

```rust
let esg = client.get_sustainability("AAPL").await?;
if let Some(score) = esg.total_esg {
    println!("ESG Score: {:.1} (Rating: {})", score, esg.rating().unwrap_or("N/A"));
}
```

### Industry Data

```rust
let industry = client.get_industry("technology-hardware").await?;
println!("Industry: {} (Sector: {:?})", industry.name, industry.sector_name);
for company in &industry.top_performing_companies {
    println!("  {} ({})", company.name, company.symbol);
}
```

### Market Status & Summary

```rust
// Check if market is open
let status = client.get_market_status("us_market").await?;
println!("Market is {}", if status.is_open() { "open" } else { "closed" });

// Get market summary with indices
let summary = client.get_market_summary("us_market").await?;
for index in &summary.indices {
    println!("{}: {:.2} ({:+.2}%)", index.short_name, index.price, index.percent_change);
}
```

### Market Movers

```rust
use finance_query_core::MoverCount;

// Get top 50 movers in each category
let (actives, gainers, losers) = client.get_movers(MoverCount::Fifty).await?;

// Display top gainers
println!("Top Gainers:");
for mover in gainers.iter().take(10) {
    println!("  {} ({}): {} {}", 
        mover.symbol, mover.name, mover.price, mover.percent_change);
}

// Find extreme movers (>10% change)
let big_movers: Vec<_> = gainers.iter()
    .filter(|m| {
        m.percent_change
            .trim_start_matches('+')
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0) > 10.0
    })
    .collect();
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
