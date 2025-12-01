# Stock Holders API

The Holders API provides access to ownership data including institutional holders, mutual fund holders, insider transactions, and major shareholder breakdowns.

## Overview

This module provides access to:

- **Major Holders Breakdown**: High-level ownership statistics
- **Institutional Holders**: Top institutional investors and their holdings
- **Mutual Fund Holders**: Top mutual funds holding the stock
- **Insider Transactions**: Recent insider buying and selling activity
- **Insider Purchases Summary**: Aggregated insider trading statistics
- **Insider Roster**: List of company insiders and their positions

## Data Structures

### HolderType

Enum for different holder data types.

```rust
pub enum HolderType {
    Major,                  // "major"
    Institutional,          // "institutional"
    MutualFund,            // "mutualfund"
    InsiderTransactions,   // "insider_transactions"
    InsiderPurchases,      // "insider_purchases"
    InsiderRoster,         // "insider_roster"
}
```

### MajorHoldersBreakdown

High-level ownership statistics.

```rust
pub struct MajorHoldersBreakdown {
    pub breakdown_data: HashMap<String, serde_json::Value>,
}
```

**Common Fields:**
- `insidersPercentHeld`: Percentage held by insiders
- `institutionsPercentHeld`: Percentage held by institutions
- `institutionsFloatPercentHeld`: Percentage of float held by institutions
- `institutionsCount`: Number of institutional holders


### InstitutionalHolder

Individual institutional holder information.

```rust
pub struct InstitutionalHolder {
    pub holder: String,
    pub shares: i64,
    pub date_reported: DateTime<Utc>,
    pub percent_out: Option<f64>,
    pub value: Option<i64>,
}
```

**Fields:**
- `holder`: Name of the institutional holder
- `shares`: Number of shares held
- `date_reported`: Date the holding was reported
- `percent_out`: Percentage of outstanding shares
- `value`: Dollar value of the holding

### MutualFundHolder

Individual mutual fund holder information.

```rust
pub struct MutualFundHolder {
    pub holder: String,
    pub shares: i64,
    pub date_reported: DateTime<Utc>,
    pub percent_out: Option<f64>,
    pub value: Option<i64>,
}
```

**Fields:**
- `holder`: Name of the mutual fund
- `shares`: Number of shares held
- `date_reported`: Date the holding was reported
- `percent_out`: Percentage of outstanding shares
- `value`: Dollar value of the holding

### InsiderTransaction

Individual insider transaction record.

```rust
pub struct InsiderTransaction {
    pub start_date: DateTime<Utc>,
    pub insider: String,
    pub position: String,
    pub transaction: String,
    pub shares: Option<i64>,
    pub value: Option<i64>,
    pub ownership: Option<String>,
}
```

**Fields:**
- `start_date`: Transaction date
- `insider`: Name of the insider
- `position`: Insider's position/title
- `transaction`: Transaction type (e.g., "Sale", "Purchase", "Stock Gift")
- `shares`: Number of shares transacted
- `value`: Dollar value of transaction
- `ownership`: Ownership type (e.g., "Direct", "Indirect")

### InsiderPurchase

Aggregated insider trading summary.

```rust
pub struct InsiderPurchase {
    pub period: String,
    pub purchases_shares: Option<i64>,
    pub purchases_transactions: Option<i64>,
    pub sales_shares: Option<i64>,
    pub sales_transactions: Option<i64>,
    pub net_shares: Option<i64>,
    pub net_transactions: Option<i64>,
    pub total_insider_shares: Option<i64>,
    pub net_percent_insider_shares: Option<f64>,
    pub buy_percent_insider_shares: Option<f64>,
    pub sell_percent_insider_shares: Option<f64>,
}
```

**Fields:**
- `period`: Time period (e.g., "Last 6 Months")
- `purchases_shares`: Total shares purchased
- `purchases_transactions`: Number of purchase transactions
- `sales_shares`: Total shares sold
- `sales_transactions`: Number of sale transactions
- `net_shares`: Net shares (purchases - sales)
- `net_transactions`: Net transactions
- `total_insider_shares`: Total shares held by insiders
- `net_percent_insider_shares`: Net change as percentage
- `buy_percent_insider_shares`: Purchases as percentage
- `sell_percent_insider_shares`: Sales as percentage

### InsiderRosterMember

Company insider roster information.

```rust
pub struct InsiderRosterMember {
    pub name: String,
    pub position: String,
    pub most_recent_transaction: Option<String>,
    pub latest_transaction_date: Option<DateTime<Utc>>,
    pub shares_owned_directly: Option<i64>,
    pub shares_owned_indirectly: Option<i64>,
    pub position_direct_date: Option<DateTime<Utc>>,
}
```

**Fields:**
- `name`: Insider's name
- `position`: Title/position in company
- `most_recent_transaction`: Type of most recent transaction
- `latest_transaction_date`: Date of most recent transaction
- `shares_owned_directly`: Shares owned directly
- `shares_owned_indirectly`: Shares owned indirectly
- `position_direct_date`: Date of direct position

## Usage Examples

### Get Major Holders Breakdown

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let major = client.get_major_holders("AAPL").await?;
    
    println!("Major Holders Breakdown for {}:\n", major.symbol);
    
    for (key, value) in &major.breakdown.breakdown_data {
        println!("{}: {:?}", key, value);
    }
    
    Ok(())
}
```

### Get Top Institutional Holders

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let institutional = client.get_institutional_holders("MSFT").await?;
    
    println!("Top Institutional Holders of {}:\n", institutional.symbol);
    
    for (i, holder) in institutional.holders.iter().enumerate().take(10) {
        println!("{}. {}", i + 1, holder.holder);
        println!("   Shares: {}", holder.shares);
        
        if let Some(percent) = holder.percent_out {
            println!("   Ownership: {:.2}%", percent);
        }
        
        if let Some(value) = holder.value {
            println!("   Value: ${:.2}M", value as f64 / 1_000_000.0);
        }
        
        println!("   Reported: {}", holder.date_reported.format("%Y-%m-%d"));
        println!();
    }
    
    Ok(())
}
```

### Get Mutual Fund Holders

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let mutual_funds = client.get_mutual_fund_holders("GOOGL").await?;
    
    println!("Top Mutual Fund Holders of {}:\n", mutual_funds.symbol);
    
    // Calculate total shares held by top funds
    let total_shares: i64 = mutual_funds.holders.iter()
        .map(|h| h.shares)
        .sum();
    
    println!("Total shares (top funds): {}\n", total_shares);
    
    for holder in mutual_funds.holders.iter().take(5) {
        println!("{}", holder.holder);
        println!("  Shares: {}", holder.shares);
        
        if let Some(percent) = holder.percent_out {
            println!("  Ownership: {:.2}%", percent);
        }
        
        println!("  As of: {}", holder.date_reported.format("%Y-%m-%d"));
        println!();
    }
    
    Ok(())
}
```

### Track Insider Transactions

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transactions = client.get_insider_transactions("TSLA").await?;
    
    println!("Recent Insider Transactions for {}:\n", transactions.symbol);
    
    for txn in &transactions.transactions {
        println!("Date: {}", txn.start_date.format("%Y-%m-%d"));
        println!("Insider: {} ({})", txn.insider, txn.position);
        println!("Transaction: {}", txn.transaction);
        
        if let Some(shares) = txn.shares {
            println!("Shares: {}", shares);
        }
        
        if let Some(value) = txn.value {
            println!("Value: ${:.2}M", value as f64 / 1_000_000.0);
        }
        
        if let Some(ownership) = &txn.ownership {
            println!("Ownership: {}", ownership);
        }
        
        println!();
    }
    
    Ok(())
}
```

### Analyze Insider Buying/Selling

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let purchases = client.get_insider_purchases("NVDA").await?;
    
    println!("Insider Trading Summary for {}:\n", purchases.symbol);
    println!("Period: {}\n", purchases.summary.period);
    
    let buys = purchases.summary.purchases_shares.unwrap_or(0);
    let sells = purchases.summary.sales_shares.unwrap_or(0);
    let net = purchases.summary.net_shares.unwrap_or(0);
    
    println!("Purchases: {} shares", buys);
    println!("Sales: {} shares", sells);
    println!("Net: {} shares", net);
    
    if let Some(buy_pct) = purchases.summary.buy_percent_insider_shares {
        println!("Buy %: {:.2}%", buy_pct);
    }
    
    if let Some(sell_pct) = purchases.summary.sell_percent_insider_shares {
        println!("Sell %: {:.2}%", sell_pct);
    }
    
    // Determine sentiment
    if net > 0 {
        println!("\n✓ BULLISH: Net insider buying");
    } else if net < 0 {
        println!("\n✗ BEARISH: Net insider selling");
    } else {
        println!("\n→ NEUTRAL: No net insider activity");
    }
    
    Ok(())
}
```

### Get Insider Roster

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let roster = client.get_insider_roster("META").await?;
    
    println!("Insider Roster for {}:\n", roster.symbol);
    
    for member in &roster.roster {
        println!("{} - {}", member.name, member.position);
        
        if let Some(direct) = member.shares_owned_directly {
            println!("  Direct ownership: {} shares", direct);
        }
        
        if let Some(indirect) = member.shares_owned_indirectly {
            println!("  Indirect ownership: {} shares", indirect);
        }
        
        if let Some(txn) = &member.most_recent_transaction {
            println!("  Recent transaction: {}", txn);
            
            if let Some(date) = member.latest_transaction_date {
                println!("  Transaction date: {}", date.format("%Y-%m-%d"));
            }
        }
        
        println!();
    }
    
    Ok(())
}
```

### Calculate Institutional Ownership Concentration

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let institutional = client.get_institutional_holders("AAPL").await?;
    
    println!("Institutional Ownership Analysis for {}:\n", institutional.symbol);
    
    // Calculate total shares held by top 10
    let top_10_shares: i64 = institutional.holders.iter()
        .take(10)
        .map(|h| h.shares)
        .sum();
    
    // Calculate total shares held by all reported institutions
    let total_shares: i64 = institutional.holders.iter()
        .map(|h| h.shares)
        .sum();
    
    println!("Total institutions: {}", institutional.holders.len());
    println!("Total shares held: {}", total_shares);
    println!("Top 10 holdings: {}", top_10_shares);
    
    if total_shares > 0 {
        let concentration = (top_10_shares as f64 / total_shares as f64) * 100.0;
        println!("Top 10 concentration: {:.2}%", concentration);
        
        if concentration > 50.0 {
            println!("\n⚠️  High concentration in top 10 holders");
        }
    }
    
    Ok(())
}
```

### Compare Insider vs Institutional Ownership

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let major = client.get_major_holders("AAPL").await?;
    
    println!("Ownership Structure for {}:\n", major.symbol);
    
    // Extract percentages from breakdown
    let insider_pct = major.breakdown.breakdown_data
        .get("insidersPercentHeld")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    let institutional_pct = major.breakdown.breakdown_data
        .get("institutionsPercentHeld")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    println!("Insider ownership: {:.2}%", insider_pct * 100.0);
    println!("Institutional ownership: {:.2}%", institutional_pct * 100.0);
    
    let retail_pct = 1.0 - insider_pct - institutional_pct;
    println!("Retail/Other: {:.2}%", retail_pct * 100.0);
    
    // Analyze ownership structure
    println!("\nAnalysis:");
    
    if insider_pct > 0.20 {
        println!("✓ High insider ownership (>20%) - strong alignment");
    } else if insider_pct < 0.01 {
        println!("⚠️  Very low insider ownership (<1%)");
    }
    
    if institutional_pct > 0.80 {
        println!("✓ Highly institutional (>80%)");
    } else if institutional_pct < 0.30 {
        println!("⚠️  Low institutional ownership (<30%)");
    }
    
    Ok(())
}
```

### Track Insider Transaction Patterns

```rust
use finance_query_core::YahooClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transactions = client.get_insider_transactions("TSLA").await?;
    
    println!("Insider Transaction Patterns for {}:\n", transactions.symbol);
    
    // Count transactions by type
    let mut txn_counts: HashMap<String, usize> = HashMap::new();
    let mut total_buy_shares = 0i64;
    let mut total_sell_shares = 0i64;
    
    for txn in &transactions.transactions {
        *txn_counts.entry(txn.transaction.clone()).or_insert(0) += 1;
        
        if let Some(shares) = txn.shares {
            if txn.transaction.contains("Sale") || txn.transaction.contains("Sell") {
                total_sell_shares += shares;
            } else if txn.transaction.contains("Purchase") || txn.transaction.contains("Buy") {
                total_buy_shares += shares;
            }
        }
    }
    
    println!("Transaction Types:");
    for (txn_type, count) in &txn_counts {
        println!("  {}: {}", txn_type, count);
    }
    
    println!("\nShare Activity:");
    println!("  Total bought: {} shares", total_buy_shares);
    println!("  Total sold: {} shares", total_sell_shares);
    println!("  Net: {} shares", total_buy_shares - total_sell_shares);
    
    Ok(())
}
```

### Find Recent Insider Buying

```rust
use finance_query_core::YahooClient;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transactions = client.get_insider_transactions("AAPL").await?;
    
    let thirty_days_ago = Utc::now() - Duration::days(30);
    
    println!("Recent Insider Purchases (Last 30 Days) for {}:\n", transactions.symbol);
    
    let recent_buys: Vec<_> = transactions.transactions.iter()
        .filter(|txn| {
            txn.start_date > thirty_days_ago &&
            (txn.transaction.contains("Purchase") || txn.transaction.contains("Buy"))
        })
        .collect();
    
    if recent_buys.is_empty() {
        println!("No insider purchases in the last 30 days");
    } else {
        for txn in recent_buys {
            println!("{} ({})", txn.insider, txn.position);
            println!("  Date: {}", txn.start_date.format("%Y-%m-%d"));
            
            if let Some(shares) = txn.shares {
                println!("  Shares: {}", shares);
            }
            
            if let Some(value) = txn.value {
                println!("  Value: ${:.2}M", value as f64 / 1_000_000.0);
            }
            
            println!();
        }
        
        println!("Total recent purchases: {}", recent_buys.len());
    }
    
    Ok(())
}
```


## JSON Response Formats

### Major Holders Response

```json
{
  "symbol": "AAPL",
  "breakdown": {
    "breakdownData": {
      "insidersPercentHeld": 0.0007,
      "institutionsPercentHeld": 0.6234,
      "institutionsFloatPercentHeld": 0.6245,
      "institutionsCount": 5234
    }
  }
}
```

### Institutional Holders Response

```json
{
  "symbol": "MSFT",
  "holders": [
    {
      "holder": "Vanguard Group, Inc.",
      "shares": 789456123,
      "dateReported": "2024-09-30T00:00:00Z",
      "percentOut": 8.52,
      "value": 312456789000
    },
    {
      "holder": "BlackRock Inc.",
      "shares": 654321098,
      "dateReported": "2024-09-30T00:00:00Z",
      "percentOut": 7.05,
      "value": 258963147000
    },
    {
      "holder": "State Street Corporation",
      "shares": 456789012,
      "dateReported": "2024-09-30T00:00:00Z",
      "percentOut": 4.92,
      "value": 180741852000
    }
  ]
}
```

### Mutual Fund Holders Response

```json
{
  "symbol": "GOOGL",
  "holders": [
    {
      "holder": "Vanguard Total Stock Market Index Fund",
      "shares": 123456789,
      "dateReported": "2024-09-30T00:00:00Z",
      "percentOut": 1.95,
      "value": 18234567890
    },
    {
      "holder": "Fidelity 500 Index Fund",
      "shares": 98765432,
      "dateReported": "2024-09-30T00:00:00Z",
      "percentOut": 1.56,
      "value": 14567890123
    }
  ]
}
```

### Insider Transactions Response

```json
{
  "symbol": "TSLA",
  "transactions": [
    {
      "startDate": "2024-11-15T00:00:00Z",
      "insider": "Musk Elon",
      "position": "CEO",
      "transaction": "Sale",
      "shares": 5000000,
      "value": 1250000000,
      "ownership": "Direct"
    },
    {
      "startDate": "2024-11-10T00:00:00Z",
      "insider": "Kirkhorn Zachary",
      "position": "CFO",
      "transaction": "Stock Option Exercise",
      "shares": 50000,
      "ownership": "Direct"
    },
    {
      "startDate": "2024-11-05T00:00:00Z",
      "insider": "Guillen Jerome",
      "position": "President",
      "transaction": "Purchase",
      "shares": 10000,
      "value": 2500000,
      "ownership": "Direct"
    }
  ]
}
```

### Insider Purchases Summary Response

```json
{
  "symbol": "NVDA",
  "summary": {
    "period": "Last 6 Months",
    "purchasesShares": 125000,
    "purchasesTransactions": 15,
    "salesShares": 2500000,
    "salesTransactions": 45,
    "netShares": -2375000,
    "netTransactions": -30,
    "totalInsiderShares": 15000000,
    "netPercentInsiderShares": -15.83,
    "buyPercentInsiderShares": 0.83,
    "sellPercentInsiderShares": 16.67
  }
}
```

### Insider Roster Response

```json
{
  "symbol": "META",
  "roster": [
    {
      "name": "Zuckerberg Mark",
      "position": "CEO & Chairman",
      "mostRecentTransaction": "Stock Gift",
      "latestTransactionDate": "2024-11-01T00:00:00Z",
      "sharesOwnedDirectly": 350000000,
      "sharesOwnedIndirectly": 50000000,
      "positionDirectDate": "2024-09-30T00:00:00Z"
    },
    {
      "name": "Sandberg Sheryl",
      "position": "COO",
      "mostRecentTransaction": "Sale",
      "latestTransactionDate": "2024-10-15T00:00:00Z",
      "sharesOwnedDirectly": 1500000,
      "sharesOwnedIndirectly": 500000,
      "positionDirectDate": "2024-09-30T00:00:00Z"
    }
  ]
}
```

## Field Details

### Ownership Percentages

- `insidersPercentHeld`: Decimal format (0.0007 = 0.07%)
- `institutionsPercentHeld`: Decimal format (0.6234 = 62.34%)
- `percent_out`: Percentage format (8.52 = 8.52%)
- All percentages represent portion of outstanding shares

### Share Counts

- All share counts are absolute numbers
- Not in thousands or millions
- Example: 789456123 = 789.5 million shares

### Values

- All monetary values in dollars
- Not in thousands or millions
- Example: 312456789000 = $312.5 billion

### Dates

- All dates in ISO 8601 format with UTC timezone
- Format: `YYYY-MM-DDTHH:MM:SSZ`
- `dateReported`: When the holding was reported (typically quarter-end)
- `startDate`: When the transaction occurred

### Transaction Types

Common transaction types:
- `"Sale"`: Insider sold shares
- `"Purchase"`: Insider bought shares
- `"Stock Option Exercise"`: Exercised stock options
- `"Stock Gift"`: Gifted shares
- `"Automatic Sale"`: Pre-planned automatic sale
- `"Conversion of Units"`: Conversion of restricted units

### Ownership Types

- `"Direct"`: Shares owned directly by the insider
- `"Indirect"`: Shares owned through trusts, family members, etc.

## Common Use Cases

### 1. Monitor Institutional Interest

```rust
// Track changes in institutional ownership over time
async fn track_institutional_changes(
    symbol: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let holders = client.get_institutional_holders(symbol).await?;
    
    // Group by reporting date
    let mut by_date: HashMap<String, Vec<&InstitutionalHolder>> = HashMap::new();
    
    for holder in &holders.holders {
        let date = holder.date_reported.format("%Y-%m-%d").to_string();
        by_date.entry(date).or_insert_with(Vec::new).push(holder);
    }
    
    // Analyze trends
    for (date, holders) in by_date {
        let total_shares: i64 = holders.iter().map(|h| h.shares).sum();
        println!("{}: {} institutions, {} total shares", 
            date, holders.len(), total_shares);
    }
    
    Ok(())
}
```

### 2. Insider Sentiment Indicator

```rust
// Calculate insider sentiment score
fn calculate_insider_sentiment(summary: &InsiderPurchase) -> f64 {
    let buys = summary.purchases_shares.unwrap_or(0) as f64;
    let sells = summary.sales_shares.unwrap_or(0) as f64;
    
    if buys + sells == 0.0 {
        return 0.0;
    }
    
    // Score from -1 (all selling) to +1 (all buying)
    (buys - sells) / (buys + sells)
}
```

### 3. Find Concentrated Ownership

```rust
// Identify stocks with concentrated ownership
async fn check_ownership_concentration(
    symbol: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let major = client.get_major_holders(symbol).await?;
    
    let insider_pct = major.breakdown.breakdown_data
        .get("insidersPercentHeld")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    let institutional_pct = major.breakdown.breakdown_data
        .get("institutionsPercentHeld")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    // Concentrated if top holders own >70%
    Ok(insider_pct + institutional_pct > 0.70)
}
```

## Important Notes

### Data Timeliness

- Institutional holdings reported quarterly (13F filings)
- Typically 45 days after quarter end
- Insider transactions reported within 2 business days
- Major holders breakdown updated periodically

### Data Accuracy

- Based on SEC filings (13F, Form 4)
- Institutional data may be 1-2 months old
- Insider data is more current
- Holdings may have changed since reporting date

### Reporting Thresholds

- Institutions must report if managing >$100M
- Only holdings >10,000 shares or $200,000 reported
- Insiders must report all transactions
- Some holdings may be unreported

### Limitations

- Not all institutional holders may be listed
- Small holders may not appear
- International holders may not be included
- Retail ownership not directly tracked

## Best Practices

1. **Check Reporting Dates**: Always verify when data was reported
2. **Compare Periods**: Look at trends over multiple quarters
3. **Insider Context**: Consider transaction type and timing
4. **Ownership Structure**: Analyze total ownership picture
5. **Cross-Reference**: Verify with SEC filings for critical decisions
6. **Volume Context**: Compare insider trades to average volume
7. **Position Context**: Consider insider's role and compensation
8. **Pattern Recognition**: Look for patterns in insider activity
9. **Concentration Risk**: Monitor ownership concentration
10. **Sentiment Analysis**: Use insider activity as one of many signals

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_institutional_holders("AAPL").await {
        Ok(holders) => {
            if holders.holders.is_empty() {
                println!("No institutional holders data available");
            } else {
                println!("Found {} institutional holders", holders.holders.len());
            }
        }
        Err(YahooError::NotFound) => {
            println!("Symbol not found");
        }
        Err(YahooError::ParseError(msg)) => {
            println!("Failed to parse holders data: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## Related APIs

- **Fundamentals API**: Get shares outstanding for ownership calculations
- **Quote API**: Get current price for position values
- **News API**: Get news about insider transactions
- **SEC Filings API**: Access original SEC filings

## Regulatory Context

### 13F Filings

- Required for institutional investment managers
- Filed quarterly within 45 days of quarter end
- Must report holdings >$100M in assets
- Includes long positions in US equities

### Form 4 Filings

- Required for corporate insiders
- Filed within 2 business days of transaction
- Includes officers, directors, and 10%+ owners
- Reports all transactions in company stock

### Schedule 13D/13G

- Required for 5%+ beneficial owners
- 13D: Filed within 10 days of crossing 5%
- 13G: Passive investors, filed annually
- Indicates significant ownership stakes

## Performance Tips

- Institutional holders data can be large (1000+ holders)
- Cache data locally as it updates infrequently
- Filter to top N holders for most use cases
- Insider transactions can be numerous for active companies
- Consider pagination or limiting results