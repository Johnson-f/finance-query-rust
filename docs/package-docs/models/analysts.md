# Analyst Data API

The Analyst API provides comprehensive access to Wall Street analyst data including recommendations, price targets, upgrades/downgrades, earnings estimates, and growth projections.

## Overview

This module provides access to multiple types of analyst data:

- **Recommendations**: Analyst buy/sell/hold ratings over time
- **Upgrades/Downgrades**: Rating changes from analyst firms
- **Price Targets**: Analyst price target statistics
- **Earnings Estimates**: Forward earnings per share (EPS) estimates
- **Revenue Estimates**: Forward revenue projections
- **Earnings History**: Historical earnings vs estimates
- **EPS Trends**: How earnings estimates have changed over time
- **EPS Revisions**: Count of upward/downward estimate revisions
- **Growth Estimates**: Growth projections vs industry/sector benchmarks

## Analysis Types

```rust
pub enum AnalysisType {
    Recommendations,
    UpgradesDowngrades,
    PriceTargets,
    EarningsEstimate,
    RevenueEstimate,
    EarningsHistory,
}
```

## Data Structures

### 1. Recommendations

Analyst rating distribution over time periods.

```rust
pub struct RecommendationData {
    pub period: String,
    pub strong_buy: Option<i32>,
    pub buy: Option<i32>,
    pub hold: Option<i32>,
    pub sell: Option<i32>,
    pub strong_sell: Option<i32>,
}
```

**Fields:**
- `period`: Time period (e.g., "2024-01", "2024-02")
- `strong_buy`: Number of strong buy ratings
- `buy`: Number of buy ratings
- `hold`: Number of hold ratings
- `sell`: Number of sell ratings
- `strong_sell`: Number of strong sell ratings

### 2. Upgrades/Downgrades

Individual analyst rating changes.

```rust
pub struct UpgradeDowngrade {
    pub firm: String,
    pub to_grade: Option<String>,
    pub from_grade: Option<String>,
    pub action: Option<String>,
    pub date: Option<DateTime<Utc>>,
}
```

**Fields:**
- `firm`: Analyst firm name (e.g., "Morgan Stanley", "Goldman Sachs")
- `to_grade`: New rating (e.g., "Buy", "Outperform")
- `from_grade`: Previous rating
- `action`: Action type (e.g., "upgrade", "downgrade", "init", "main")
- `date`: Date of the rating change

### 3. Price Targets

Analyst price target statistics.

```rust
pub struct PriceTarget {
    pub current: Option<f64>,
    pub mean: Option<f64>,
    pub median: Option<f64>,
    pub low: Option<f64>,
    pub high: Option<f64>,
}
```

**Fields:**
- `current`: Current stock price
- `mean`: Average analyst price target
- `median`: Median analyst price target
- `low`: Lowest price target
- `high`: Highest price target

### 4. Earnings History

Historical earnings results vs analyst estimates.

```rust
pub struct EarningsHistoryItem {
    pub date: DateTime<Utc>,
    pub eps_actual: Option<f64>,
    pub eps_estimate: Option<f64>,
    pub surprise: Option<f64>,
    pub surprise_percent: Option<f64>,
}
```

**Fields:**
- `date`: Earnings report date
- `eps_actual`: Actual reported EPS
- `eps_estimate`: Analyst consensus estimate
- `surprise`: Difference (actual - estimate)
- `surprise_percent`: Surprise as percentage

### 5. EPS Trends

How earnings estimates have evolved over time.

```rust
pub struct EpsTrend {
    pub period: String,
    pub current: Option<f64>,
    pub days_7_ago: Option<f64>,
    pub days_30_ago: Option<f64>,
    pub days_60_ago: Option<f64>,
    pub days_90_ago: Option<f64>,
}
```

**Fields:**
- `period`: Forecast period (e.g., "0q", "+1q", "0y")
- `current`: Current consensus estimate
- `days_7_ago`: Estimate from 7 days ago
- `days_30_ago`: Estimate from 30 days ago
- `days_60_ago`: Estimate from 60 days ago
- `days_90_ago`: Estimate from 90 days ago

### 6. EPS Revisions

Count of analyst estimate revisions.

```rust
pub struct EpsRevisions {
    pub period: String,
    pub up_last_7_days: Option<i32>,
    pub up_last_30_days: Option<i32>,
    pub down_last_7_days: Option<i32>,
    pub down_last_30_days: Option<i32>,
}
```

**Fields:**
- `period`: Forecast period
- `up_last_7_days`: Upward revisions in last 7 days
- `up_last_30_days`: Upward revisions in last 30 days
- `down_last_7_days`: Downward revisions in last 7 days
- `down_last_30_days`: Downward revisions in last 30 days

### 7. Growth Estimates

Growth projections compared to benchmarks.

```rust
pub struct GrowthEstimate {
    pub period: String,
    pub stock: Option<f64>,
    pub industry: Option<f64>,
    pub sector: Option<f64>,
    pub index: Option<f64>,
}
```

**Fields:**
- `period`: Time period (e.g., "Current Qtr", "Next Year")
- `stock`: Stock's estimated growth rate
- `industry`: Industry average growth rate
- `sector`: Sector average growth rate
- `index`: Market index growth rate

## Usage Examples

### 1. Get Analyst Recommendations

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let recommendations = client.get_recommendations("AAPL").await?;
    
    println!("Symbol: {}", recommendations.symbol);
    
    for rec in &recommendations.recommendations {
        println!("\nPeriod: {}", rec.period);
        println!("  Strong Buy: {:?}", rec.strong_buy);
        println!("  Buy: {:?}", rec.buy);
        println!("  Hold: {:?}", rec.hold);
        println!("  Sell: {:?}", rec.sell);
        println!("  Strong Sell: {:?}", rec.strong_sell);
        
        // Calculate total analysts
        let total = rec.strong_buy.unwrap_or(0) 
            + rec.buy.unwrap_or(0)
            + rec.hold.unwrap_or(0)
            + rec.sell.unwrap_or(0)
            + rec.strong_sell.unwrap_or(0);
        println!("  Total Analysts: {}", total);
    }
    
    Ok(())
}
```

### 2. Track Upgrades and Downgrades

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let changes = client.get_upgrades_downgrades("TSLA").await?;
    
    println!("Recent analyst actions for {}:\n", changes.symbol);
    
    for change in &changes.upgrades_downgrades {
        println!("Firm: {}", change.firm);
        
        if let Some(action) = &change.action {
            println!("  Action: {}", action);
        }
        
        if let (Some(from), Some(to)) = (&change.from_grade, &change.to_grade) {
            println!("  {} → {}", from, to);
        } else if let Some(to) = &change.to_grade {
            println!("  New Rating: {}", to);
        }
        
        if let Some(date) = change.date {
            println!("  Date: {}", date.format("%Y-%m-%d"));
        }
        println!();
    }
    
    Ok(())
}
```

### 3. Analyze Price Targets

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let targets = client.get_price_targets("NVDA").await?;
    let pt = &targets.price_targets;
    
    println!("Price Target Analysis for {}:\n", targets.symbol);
    
    if let Some(current) = pt.current {
        println!("Current Price: ${:.2}", current);
    }
    
    if let Some(mean) = pt.mean {
        println!("Mean Target: ${:.2}", mean);
        
        if let Some(current) = pt.current {
            let upside = ((mean - current) / current) * 100.0;
            println!("Implied Upside: {:.2}%", upside);
        }
    }
    
    if let Some(median) = pt.median {
        println!("Median Target: ${:.2}", median);
    }
    
    if let (Some(low), Some(high)) = (pt.low, pt.high) {
        println!("Target Range: ${:.2} - ${:.2}", low, high);
        let range_width = ((high - low) / low) * 100.0;
        println!("Range Width: {:.2}%", range_width);
    }
    
    Ok(())
}
```

### 4. Review Earnings History

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_earnings_history("MSFT").await?;
    
    println!("Earnings History for {}:\n", history.symbol);
    
    let mut beat_count = 0;
    let mut miss_count = 0;
    
    for item in &history.earnings_history {
        println!("Date: {}", item.date.format("%Y-%m-%d"));
        
        if let (Some(actual), Some(estimate)) = (item.eps_actual, item.eps_estimate) {
            println!("  Actual: ${:.2}", actual);
            println!("  Estimate: ${:.2}", estimate);
            
            if let Some(surprise_pct) = item.surprise_percent {
                println!("  Surprise: {:.2}%", surprise_pct);
                
                if surprise_pct > 0.0 {
                    beat_count += 1;
                    println!("  Result: BEAT ✓");
                } else if surprise_pct < 0.0 {
                    miss_count += 1;
                    println!("  Result: MISS ✗");
                } else {
                    println!("  Result: IN-LINE");
                }
            }
        }
        println!();
    }
    
    println!("Summary:");
    println!("  Beats: {}", beat_count);
    println!("  Misses: {}", miss_count);
    
    Ok(())
}
```

### 5. Monitor EPS Trends

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let trends = client.get_eps_trends("GOOGL").await?;
    
    println!("EPS Estimate Trends for {}:\n", trends.symbol);
    
    for trend in &trends.eps_trend {
        println!("Period: {}", trend.period);
        
        if let Some(current) = trend.current {
            println!("  Current Estimate: ${:.2}", current);
            
            // Check if estimates are improving
            if let Some(ago_30) = trend.days_30_ago {
                let change = current - ago_30;
                let change_pct = (change / ago_30) * 100.0;
                
                println!("  30-Day Change: ${:.2} ({:.2}%)", change, change_pct);
                
                if change > 0.0 {
                    println!("  Trend: IMPROVING ↑");
                } else if change < 0.0 {
                    println!("  Trend: DECLINING ↓");
                } else {
                    println!("  Trend: STABLE →");
                }
            }
            
            if let Some(ago_90) = trend.days_90_ago {
                let change = current - ago_90;
                let change_pct = (change / ago_90) * 100.0;
                println!("  90-Day Change: ${:.2} ({:.2}%)", change, change_pct);
            }
        }
        println!();
    }
    
    Ok(())
}
```

### 6. Analyze EPS Revisions

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let revisions = client.get_eps_revisions("META").await?;
    
    println!("EPS Revisions for {}:\n", revisions.symbol);
    
    for rev in &revisions.eps_revisions {
        println!("Period: {}", rev.period);
        
        let up_7 = rev.up_last_7_days.unwrap_or(0);
        let down_7 = rev.down_last_7_days.unwrap_or(0);
        let up_30 = rev.up_last_30_days.unwrap_or(0);
        let down_30 = rev.down_last_30_days.unwrap_or(0);
        
        println!("  Last 7 Days:");
        println!("    Up: {}, Down: {}", up_7, down_7);
        
        println!("  Last 30 Days:");
        println!("    Up: {}, Down: {}", up_30, down_30);
        
        // Calculate sentiment
        let net_7 = up_7 - down_7;
        let net_30 = up_30 - down_30;
        
        println!("  Net Sentiment (7d): {}", net_7);
        println!("  Net Sentiment (30d): {}", net_30);
        
        if net_30 > 0 {
            println!("  Overall: POSITIVE ✓");
        } else if net_30 < 0 {
            println!("  Overall: NEGATIVE ✗");
        } else {
            println!("  Overall: NEUTRAL");
        }
        println!();
    }
    
    Ok(())
}
```

### 7. Compare Growth Estimates

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let growth = client.get_growth_estimates("AMZN").await?;
    
    println!("Growth Estimates for {}:\n", growth.symbol);
    
    for estimate in &growth.growth_estimates {
        println!("Period: {}", estimate.period);
        
        if let Some(stock) = estimate.stock {
            println!("  Stock: {:.2}%", stock);
            
            if let Some(industry) = estimate.industry {
                println!("  Industry: {:.2}%", industry);
                let vs_industry = stock - industry;
                println!("  vs Industry: {:+.2}%", vs_industry);
            }
            
            if let Some(sector) = estimate.sector {
                println!("  Sector: {:.2}%", sector);
                let vs_sector = stock - sector;
                println!("  vs Sector: {:+.2}%", vs_sector);
            }
            
            if let Some(index) = estimate.index {
                println!("  S&P 500: {:.2}%", index);
                let vs_index = stock - index;
                println!("  vs S&P 500: {:+.2}%", vs_index);
            }
        }
        println!();
    }
    
    Ok(())
}
```

### 8. Calculate Recommendation Score

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let recommendations = client.get_recommendations("AAPL").await?;
    
    if let Some(latest) = recommendations.recommendations.last() {
        // Calculate weighted score (1=Strong Buy, 5=Strong Sell)
        let strong_buy = latest.strong_buy.unwrap_or(0) as f64;
        let buy = latest.buy.unwrap_or(0) as f64;
        let hold = latest.hold.unwrap_or(0) as f64;
        let sell = latest.sell.unwrap_or(0) as f64;
        let strong_sell = latest.strong_sell.unwrap_or(0) as f64;
        
        let total = strong_buy + buy + hold + sell + strong_sell;
        
        if total > 0.0 {
            let score = (strong_buy * 1.0 + buy * 2.0 + hold * 3.0 
                        + sell * 4.0 + strong_sell * 5.0) / total;
            
            println!("Analyst Consensus Score: {:.2}", score);
            println!("(1.0 = Strong Buy, 5.0 = Strong Sell)");
            
            let rating = match score {
                s if s < 1.5 => "Strong Buy",
                s if s < 2.5 => "Buy",
                s if s < 3.5 => "Hold",
                s if s < 4.5 => "Sell",
                _ => "Strong Sell",
            };
            
            println!("Consensus Rating: {}", rating);
        }
    }
    
    Ok(())
}
```

## JSON Response Formats

### Recommendations Response

```json
{
  "symbol": "AAPL",
  "recommendations": [
    {
      "period": "2024-01",
      "strongBuy": 15,
      "buy": 20,
      "hold": 8,
      "sell": 2,
      "strongSell": 0
    },
    {
      "period": "2024-02",
      "strongBuy": 16,
      "buy": 19,
      "hold": 9,
      "sell": 1,
      "strongSell": 0
    }
  ]
}
```

### Upgrades/Downgrades Response

```json
{
  "symbol": "TSLA",
  "upgradesDowngrades": [
    {
      "firm": "Morgan Stanley",
      "toGrade": "Overweight",
      "fromGrade": "Equal-Weight",
      "action": "upgrade",
      "date": "2024-11-15T00:00:00Z"
    },
    {
      "firm": "Goldman Sachs",
      "toGrade": "Buy",
      "action": "init",
      "date": "2024-11-10T00:00:00Z"
    },
    {
      "firm": "JP Morgan",
      "toGrade": "Neutral",
      "fromGrade": "Overweight",
      "action": "downgrade",
      "date": "2024-11-05T00:00:00Z"
    }
  ]
}
```

### Price Targets Response

```json
{
  "symbol": "NVDA",
  "priceTargets": {
    "current": 495.50,
    "mean": 650.25,
    "median": 640.00,
    "low": 450.00,
    "high": 900.00
  }
}
```

### Earnings History Response

```json
{
  "symbol": "MSFT",
  "earningsHistory": [
    {
      "date": "2024-10-30T00:00:00Z",
      "epsActual": 3.30,
      "epsEstimate": 3.10,
      "surprise": 0.20,
      "surprisePercent": 6.45
    },
    {
      "date": "2024-07-30T00:00:00Z",
      "epsActual": 2.95,
      "epsEstimate": 2.93,
      "surprise": 0.02,
      "surprisePercent": 0.68
    }
  ]
}
```

### EPS Trends Response

```json
{
  "symbol": "GOOGL",
  "epsTrend": [
    {
      "period": "0q",
      "current": 1.85,
      "7daysAgo": 1.84,
      "30daysAgo": 1.82,
      "60daysAgo": 1.80,
      "90daysAgo": 1.78
    },
    {
      "period": "+1q",
      "current": 2.10,
      "7daysAgo": 2.09,
      "30daysAgo": 2.05,
      "60daysAgo": 2.00,
      "90daysAgo": 1.95
    }
  ]
}
```

### EPS Revisions Response

```json
{
  "symbol": "META",
  "epsRevisions": [
    {
      "period": "0q",
      "upLast7days": 3,
      "upLast30days": 8,
      "downLast7days": 0,
      "downLast30days": 2
    },
    {
      "period": "+1q",
      "upLast7days": 2,
      "upLast30days": 5,
      "downLast7days": 1,
      "downLast30days": 3
    }
  ]
}
```

### Growth Estimates Response

```json
{
  "symbol": "AMZN",
  "growthEstimates": [
    {
      "period": "Current Qtr",
      "stock": 12.5,
      "industry": 8.3,
      "sector": 9.1,
      "index": 7.5
    },
    {
      "period": "Next Year",
      "stock": 15.2,
      "industry": 10.5,
      "sector": 11.0,
      "index": 9.8
    }
  ]
}
```

### Earnings Estimate Response

```json
{
  "symbol": "AAPL",
  "earningsEstimate": {
    "estimates": {
      "0q": {
        "avg": 1.54,
        "low": 1.48,
        "high": 1.62,
        "numberOfAnalysts": 28
      },
      "+1q": {
        "avg": 2.10,
        "low": 1.95,
        "high": 2.25,
        "numberOfAnalysts": 25
      }
    }
  }
}
```

### Revenue Estimate Response

```json
{
  "symbol": "AAPL",
  "revenueEstimate": {
    "estimates": {
      "0q": {
        "avg": 123500000000,
        "low": 119000000000,
        "high": 128000000000,
        "numberOfAnalysts": 30
      }
    }
  }
}
```

## Common Patterns

### Sentiment Analysis

```rust
// Calculate overall analyst sentiment
fn calculate_sentiment(recommendations: &RecommendationData) -> String {
    let bullish = recommendations.strong_buy.unwrap_or(0) 
                + recommendations.buy.unwrap_or(0);
    let bearish = recommendations.sell.unwrap_or(0) 
                + recommendations.strong_sell.unwrap_or(0);
    let neutral = recommendations.hold.unwrap_or(0);
    
    let total = bullish + bearish + neutral;
    if total == 0 {
        return "Unknown".to_string();
    }
    
    let bullish_pct = (bullish as f64 / total as f64) * 100.0;
    
    match bullish_pct {
        p if p >= 70.0 => "Very Bullish".to_string(),
        p if p >= 55.0 => "Bullish".to_string(),
        p if p >= 45.0 => "Neutral".to_string(),
        p if p >= 30.0 => "Bearish".to_string(),
        _ => "Very Bearish".to_string(),
    }
}
```

### Earnings Beat Rate

```rust
// Calculate percentage of earnings beats
fn calculate_beat_rate(history: &[EarningsHistoryItem]) -> f64 {
    let beats = history.iter()
        .filter(|item| {
            item.surprise_percent.map_or(false, |s| s > 0.0)
        })
        .count();
    
    if history.is_empty() {
        0.0
    } else {
        (beats as f64 / history.len() as f64) * 100.0
    }
}
```

### Price Target Upside

```rust
// Calculate implied upside from price targets
fn calculate_upside(targets: &PriceTarget) -> Option<f64> {
    match (targets.current, targets.mean) {
        (Some(current), Some(mean)) => {
            Some(((mean - current) / current) * 100.0)
        }
        _ => None,
    }
}
```

## Period Codes

Common period codes used in estimates:

- `"0q"`: Current quarter
- `"+1q"`: Next quarter
- `"+2q"`: Quarter after next
- `"0y"`: Current year
- `"+1y"`: Next year
- `"+5y"`: Next 5 years (annualized)

## Action Types

Common upgrade/downgrade actions:

- `"upgrade"`: Rating improved
- `"downgrade"`: Rating lowered
- `"init"`: Initial coverage started
- `"main"`: Rating maintained/reiterated
- `"reit"`: Rating reiterated

## Rating Grades

Common analyst rating grades:

- Buy ratings: "Strong Buy", "Buy", "Overweight", "Outperform"
- Neutral ratings: "Hold", "Neutral", "Equal-Weight", "Market Perform"
- Sell ratings: "Sell", "Strong Sell", "Underweight", "Underperform"

## Notes

- Analyst data is aggregated from multiple sources
- Recommendation counts represent number of analysts with each rating
- Price targets are typically 12-month forward projections
- EPS estimates are consensus (average) of all analyst estimates
- Growth rates are typically expressed as percentages
- Historical data availability varies by symbol
- Some fields may be `None` if data is unavailable
- Dates are in UTC timezone

## Related APIs

- **Calendar API**: Get upcoming earnings dates
- **Fundamentals API**: Get actual financial results
- **Quote API**: Get current stock price for comparison
- **News API**: Get news that may affect analyst ratings

