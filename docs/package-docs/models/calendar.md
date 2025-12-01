# Calendar Events API

The Calendar API provides access to upcoming corporate events including earnings announcements and dividend payments.

## Overview

This module provides calendar data for:

- **Earnings Dates**: Scheduled earnings announcement dates (single date or date range)
- **Dividend Dates**: Upcoming dividend payment dates
- **Ex-Dividend Dates**: Last date to buy stock to receive dividend
- **Dividend Information**: Dividend rate and yield

## Data Structure

### Calendar

```rust
pub struct Calendar {
    pub symbol: String,
    pub earnings_date: Option<DateTime<Utc>>,
    pub earnings_date_start: Option<DateTime<Utc>>,
    pub earnings_date_end: Option<DateTime<Utc>>,
    pub dividend_date: Option<DateTime<Utc>>,
    pub ex_dividend_date: Option<DateTime<Utc>>,
    pub dividend_rate: Option<f64>,
    pub dividend_yield: Option<f64>,
}
```

**Fields:**
- `symbol`: Stock ticker symbol
- `earnings_date`: Primary earnings date (typically the start of the range)
- `earnings_date_start`: Start of earnings date range
- `earnings_date_end`: End of earnings date range
- `dividend_date`: Next dividend payment date
- `ex_dividend_date`: Ex-dividend date (last day to buy for dividend)
- `dividend_rate`: Dividend amount per share
- `dividend_yield`: Annual dividend yield as percentage


## Usage Examples

### Basic Calendar Retrieval

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let calendar = client.get_calendar("AAPL").await?;
    
    println!("Calendar for {}", calendar.symbol);
    
    if let Some(earnings) = calendar.earnings_date {
        println!("Next Earnings: {}", earnings.format("%Y-%m-%d"));
    }
    
    if let Some(dividend) = calendar.dividend_date {
        println!("Next Dividend: {}", dividend.format("%Y-%m-%d"));
    }
    
    if let Some(ex_div) = calendar.ex_dividend_date {
        println!("Ex-Dividend Date: {}", ex_div.format("%Y-%m-%d"));
    }
    
    Ok(())
}
```

### Earnings Date Range

```rust
use finance_query_core::YahooClient;
use chrono::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let calendar = client.get_calendar("TSLA").await?;
    
    println!("Earnings Schedule for {}:", calendar.symbol);
    
    match (calendar.earnings_date_start, calendar.earnings_date_end) {
        (Some(start), Some(end)) => {
            println!("Earnings Window: {} to {}", 
                start.format("%Y-%m-%d"),
                end.format("%Y-%m-%d")
            );
            
            let duration = end.signed_duration_since(start);
            println!("Window Duration: {} days", duration.num_days());
            
            // Check if earnings are soon
            let now = chrono::Utc::now();
            let days_until = start.signed_duration_since(now).num_days();
            
            if days_until <= 7 && days_until >= 0 {
                println!("⚠️  Earnings coming within 7 days!");
            } else if days_until > 0 {
                println!("Days until earnings: {}", days_until);
            }
        }
        (Some(date), None) => {
            println!("Earnings Date: {}", date.format("%Y-%m-%d"));
        }
        _ => {
            println!("No earnings date scheduled");
        }
    }
    
    Ok(())
}
```

### Dividend Calendar

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let calendar = client.get_calendar("KO").await?;
    
    println!("Dividend Information for {}:\n", calendar.symbol);
    
    if let Some(rate) = calendar.dividend_rate {
        println!("Dividend Rate: ${:.2} per share", rate);
    }
    
    if let Some(yield_pct) = calendar.dividend_yield {
        println!("Dividend Yield: {:.2}%", yield_pct);
    }
    
    if let Some(ex_date) = calendar.ex_dividend_date {
        println!("\nEx-Dividend Date: {}", ex_date.format("%Y-%m-%d"));
        println!("(Buy before this date to receive dividend)");
    }
    
    if let Some(pay_date) = calendar.dividend_date {
        println!("Payment Date: {}", pay_date.format("%Y-%m-%d"));
        
        // Calculate days until payment
        let now = chrono::Utc::now();
        let days_until = pay_date.signed_duration_since(now).num_days();
        
        if days_until > 0 {
            println!("Days until payment: {}", days_until);
        }
    }
    
    Ok(())
}
```

### Multiple Stocks Calendar

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN"];
    
    println!("Upcoming Earnings Calendar:\n");
    
    for symbol in symbols {
        match client.get_calendar(symbol).await {
            Ok(calendar) => {
                print!("{}: ", calendar.symbol);
                
                if let Some(date) = calendar.earnings_date {
                    println!("{}", date.format("%Y-%m-%d"));
                } else {
                    println!("Not scheduled");
                }
            }
            Err(e) => {
                println!("{}: Error - {}", symbol, e);
            }
        }
    }
    
    Ok(())
}
```

### Earnings Alert System

```rust
use finance_query_core::YahooClient;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let watchlist = vec!["AAPL", "NVDA", "TSLA", "META"];
    
    let now = Utc::now();
    let week_from_now = now + Duration::days(7);
    
    println!("Earnings Alerts (Next 7 Days):\n");
    
    for symbol in watchlist {
        if let Ok(calendar) = client.get_calendar(symbol).await {
            if let Some(earnings_date) = calendar.earnings_date {
                if earnings_date >= now && earnings_date <= week_from_now {
                    let days = earnings_date.signed_duration_since(now).num_days();
                    
                    println!("🔔 {} - Earnings in {} days", 
                        calendar.symbol, 
                        days
                    );
                    println!("   Date: {}", earnings_date.format("%Y-%m-%d %H:%M UTC"));
                    
                    if let (Some(start), Some(end)) = 
                        (calendar.earnings_date_start, calendar.earnings_date_end) {
                        println!("   Window: {} to {}", 
                            start.format("%m/%d"),
                            end.format("%m/%d")
                        );
                    }
                    println!();
                }
            }
        }
    }
    
    Ok(())
}
```

### Dividend Eligibility Checker

```rust
use finance_query_core::YahooClient;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let calendar = client.get_calendar("JNJ").await?;
    
    println!("Dividend Eligibility for {}:\n", calendar.symbol);
    
    if let Some(ex_date) = calendar.ex_dividend_date {
        let now = Utc::now();
        
        if now < ex_date {
            let days_left = ex_date.signed_duration_since(now).num_days();
            println!("✓ Still eligible for next dividend");
            println!("  Buy before: {}", ex_date.format("%Y-%m-%d"));
            println!("  Days remaining: {}", days_left);
            
            if let Some(rate) = calendar.dividend_rate {
                println!("  Dividend amount: ${:.2}", rate);
            }
        } else {
            println!("✗ Too late for this dividend");
            println!("  Ex-dividend date passed: {}", ex_date.format("%Y-%m-%d"));
        }
        
        if let Some(pay_date) = calendar.dividend_date {
            println!("\nPayment date: {}", pay_date.format("%Y-%m-%d"));
        }
    } else {
        println!("No upcoming dividend scheduled");
    }
    
    Ok(())
}
```

### Calendar Summary

```rust
use finance_query_core::YahooClient;

async fn print_calendar_summary(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let calendar = client.get_calendar(symbol).await?;
    
    println!("═══════════════════════════════════");
    println!("  Calendar Summary: {}", calendar.symbol);
    println!("═══════════════════════════════════");
    
    // Earnings section
    println!("\n📊 EARNINGS");
    match (calendar.earnings_date_start, calendar.earnings_date_end) {
        (Some(start), Some(end)) if start != end => {
            println!("  Range: {} to {}", 
                start.format("%b %d, %Y"),
                end.format("%b %d, %Y")
            );
        }
        (Some(date), _) => {
            println!("  Date: {}", date.format("%b %d, %Y"));
        }
        _ => {
            println!("  Not scheduled");
        }
    }
    
    // Dividend section
    println!("\n💰 DIVIDEND");
    if let Some(rate) = calendar.dividend_rate {
        println!("  Rate: ${:.2}", rate);
    }
    if let Some(yield_pct) = calendar.dividend_yield {
        println!("  Yield: {:.2}%", yield_pct);
    }
    if let Some(ex_date) = calendar.ex_dividend_date {
        println!("  Ex-Date: {}", ex_date.format("%b %d, %Y"));
    }
    if let Some(pay_date) = calendar.dividend_date {
        println!("  Pay Date: {}", pay_date.format("%b %d, %Y"));
    }
    
    if calendar.dividend_rate.is_none() && calendar.ex_dividend_date.is_none() {
        println!("  No dividend scheduled");
    }
    
    println!("\n═══════════════════════════════════\n");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_calendar_summary("AAPL").await?;
    Ok(())
}
```


## JSON Response Formats

### Complete Calendar Response

```json
{
  "symbol": "AAPL",
  "earningsDate": "2024-11-01T20:00:00Z",
  "earningsDateStart": "2024-11-01T20:00:00Z",
  "earningsDateEnd": "2024-11-05T21:00:00Z",
  "dividendDate": "2024-11-14T00:00:00Z",
  "exDividendDate": "2024-11-08T00:00:00Z",
  "dividendRate": 0.24,
  "dividendYield": 0.52
}
```

### Earnings Only

```json
{
  "symbol": "TSLA",
  "earningsDate": "2024-10-23T20:00:00Z",
  "earningsDateStart": "2024-10-23T20:00:00Z",
  "earningsDateEnd": "2024-10-27T21:00:00Z"
}
```

### Dividend Only

```json
{
  "symbol": "KO",
  "dividendDate": "2024-12-15T00:00:00Z",
  "exDividendDate": "2024-11-29T00:00:00Z",
  "dividendRate": 0.46,
  "dividendYield": 3.12
}
```

### No Scheduled Events

```json
{
  "symbol": "NEWCO"
}
```

### Single Earnings Date

When the earnings date range is a single day:

```json
{
  "symbol": "MSFT",
  "earningsDate": "2024-10-30T20:00:00Z",
  "earningsDateStart": "2024-10-30T20:00:00Z",
  "earningsDateEnd": "2024-10-30T20:00:00Z"
}
```

## Field Details

### Earnings Dates

**Date Format:**
- All dates are ISO 8601 formatted UTC timestamps
- Format: `YYYY-MM-DDTHH:MM:SSZ`
- Timezone: Always UTC (indicated by `Z`)

**Date Range:**
- `earningsDate`: Primary date (usually same as `earningsDateStart`)
- `earningsDateStart`: Beginning of the earnings window
- `earningsDateEnd`: End of the earnings window
- If start and end are the same, earnings are scheduled for a specific date
- If different, earnings could occur anytime within the range

**Timing:**
- Earnings are typically announced after market close (20:00 UTC / 4:00 PM ET)
- Or before market open (12:00 UTC / 8:00 AM ET)
- Check the hour component to determine timing

### Dividend Dates

**Ex-Dividend Date:**
- Last day to purchase stock to receive the dividend
- Must own stock before market open on this date
- Stock typically drops by dividend amount on this date

**Payment Date:**
- Date when dividend is actually paid to shareholders
- Usually 2-4 weeks after ex-dividend date

**Dividend Rate:**
- Amount paid per share
- Expressed in dollars (e.g., 0.24 = $0.24 per share)
- Typically represents quarterly payment for US stocks

**Dividend Yield:**
- Annual dividend yield as a percentage
- Calculated as: (Annual Dividend / Current Price) × 100
- Example: 0.52 = 0.52% yield

## Common Use Cases

### 1. Earnings Calendar Widget

```rust
use finance_query_core::YahooClient;
use chrono::{Utc, Duration};

async fn get_this_week_earnings(
    symbols: Vec<&str>
) -> Result<Vec<(String, chrono::DateTime<Utc>)>, Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let mut earnings = Vec::new();
    
    let now = Utc::now();
    let week_end = now + Duration::days(7);
    
    for symbol in symbols {
        if let Ok(calendar) = client.get_calendar(symbol).await {
            if let Some(date) = calendar.earnings_date {
                if date >= now && date <= week_end {
                    earnings.push((calendar.symbol, date));
                }
            }
        }
    }
    
    // Sort by date
    earnings.sort_by_key(|(_, date)| *date);
    
    Ok(earnings)
}
```

### 2. Dividend Capture Strategy

```rust
use finance_query_core::YahooClient;
use chrono::{Utc, Duration};

async fn find_dividend_opportunities(
    symbols: Vec<&str>,
    min_yield: f64,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let mut opportunities = Vec::new();
    
    let now = Utc::now();
    let cutoff = now + Duration::days(5);
    
    for symbol in symbols {
        if let Ok(calendar) = client.get_calendar(symbol).await {
            // Check if ex-dividend date is coming up
            if let Some(ex_date) = calendar.ex_dividend_date {
                if ex_date > now && ex_date <= cutoff {
                    // Check if yield meets minimum
                    if let Some(yield_pct) = calendar.dividend_yield {
                        if yield_pct >= min_yield {
                            opportunities.push(calendar.symbol);
                        }
                    }
                }
            }
        }
    }
    
    Ok(opportunities)
}
```

### 3. Earnings Volatility Warning

```rust
use finance_query_core::YahooClient;
use chrono::{Utc, Duration};

async fn check_earnings_risk(
    symbol: &str
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let calendar = client.get_calendar(symbol).await?;
    
    if let Some(earnings_date) = calendar.earnings_date {
        let now = Utc::now();
        let days_until = earnings_date.signed_duration_since(now).num_days();
        
        // Warn if earnings within 3 days
        if days_until >= 0 && days_until <= 3 {
            println!("⚠️  WARNING: {} has earnings in {} days", symbol, days_until);
            println!("   Expected volatility may be high");
            return Ok(true);
        }
    }
    
    Ok(false)
}
```

### 4. Dividend Payment Calculator

```rust
use finance_query_core::YahooClient;

async fn calculate_dividend_income(
    symbol: &str,
    shares: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let calendar = client.get_calendar(symbol).await?;
    
    if let Some(rate) = calendar.dividend_rate {
        let payment = rate * shares;
        let annual = payment * 4.0; // Assuming quarterly
        
        println!("Dividend Income for {} ({} shares):", symbol, shares);
        println!("  Per Payment: ${:.2}", payment);
        println!("  Annual (est): ${:.2}", annual);
        
        if let Some(pay_date) = calendar.dividend_date {
            println!("  Next Payment: {}", pay_date.format("%Y-%m-%d"));
        }
    } else {
        println!("{} does not pay dividends", symbol);
    }
    
    Ok(())
}
```

### 5. Calendar Comparison

```rust
use finance_query_core::YahooClient;

async fn compare_calendars(
    symbol1: &str,
    symbol2: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let cal1 = client.get_calendar(symbol1).await?;
    let cal2 = client.get_calendar(symbol2).await?;
    
    println!("Calendar Comparison:\n");
    
    // Compare earnings
    match (cal1.earnings_date, cal2.earnings_date) {
        (Some(d1), Some(d2)) => {
            println!("Earnings:");
            println!("  {}: {}", cal1.symbol, d1.format("%Y-%m-%d"));
            println!("  {}: {}", cal2.symbol, d2.format("%Y-%m-%d"));
            
            let diff = d1.signed_duration_since(d2).num_days().abs();
            println!("  Days apart: {}", diff);
        }
        _ => println!("Earnings: Not both scheduled"),
    }
    
    // Compare dividends
    match (cal1.dividend_yield, cal2.dividend_yield) {
        (Some(y1), Some(y2)) => {
            println!("\nDividend Yields:");
            println!("  {}: {:.2}%", cal1.symbol, y1);
            println!("  {}: {:.2}%", cal2.symbol, y2);
            
            if y1 > y2 {
                println!("  {} yields {:.2}% more", cal1.symbol, y1 - y2);
            } else {
                println!("  {} yields {:.2}% more", cal2.symbol, y2 - y1);
            }
        }
        _ => println!("\nDividend Yields: Not both available"),
    }
    
    Ok(())
}
```

## Important Notes

### Earnings Dates

- Earnings dates are estimates and can change
- Companies may announce earnings outside the scheduled window
- After-hours earnings typically occur at 20:00 UTC (4:00 PM ET)
- Pre-market earnings typically occur at 12:00 UTC (8:00 AM ET)
- Always verify with official company announcements

### Dividend Dates

- Ex-dividend date is critical for dividend eligibility
- Must own stock before market open on ex-dividend date
- Stock price typically drops by dividend amount on ex-date
- Payment date is when dividend is deposited to account
- Dividend rate shown is typically per-quarter for US stocks

### Data Availability

- Not all stocks have scheduled earnings dates
- Growth stocks may not pay dividends
- New listings may not have historical calendar data
- Calendar data is updated regularly but may lag
- Some fields may be `None` if data is unavailable

### Time Zones

- All timestamps are in UTC
- Convert to local timezone for display
- Market hours are typically:
  - US Regular: 14:30-21:00 UTC (9:30 AM - 4:00 PM ET)
  - US Pre-market: 09:00-14:30 UTC (4:00 AM - 9:30 AM ET)
  - US After-hours: 21:00-01:00 UTC (4:00 PM - 8:00 PM ET)

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_calendar("AAPL").await {
        Ok(calendar) => {
            if calendar.earnings_date.is_none() && calendar.dividend_date.is_none() {
                println!("No upcoming events scheduled");
            } else {
                println!("Calendar data retrieved successfully");
            }
        }
        Err(YahooError::NotFound) => {
            println!("Symbol not found");
        }
        Err(YahooError::ParseError(msg)) => {
            println!("Failed to parse calendar data: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## Related APIs

- **Earnings History**: Get historical earnings results (Analyst API)
- **Actions API**: Get historical dividend payments
- **Quote API**: Get current price for yield calculations
- **Fundamentals API**: Get detailed dividend and earnings metrics

## Best Practices

1. **Cache calendar data** - It doesn't change frequently
2. **Set up alerts** - Monitor for earnings dates within your risk window
3. **Verify ex-dividend dates** - Double-check before dividend capture trades
4. **Account for time zones** - Convert UTC to your local time
5. **Handle missing data** - Not all stocks have complete calendar info
6. **Refresh regularly** - Earnings dates can be rescheduled
7. **Cross-reference** - Verify important dates with company IR pages

