# Stock Actions API

The Actions API provides access to corporate actions data including dividends, stock splits, and capital gains distributions for stocks, ETFs, and mutual funds.

## Overview

Corporate actions are significant events that affect a company's stock and shareholders. This module provides structured access to:

- **Dividends**: Cash payments distributed to shareholders
- **Stock Splits**: Changes in the number of shares outstanding
- **Capital Gains**: Distributions from ETFs and mutual funds

## Data Structures

### ActionsResponse

The main response structure containing all corporate actions for a symbol.

```rust
pub struct ActionsResponse {
    pub symbol: String,
    pub dividends: Vec<Dividend>,
    pub splits: Vec<StockSplit>,
    pub capital_gains: Vec<CapitalGain>,
}
```

**Fields:**
- `symbol`: The stock ticker symbol
- `dividends`: List of dividend payments
- `splits`: List of stock splits
- `capital_gains`: List of capital gain distributions

### Dividend

Represents a dividend payment to shareholders.

```rust
pub struct Dividend {
    pub date: DateTime<Utc>,
    pub amount: f64,
    pub currency: Option<String>,
}
```

**Fields:**
- `date`: Payment date (UTC timestamp)
- `amount`: Dividend amount per share
- `currency`: Currency code (optional, e.g., "USD")

### StockSplit

Represents a stock split event.

```rust
pub struct StockSplit {
    pub date: DateTime<Utc>,
    pub numerator: f64,
    pub denominator: f64,
    pub split_ratio: String,
}
```

**Fields:**
- `date`: Split effective date (UTC timestamp)
- `numerator`: Split numerator (new shares)
- `denominator`: Split denominator (old shares)
- `split_ratio`: Human-readable ratio (e.g., "2:1" for a 2-for-1 split)

**Example Split Ratios:**
- `"2:1"` - 2-for-1 split (shareholders receive 2 shares for every 1 held)
- `"3:2"` - 3-for-2 split (shareholders receive 3 shares for every 2 held)
- `"1:2"` - Reverse split (shareholders receive 1 share for every 2 held)

### CapitalGain

Represents a capital gain distribution (common for ETFs and mutual funds).

```rust
pub struct CapitalGain {
    pub date: DateTime<Utc>,
    pub amount: f64,
}
```

**Fields:**
- `date`: Distribution date (UTC timestamp)
- `amount`: Capital gain amount per share

## Usage Examples

### Basic Usage

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Fetch all corporate actions for Apple
    let actions = client.get_actions("AAPL").await?;
    
    println!("Symbol: {}", actions.symbol);
    println!("Total dividends: {}", actions.dividends.len());
    println!("Total splits: {}", actions.splits.len());
    println!("Total capital gains: {}", actions.capital_gains.len());
    
    Ok(())
}
```

### Working with Dividends

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let actions = client.get_actions("MSFT").await?;
    
    // Calculate total dividend income
    let total = actions.total_dividends();
    println!("Total dividend amount: ${:.2}", total);
    
    // Get most recent dividend
    if let Some(latest) = actions.dividends.last() {
        println!("Latest dividend: ${:.2} on {}", 
            latest.amount, 
            latest.date.format("%Y-%m-%d")
        );
    }
    
    // Filter dividends by year
    let year_2024_divs: Vec<_> = actions.dividends
        .iter()
        .filter(|d| d.date.year() == 2024)
        .collect();
    
    println!("Dividends in 2024: {}", year_2024_divs.len());
    
    Ok(())
}
```

### Analyzing Stock Splits

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let actions = client.get_actions("TSLA").await?;
    
    for split in &actions.splits {
        println!("Split on {}: {} ({}:{})", 
            split.date.format("%Y-%m-%d"),
            split.split_ratio,
            split.numerator,
            split.denominator
        );
        
        // Calculate split multiplier
        let multiplier = split.numerator / split.denominator;
        println!("  Share multiplier: {:.2}x", multiplier);
        
        // Determine split type
        if multiplier > 1.0 {
            println!("  Type: Forward split (increases shares)");
        } else {
            println!("  Type: Reverse split (decreases shares)");
        }
    }
    
    Ok(())
}
```

### Checking for Recent Actions

```rust
use finance_query_core::YahooClient;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let actions = client.get_actions("VTI").await?;
    
    let thirty_days_ago = Utc::now() - Duration::days(30);
    
    // Recent dividends
    let recent_divs: Vec<_> = actions.dividends
        .iter()
        .filter(|d| d.date > thirty_days_ago)
        .collect();
    
    if !recent_divs.is_empty() {
        println!("Recent dividends (last 30 days):");
        for div in recent_divs {
            println!("  ${:.4} on {}", div.amount, div.date.format("%Y-%m-%d"));
        }
    }
    
    // Recent capital gains
    let recent_gains: Vec<_> = actions.capital_gains
        .iter()
        .filter(|g| g.date > thirty_days_ago)
        .collect();
    
    if !recent_gains.is_empty() {
        println!("Recent capital gains (last 30 days):");
        for gain in recent_gains {
            println!("  ${:.4} on {}", gain.amount, gain.date.format("%Y-%m-%d"));
        }
    }
    
    Ok(())
}
```

### Helper Methods

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let actions = client.get_actions("AAPL").await?;
    
    // Check if any actions exist
    if actions.is_empty() {
        println!("No corporate actions found");
    } else {
        println!("Found {} total actions", 
            actions.dividends.len() + 
            actions.splits.len() + 
            actions.capital_gains.len()
        );
    }
    
    // Calculate total dividend payout
    let total_dividends = actions.total_dividends();
    println!("Total dividends paid: ${:.2}", total_dividends);
    
    Ok(())
}
```

## JSON Response Format

### Complete Response

```json
{
  "symbol": "AAPL",
  "dividends": [
    {
      "date": "2024-02-16T00:00:00Z",
      "amount": 0.24,
      "currency": "USD"
    },
    {
      "date": "2024-05-16T00:00:00Z",
      "amount": 0.25
    }
  ],
  "splits": [
    {
      "date": "2020-08-31T00:00:00Z",
      "numerator": 4.0,
      "denominator": 1.0,
      "splitRatio": "4:1"
    }
  ],
  "capitalGains": [
    {
      "date": "2023-12-15T00:00:00Z",
      "amount": 0.15
    }
  ]
}
```

### Empty Response

When no corporate actions are found:

```json
{
  "symbol": "NEWCO",
  "dividends": [],
  "splits": [],
  "capitalGains": []
}
```

### Dividends Only

```json
{
  "symbol": "KO",
  "dividends": [
    {
      "date": "2024-01-01T00:00:00Z",
      "amount": 0.46
    },
    {
      "date": "2024-04-01T00:00:00Z",
      "amount": 0.48
    },
    {
      "date": "2024-07-01T00:00:00Z",
      "amount": 0.48
    },
    {
      "date": "2024-10-01T00:00:00Z",
      "amount": 0.48
    }
  ],
  "splits": [],
  "capitalGains": []
}
```

### Stock Split Example

```json
{
  "symbol": "TSLA",
  "dividends": [],
  "splits": [
    {
      "date": "2020-08-31T00:00:00Z",
      "numerator": 5.0,
      "denominator": 1.0,
      "splitRatio": "5:1"
    },
    {
      "date": "2022-08-25T00:00:00Z",
      "numerator": 3.0,
      "denominator": 1.0,
      "splitRatio": "3:1"
    }
  ],
  "capitalGains": []
}
```

### ETF with Capital Gains

```json
{
  "symbol": "VTI",
  "dividends": [
    {
      "date": "2024-03-25T00:00:00Z",
      "amount": 0.7854
    },
    {
      "date": "2024-06-24T00:00:00Z",
      "amount": 0.8123
    }
  ],
  "splits": [],
  "capitalGains": [
    {
      "date": "2023-12-20T00:00:00Z",
      "amount": 0.3421
    }
  ]
}
```

## Field Details

### Date Format

All dates are returned as ISO 8601 formatted UTC timestamps:
- Format: `YYYY-MM-DDTHH:MM:SSZ`
- Example: `"2024-05-16T00:00:00Z"`
- Timezone: Always UTC (indicated by `Z`)

### Amount Precision

- Dividend amounts: Typically 2-4 decimal places (e.g., `0.24`, `0.7854`)
- Capital gain amounts: Typically 2-4 decimal places
- Split ratios: Whole numbers or decimals (e.g., `2.0`, `1.5`)

### Optional Fields

- `currency`: Only included when available from the data source
- All arrays may be empty if no actions of that type exist

## Data Ordering

All action arrays are automatically sorted by date in ascending order (oldest first):
- `dividends`: Sorted by payment date
- `splits`: Sorted by effective date
- `capital_gains`: Sorted by distribution date

## Common Use Cases

### 1. Dividend Yield Calculation

```rust
// Calculate annual dividend yield
let annual_dividends: f64 = actions.dividends
    .iter()
    .filter(|d| d.date.year() == 2024)
    .map(|d| d.amount)
    .sum();

let current_price = 180.0; // Get from quote API
let dividend_yield = (annual_dividends / current_price) * 100.0;
println!("Dividend yield: {:.2}%", dividend_yield);
```

### 2. Adjusting Historical Prices for Splits

```rust
// Adjust a historical price for all splits since that date
let historical_price = 100.0;
let historical_date = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);

let adjusted_price = actions.splits
    .iter()
    .filter(|s| s.date > historical_date)
    .fold(historical_price, |price, split| {
        price * (split.numerator / split.denominator)
    });

println!("Adjusted price: ${:.2}", adjusted_price);
```

### 3. Dividend Payment Schedule

```rust
// Determine dividend payment frequency
let mut months: Vec<u32> = actions.dividends
    .iter()
    .map(|d| d.date.month())
    .collect();
months.sort();
months.dedup();

let frequency = match months.len() {
    4 => "Quarterly",
    12 => "Monthly",
    2 => "Semi-annual",
    1 => "Annual",
    _ => "Irregular",
};

println!("Dividend frequency: {}", frequency);
```

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_actions("INVALID").await {
        Ok(actions) => {
            if actions.is_empty() {
                println!("No actions found for symbol");
            } else {
                println!("Found {} actions", actions.dividends.len());
            }
        }
        Err(YahooError::NotFound) => {
            println!("Symbol not found");
        }
        Err(YahooError::ParseError(msg)) => {
            println!("Failed to parse response: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## Notes

- All monetary amounts are in the stock's trading currency (typically USD for US stocks)
- Dates represent the ex-dividend date for dividends and effective date for splits
- Historical data availability varies by symbol and exchange
- Some symbols may have no corporate actions (especially newer listings)
- Capital gains are primarily relevant for ETFs and mutual funds
- Data is sourced from Yahoo Finance and updated regularly

## Related APIs

- **Quote API**: Get current price for yield calculations
- **Historical API**: Get price history adjusted for splits
- **Calendar API**: Get upcoming dividend dates
- **Fundamentals API**: Get dividend payout ratio and other metrics
