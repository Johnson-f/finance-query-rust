# finance-query-core

A Rust client library for Yahoo Finance API. Fetch quotes, historical data, financials, news, and more.

## Features

- Framework-agnostic design - use with any Rust application
- Automatic authentication handling with cookie/crumb management
- Strongly-typed data models for all Yahoo Finance responses
- Comprehensive error handling
- WebSocket support for real-time data

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
    // Create client components
    let fetch_client = Arc::new(FetchClient::new());
    let auth_manager = Arc::new(YahooAuthManager::new(fetch_client.clone()));
    let client = YahooFinanceClient::new(auth_manager, fetch_client);

    // Fetch a quote
    let quote = client.get_quote("AAPL").await?;
    println!("Price: {}", quote["regularMarketPrice"]);

    Ok(())
}
```

## Usage Examples

### Fetching Stock Quotes

```rust
// Get a simple quote
let quote = client.get_quote("AAPL").await?;

// Get multiple quotes
let quotes = client.get_quotes(&["AAPL", "GOOGL", "MSFT"]).await?;
```

### Historical Data

```rust
use finance_query_core::{TimeRange, Interval};

// Get historical data for the past month with daily intervals
let history = client.get_historical("AAPL", TimeRange::OneMonth, Interval::OneDay).await?;
```

### Financial Statements

```rust
use finance_query_core::{StatementType, Frequency};

// Get income statement
let financials = client.get_financials("AAPL", StatementType::Income, Frequency::Annual).await?;
```

### News

```rust
// Get news for a symbol
let news = client.get_news("AAPL").await?;
```

### Search

```rust
// Search for symbols
let results = client.search("Apple").await?;
```

## Available Models

### Quote Models
- `Quote` - Full quote data with all available fields
- `SimpleQuote` - Simplified quote with essential fields
- `DetailedQuote` - Extended quote with additional details

### Historical Models
- `HistoricalData` - OHLCV price data
- `HistoricalResponse` - Response wrapper for historical queries
- `TimeRange` - Time range options (1d, 5d, 1mo, 3mo, 6mo, 1y, 5y, max)
- `Interval` - Data interval options (1m, 5m, 15m, 1h, 1d, 1wk, 1mo)

### Financial Models
- `FinancialStatement` - Income, balance sheet, and cash flow data
- `StatementType` - Type of financial statement
- `Frequency` - Annual or quarterly frequency

### News & Search
- `News` - News article data
- `SearchResult` - Individual search result
- `SearchResponse` - Search response wrapper

### Market Data
- `MarketMover` - Top gainers/losers data
- `MoverCount` - Count options for movers
- `MarketIndex` - Index data (S&P 500, Dow Jones, etc.)
- `MarketSector` - Sector performance data

### Holders & Analysts
- `InstitutionalHolder` - Institutional ownership data
- `MutualFundHolder` - Mutual fund holdings
- `InsiderTransaction` - Insider trading data
- `RecommendationData` - Analyst recommendations
- `PriceTarget` - Analyst price targets
- `EarningsEstimate` - Earnings estimates

### Earnings
- `EarningsTranscript` - Earnings call transcript
- `EarningsCallListing` - List of available earnings calls

### WebSocket Types
- `QuotesUpdate` - Real-time streaming quotes for single or multiple stocks
- `ProfileUpdate` - Real-time profile updates
- `MoversUpdate` - Real-time market movers
- `MarketHours` - Market hours information
- `MovingAverageUpdate` - Moving average calculations

## WebSocket Support

The crate provides framework-agnostic data structures for real-time WebSocket streaming. These types can be used with any WebSocket framework (tokio-tungstenite, actix-web, axum, etc.).

### WebSocket Types

#### QuotesUpdate

Stream real-time quotes for one or multiple stocks. This is the primary type for quote streaming.

```rust
use finance_query_core::{QuotesUpdate, SimpleQuote};
use chrono::Utc;

// Create a single quote update
let quote = SimpleQuote {
    symbol: "AAPL".to_string(),
    name: "Apple Inc.".to_string(),
    price: "175.50".to_string(),
    pre_market_price: None,
    after_hours_price: None,
    change: "+2.50".to_string(),
    percent_change: "+1.45%".to_string(),
    logo: None,
};
let update = QuotesUpdate::single(quote);

// Create a multi-quote update
let quotes = vec![aapl_quote, googl_quote, msft_quote];
let update = QuotesUpdate::multiple(quotes);

// With custom timestamp
let update = QuotesUpdate::with_timestamp(quotes, Utc::now());

// Helper methods
update.contains_symbol("AAPL");  // Check if symbol is in update
update.get_quote("AAPL");        // Get specific quote
update.len();                     // Number of quotes
update.is_empty();               // Check if empty

// Serialize for WebSocket transmission
let json = serde_json::to_string(&update)?;
```

#### ProfileUpdate

Real-time updates for a stock profile including quote, similar stocks, sector performance, and news.

```rust
use finance_query_core::ProfileUpdate;

// ProfileUpdate contains:
// - quote: Option<Quote> - Current quote data
// - similar: Option<Vec<SimpleQuote>> - Similar stocks
// - sector_performance: Option<MarketSector> - Sector data
// - news: Option<Vec<News>> - Recent news

let update = ProfileUpdate {
    quote: Some(quote),
    similar: Some(vec![similar_stock]),
    sector_performance: Some(sector),
    news: Some(vec![news_item]),
};

// Serialize for WebSocket transmission
let json = serde_json::to_string(&update)?;
```

#### MoversUpdate

Real-time market movers data including most active, top gainers, and top losers.

```rust
use finance_query_core::MoversUpdate;

// MoversUpdate contains:
// - actives: Option<Vec<MarketMover>> - Most active stocks
// - gainers: Option<Vec<MarketMover>> - Top gaining stocks
// - losers: Option<Vec<MarketMover>> - Top losing stocks

let movers = MoversUpdate {
    actives: Some(actives_list),
    gainers: Some(gainers_list),
    losers: Some(losers_list),
};
```

#### MarketHours

Market hours status updates.

```rust
use finance_query_core::MarketHours;
use chrono::Utc;

// MarketHours contains:
// - status: String - "open", "closed", "pre-market", "after-hours"
// - reason: Option<String> - Holiday name or other reason
// - timestamp: DateTime<Utc> - When the status was updated

let hours = MarketHours {
    status: "open".to_string(),
    reason: None,
    timestamp: Utc::now(),
};
```

#### MovingAverageUpdate

Real-time moving average indicator updates.

```rust
use finance_query_core::MovingAverageUpdate;
use chrono::Utc;

// MovingAverageUpdate contains:
// - symbol: String - Stock symbol
// - indicator_type: String - "SMA" or "EMA"
// - period: i32 - Moving average period
// - value: f64 - Calculated value
// - timestamp: DateTime<Utc> - Calculation time

let ma_update = MovingAverageUpdate {
    symbol: "AAPL".to_string(),
    indicator_type: "SMA".to_string(),
    period: 20,
    value: 175.50,
    timestamp: Utc::now(),
};
```

### WebSocket Integration Example

Here's an example of using these types with a WebSocket server:

```rust
use finance_query_core::{ProfileUpdate, MoversUpdate, MarketHours};
use serde_json;

// Serialize updates for transmission
fn send_profile_update(update: &ProfileUpdate) -> String {
    serde_json::to_string(update).unwrap()
}

// Deserialize received messages
fn parse_movers_update(json: &str) -> Result<MoversUpdate, serde_json::Error> {
    serde_json::from_str(json)
}

// All WebSocket types implement:
// - Debug, Clone for flexibility
// - Serialize, Deserialize for JSON encoding
```

## Error Handling

The library uses `YahooError` for all error cases:

```rust
use finance_query_core::YahooError;

match client.get_quote("INVALID").await {
    Ok(quote) => println!("Got quote"),
    Err(YahooError::NotFound(msg)) => println!("Symbol not found: {}", msg),
    Err(YahooError::AuthFailed(msg)) => println!("Auth failed: {}", msg),
    Err(YahooError::RateLimited) => println!("Rate limited, try again later"),
    Err(YahooError::HttpError(code, msg)) => println!("HTTP {}: {}", code, msg),
    Err(YahooError::ParseError(msg)) => println!("Parse error: {}", msg),
    Err(YahooError::NetworkError(e)) => println!("Network error: {}", e),
}
```

### Error Types

| Error | Description |
|-------|-------------|
| `AuthFailed` | Authentication with Yahoo Finance failed |
| `NotFound` | Requested resource (symbol, data) not found |
| `RateLimited` | Too many requests, rate limit exceeded |
| `HttpError` | HTTP error with status code |
| `ParseError` | Failed to parse response data |
| `NetworkError` | Network-level error |

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
