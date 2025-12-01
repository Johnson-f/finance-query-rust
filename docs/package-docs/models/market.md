# Market Model

The Market model provides real-time market status information and summary data for major market indices.

## Overview

The Market module contains structures for tracking market hours, trading status, and index performance. It includes `MarketStatus` for market timing information and `MarketSummaryResponse` for comprehensive market overview data.

## Data Structures

### MarketStatus

Represents the current status and trading hours of a market.

```rust
pub struct MarketStatus {
    pub market: String,
    pub status: String,
    pub open_time: Option<DateTime<Utc>>,
    pub close_time: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub timezone_short: Option<String>,
    pub gmt_offset: Option<i32>,
}
```

**Fields:**
- `market` - Market identifier (e.g., "us_market", "uk_market")
- `status` - Current trading status: "open", "closed", "pre", "post"
- `open_time` - Market opening time in UTC
- `close_time` - Market closing time in UTC
- `timezone` - Full timezone name (e.g., "America/New_York")
- `timezone_short` - Short timezone code (e.g., "EST", "EDT")
- `gmt_offset` - GMT offset in seconds

**Helper Methods:**
```rust
impl MarketStatus {
    pub fn is_open(&self) -> bool;
    pub fn is_pre_market(&self) -> bool;
    pub fn is_after_hours(&self) -> bool;
}
```

### MarketSummaryItem

Represents a single market index or major indicator.

```rust
pub struct MarketSummaryItem {
    pub exchange: String,
    pub short_name: String,
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub percent_change: f64,
}
```

**Fields:**
- `exchange` - Exchange where the index trades
- `short_name` - Display name of the index
- `symbol` - Index symbol (e.g., "^GSPC", "^DJI")
- `price` - Current index value
- `change` - Absolute point change
- `percent_change` - Percentage change

### MarketSummaryResponse

Complete market overview with status and indices.

```rust
pub struct MarketSummaryResponse {
    pub market: String,
    pub status: Option<MarketStatus>,
    pub indices: Vec<MarketSummaryItem>,
}
```

**Fields:**
- `market` - Market identifier
- `status` - Optional market status information
- `indices` - List of major market indices

## JSON Format

### MarketStatus Example

```json
{
  "market": "us_market",
  "status": "open",
  "openTime": "2024-12-01T14:30:00Z",
  "closeTime": "2024-12-01T21:00:00Z",
  "timezone": "America/New_York",
  "timezoneShort": "EST",
  "gmtOffset": -18000
}
```

### MarketSummaryResponse Example

```json
{
  "market": "us_market",
  "status": {
    "market": "us_market",
    "status": "open",
    "openTime": "2024-12-01T14:30:00Z",
    "closeTime": "2024-12-01T21:00:00Z",
    "timezone": "America/New_York",
    "timezoneShort": "EST",
    "gmtOffset": -18000
  },
  "indices": [
    {
      "exchange": "SNP",
      "shortName": "S&P 500",
      "symbol": "^GSPC",
      "price": 4567.89,
      "change": 23.45,
      "percentChange": 0.52
    },
    {
      "exchange": "DJI",
      "shortName": "Dow Jones Industrial Average",
      "symbol": "^DJI",
      "price": 35678.90,
      "change": -45.67,
      "percentChange": -0.13
    },
    {
      "exchange": "NASDAQ",
      "shortName": "NASDAQ Composite",
      "symbol": "^IXIC",
      "price": 14234.56,
      "change": 78.90,
      "percentChange": 0.56
    },
    {
      "exchange": "NYE",
      "shortName": "NYSE Composite",
      "symbol": "^NYA",
      "price": 16789.12,
      "change": 12.34,
      "percentChange": 0.07
    },
    {
      "exchange": "AMEX",
      "shortName": "Russell 2000",
      "symbol": "^RUT",
      "price": 1987.65,
      "change": -5.43,
      "percentChange": -0.27
    }
  ]
}
```

### Field Descriptions

**Status Values:**
- `"open"` - Market is currently in regular trading hours
- `"closed"` - Market is closed
- `"pre"` - Pre-market trading session
- `"post"` - After-hours trading session

**Time Fields:**
- All timestamps are in ISO 8601 format with UTC timezone
- `gmtOffset` is in seconds (e.g., -18000 = -5 hours for EST)

**Price Fields:**
- `price` - Current index level
- `change` - Absolute point change from previous close
- `percentChange` - Percentage change (e.g., 0.52 = 0.52%)

## Usage Examples

### Check Market Status

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Get US market status
    let status = client.get_market_status("us_market").await?;
    
    println!("Market: {}", status.market);
    println!("Status: {}", status.status);
    
    if status.is_open() {
        println!("✓ Market is currently open for trading");
    } else if status.is_pre_market() {
        println!("⏰ Pre-market trading session");
    } else if status.is_after_hours() {
        println!("🌙 After-hours trading session");
    } else {
        println!("✗ Market is closed");
    }
    
    if let Some(open) = status.open_time {
        println!("Opens at: {}", open);
    }
    
    if let Some(close) = status.close_time {
        println!("Closes at: {}", close);
    }
    
    Ok(())
}
```

### Get Market Summary

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Get complete market summary
    let summary = client.get_market_summary("us_market").await?;
    
    println!("Market Summary - {}", summary.market);
    println!("{}", "=".repeat(70));
    
    if let Some(status) = &summary.status {
        println!("Status: {} ({})", 
            status.status.to_uppercase(), 
            status.timezone_short.as_deref().unwrap_or("UTC")
        );
        println!();
    }
    
    println!("{:<30} {:>12} {:>12} {:>12}", 
        "Index", "Price", "Change", "% Change"
    );
    println!("{}", "-".repeat(70));
    
    for index in &summary.indices {
        let change_sign = if index.change >= 0.0 { "+" } else { "" };
        println!("{:<30} {:>12.2} {:>12.2} {:>11.2}%", 
            index.short_name,
            index.price,
            index.change,
            index.percent_change
        );
    }
    
    Ok(())
}
```

### Market Dashboard

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let summary = client.get_market_summary("us_market").await?;
    
    println!("📊 MARKET DASHBOARD");
    println!("{}", "=".repeat(70));
    
    // Market status
    if let Some(status) = &summary.status {
        let status_emoji = match status.status.as_str() {
            "open" => "🟢",
            "closed" => "🔴",
            "pre" => "🟡",
            "post" => "🟠",
            _ => "⚪",
        };
        println!("{} Market Status: {}", status_emoji, status.status.to_uppercase());
        println!();
    }
    
    // Calculate market sentiment
    let gainers = summary.indices.iter().filter(|i| i.change > 0.0).count();
    let losers = summary.indices.iter().filter(|i| i.change < 0.0).count();
    
    println!("Market Breadth:");
    println!("  Gainers: {} | Losers: {}", gainers, losers);
    println!();
    
    // Show indices with color coding
    for index in &summary.indices {
        let emoji = if index.change > 0.0 { "📈" } else { "📉" };
        println!("{} {} ({})", emoji, index.short_name, index.symbol);
        println!("   Price: {:.2} | Change: {:+.2} ({:+.2}%)", 
            index.price, index.change, index.percent_change
        );
    }
    
    Ok(())
}
```

### Trading Hours Calculator

```rust
use finance_query_core::YahooClient;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let status = client.get_market_status("us_market").await?;
    
    let now = Utc::now();
    
    println!("Trading Hours Information");
    println!("{}", "=".repeat(50));
    println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S UTC"));
    println!();
    
    if let Some(open) = status.open_time {
        if now < open {
            let time_until = open.signed_duration_since(now);
            println!("⏰ Market opens in: {} hours {} minutes", 
                time_until.num_hours(), 
                time_until.num_minutes() % 60
            );
        }
    }
    
    if let Some(close) = status.close_time {
        if now < close {
            let time_until = close.signed_duration_since(now);
            println!("⏰ Market closes in: {} hours {} minutes", 
                time_until.num_hours(), 
                time_until.num_minutes() % 60
            );
        } else {
            let time_since = now.signed_duration_since(close);
            println!("🔒 Market closed {} hours {} minutes ago", 
                time_since.num_hours(), 
                time_since.num_minutes() % 60
            );
        }
    }
    
    Ok(())
}
```

### Market Sentiment Analysis

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let summary = client.get_market_summary("us_market").await?;
    
    // Calculate average performance
    let total_change: f64 = summary.indices.iter()
        .map(|i| i.percent_change)
        .sum();
    let avg_change = total_change / summary.indices.len() as f64;
    
    // Determine sentiment
    let sentiment = if avg_change > 0.5 {
        "🚀 Bullish"
    } else if avg_change > 0.0 {
        "📈 Slightly Bullish"
    } else if avg_change > -0.5 {
        "📉 Slightly Bearish"
    } else {
        "🔻 Bearish"
    };
    
    println!("Market Sentiment Analysis");
    println!("{}", "=".repeat(50));
    println!("Overall Sentiment: {}", sentiment);
    println!("Average Change: {:.2}%", avg_change);
    println!();
    
    // Find best and worst performers
    let best = summary.indices.iter()
        .max_by(|a, b| a.percent_change.partial_cmp(&b.percent_change).unwrap());
    let worst = summary.indices.iter()
        .min_by(|a, b| a.percent_change.partial_cmp(&b.percent_change).unwrap());
    
    if let Some(best) = best {
        println!("🏆 Best Performer: {} ({:+.2}%)", 
            best.short_name, best.percent_change
        );
    }
    
    if let Some(worst) = worst {
        println!("📊 Worst Performer: {} ({:+.2}%)", 
            worst.short_name, worst.percent_change
        );
    }
    
    Ok(())
}
```

### Monitor Market Opening

```rust
use finance_query_core::YahooClient;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    println!("Monitoring market status...");
    println!("Press Ctrl+C to stop");
    println!();
    
    loop {
        let status = client.get_market_status("us_market").await?;
        
        let status_display = match status.status.as_str() {
            "open" => "🟢 OPEN",
            "closed" => "🔴 CLOSED",
            "pre" => "🟡 PRE-MARKET",
            "post" => "🟠 AFTER-HOURS",
            _ => "⚪ UNKNOWN",
        };
        
        println!("[{}] Market Status: {}", 
            chrono::Utc::now().format("%H:%M:%S"), 
            status_display
        );
        
        // Check every 60 seconds
        sleep(Duration::from_secs(60)).await;
    }
}
```

### Export Market Data

```rust
use finance_query_core::YahooClient;
use serde_json;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let summary = client.get_market_summary("us_market").await?;
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&summary)?;
    
    // Save to file with timestamp
    let filename = format!(
        "market_summary_{}.json", 
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );
    fs::write(&filename, json)?;
    
    println!("Market data saved to: {}", filename);
    
    Ok(())
}
```

### Compare Multiple Markets

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let markets = vec!["us_market", "uk_market", "asia_market"];
    
    println!("Global Market Overview");
    println!("{}", "=".repeat(70));
    
    for market_id in markets {
        match client.get_market_status(market_id).await {
            Ok(status) => {
                println!("\n{} - {}", 
                    market_id.to_uppercase(), 
                    status.status.to_uppercase()
                );
                
                if let Some(tz) = status.timezone_short {
                    println!("Timezone: {}", tz);
                }
                
                if status.is_open() {
                    println!("✓ Currently trading");
                } else {
                    println!("✗ Market closed");
                }
            }
            Err(e) => {
                eprintln!("Error fetching {}: {}", market_id, e);
            }
        }
    }
    
    Ok(())
}
```

## Common Market Identifiers

- `us_market` - United States markets (NYSE, NASDAQ)
- `uk_market` - United Kingdom markets (LSE)
- `asia_market` - Asian markets
- `europe_market` - European markets

## Major Index Symbols

Common symbols you'll see in market summaries:

- `^GSPC` - S&P 500
- `^DJI` - Dow Jones Industrial Average
- `^IXIC` - NASDAQ Composite
- `^NYA` - NYSE Composite
- `^RUT` - Russell 2000
- `^VIX` - CBOE Volatility Index

## Notes

- Market status is updated in real-time
- Times are provided in UTC and should be converted to local time as needed
- Pre-market typically runs from 4:00 AM to 9:30 AM ET
- Regular hours are 9:30 AM to 4:00 PM ET
- After-hours trading runs from 4:00 PM to 8:00 PM ET
- The `gmt_offset` is in seconds (divide by 3600 for hours)
- Index prices are point values, not dollar amounts
- Percentage changes are already in percentage form (0.52 = 0.52%, not 52%)

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_market_summary("us_market").await {
        Ok(summary) => {
            println!("Market data retrieved successfully");
            println!("Indices: {}", summary.indices.len());
        }
        Err(YahooError::NotFound) => {
            eprintln!("Market not found");
        }
        Err(YahooError::Network(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(YahooError::ParseError(e)) => {
            eprintln!("Failed to parse response: {}", e);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## See Also

- [Indices Model](./indicies.md) - For detailed index data
- [Quote Model](./quote.md) - For individual stock quotes
- [Sectors Model](./sectors.md) - For sector performance data
- [Industry Model](./industry.md) - For industry-specific data
