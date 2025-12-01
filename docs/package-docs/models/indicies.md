# Market Indices API

The Market Indices API provides access to global stock market indices including current values, changes, and historical returns across multiple time periods.

## Overview

This module provides access to:

- **70+ Global Indices**: Major market indices from around the world
- **Real-time Data**: Current index values and daily changes
- **Historical Returns**: Performance over multiple time periods
- **Regional Organization**: Indices grouped by geographic region
- **Currency Indices**: Major currency indices and volatility measures

## Data Structures

### MarketIndex

Individual market index data with current value and returns.

```rust
pub struct MarketIndex {
    pub name: String,
    pub value: f64,
    pub change: String,
    pub percent_change: String,
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
}
```

**Fields:**
- `name`: Index name (e.g., "S&P 500", "NASDAQ Composite")
- `value`: Current index value
- `change`: Point change (e.g., "+45.23", "-12.45")
- `percent_change`: Percentage change (e.g., "+1.23%", "-0.45%")
- `five_days_return`: 5-day return percentage
- `one_month_return`: 1-month return percentage
- `three_month_return`: 3-month return percentage
- `six_month_return`: 6-month return percentage
- `ytd_return`: Year-to-date return percentage
- `year_return`: 1-year return percentage
- `three_year_return`: 3-year return percentage
- `five_year_return`: 5-year return percentage
- `ten_year_return`: 10-year return percentage
- `max_return`: All-time return percentage


### Index

Enum representing available market indices.

```rust
pub enum Index {
    // United States
    Gspc,        // S&P 500
    Dji,         // Dow Jones Industrial Average
    Ixic,        // NASDAQ Composite
    Nya,         // NYSE Composite
    Xax,         // NYSE American Composite
    Rut,         // Russell 2000
    Vix,         // CBOE Volatility Index
    
    // Europe
    Ftse,        // FTSE 100 (UK)
    Gdaxi,       // DAX (Germany)
    Fchi,        // CAC 40 (France)
    Stoxx50e,    // Euro Stoxx 50
    
    // Asia
    Hsi,         // Hang Seng (Hong Kong)
    N225,        // Nikkei 225 (Japan)
    Shanghai,    // Shanghai Composite (China)
    Bsesn,       // BSE Sensex (India)
    
    // And 50+ more indices...
}
```

### Region

Geographic regions for index classification.

```rust
pub enum Region {
    UnitedStates,    // "US"
    NorthAmerica,    // "NA"
    SouthAmerica,    // "SA"
    Europe,          // "EU"
    Asia,            // "AS"
    Africa,          // "AF"
    MiddleEast,      // "ME"
    Oceania,         // "OCE"
    Global,          // "global"
}
```

## Available Indices

### United States
- **S&P 500** (`snp`) - 500 largest US companies
- **Dow Jones** (`djia`) - 30 blue-chip US companies
- **NASDAQ** (`nasdaq`) - Technology-heavy composite
- **Russell 2000** (`rut`) - Small-cap index
- **VIX** (`vix`) - Volatility index

### Europe
- **FTSE 100** (`ftse-100`) - UK's top 100 companies
- **DAX** (`dax`) - Germany's top 40 companies
- **CAC 40** (`cac-40`) - France's top 40 companies
- **Euro Stoxx 50** (`euro-stoxx-50`) - Eurozone blue chips
- **FTSE MIB** (`ftse-mib`) - Italy's main index

### Asia
- **Nikkei 225** (`nikkei-225`) - Japan's premier index
- **Hang Seng** (`hang-seng`) - Hong Kong's main index
- **Shanghai Composite** (`shanghai`) - China A-shares
- **Sensex** (`sensex`) - India's BSE index
- **Nifty 50** (`nifty-50`) - India's NSE index
- **KOSPI** (`kospi`) - South Korea's main index

### South America
- **Ibovespa** (`ibovespa`) - Brazil's main index
- **IPC Mexico** (`ipc-mexico`) - Mexican index
- **IPSA** (`ipsa`) - Chile's main index
- **Merval** (`merval`) - Argentina's index

### Oceania
- **ASX 200** (`asx-200`) - Australia's top 200
- **NZX 50** (`nzx-50`) - New Zealand's top 50

### Africa
- **EGX 30** (`egx-30`) - Egypt's main index
- **JSE Top 40** (`jse-40`) - South Africa's top 40

### Middle East
- **Tel Aviv 125** (`ta-125`) - Israel's main index
- **Tadawul** (`tadawul-all-share`) - Saudi Arabia
- **BIST 100** (`bist-100`) - Turkey's main index

### Global/Currency
- **US Dollar Index** (`usd`) - Dollar strength
- **MSCI World** (`msci-world`) - Global equity index

## Usage Examples

### Get Single Index

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let sp500 = client.get_market_index("snp").await?;
    
    println!("{}", sp500.name);
    println!("Value: {:.2}", sp500.value);
    println!("Change: {} ({})", sp500.change, sp500.percent_change);
    
    if let Some(ytd) = &sp500.ytd_return {
        println!("YTD Return: {}", ytd);
    }
    
    if let Some(year) = &sp500.year_return {
        println!("1-Year Return: {}", year);
    }
    
    Ok(())
}
```

### Get Multiple Indices

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let indices = vec!["snp", "djia", "nasdaq", "rut"];
    
    println!("US Market Overview:\n");
    
    for index_id in indices {
        match client.get_market_index(index_id).await {
            Ok(index) => {
                println!("{}: {:.2} ({})", 
                    index.name, 
                    index.value, 
                    index.percent_change
                );
            }
            Err(e) => {
                println!("{}: Error - {}", index_id, e);
            }
        }
    }
    
    Ok(())
}
```

### Compare Global Markets

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let global_indices = vec![
        ("snp", "US"),
        ("ftse-100", "UK"),
        ("dax", "Germany"),
        ("nikkei-225", "Japan"),
        ("hang-seng", "Hong Kong"),
        ("sensex", "India"),
    ];
    
    println!("Global Market Performance:\n");
    
    for (index_id, country) in global_indices {
        if let Ok(index) = client.get_market_index(index_id).await {
            println!("{} ({})", country, index.name);
            println!("  Current: {:.2}", index.value);
            println!("  Change: {}", index.percent_change);
            
            if let Some(ytd) = &index.ytd_return {
                println!("  YTD: {}", ytd);
            }
            
            println!();
        }
    }
    
    Ok(())
}
```

### Track Index Performance

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let index = client.get_market_index("snp").await?;
    
    println!("Performance Analysis: {}\n", index.name);
    println!("Current Value: {:.2}", index.value);
    println!("Today: {}", index.percent_change);
    
    println!("\nHistorical Returns:");
    
    if let Some(five_day) = &index.five_days_return {
        println!("  5 Days: {}", five_day);
    }
    
    if let Some(one_month) = &index.one_month_return {
        println!("  1 Month: {}", one_month);
    }
    
    if let Some(three_month) = &index.three_month_return {
        println!("  3 Months: {}", three_month);
    }
    
    if let Some(ytd) = &index.ytd_return {
        println!("  YTD: {}", ytd);
    }
    
    if let Some(one_year) = &index.year_return {
        println!("  1 Year: {}", one_year);
    }
    
    if let Some(three_year) = &index.three_year_return {
        println!("  3 Years: {}", three_year);
    }
    
    if let Some(five_year) = &index.five_year_return {
        println!("  5 Years: {}", five_year);
    }
    
    Ok(())
}
```

### Market Sentiment Dashboard

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    println!("═══════════════════════════════════════");
    println!("       Market Sentiment Dashboard");
    println!("═══════════════════════════════════════\n");
    
    // Major US indices
    let us_indices = vec!["snp", "djia", "nasdaq"];
    let mut positive = 0;
    let mut negative = 0;
    
    for index_id in &us_indices {
        if let Ok(index) = client.get_market_index(index_id).await {
            let is_positive = index.percent_change.starts_with('+');
            
            if is_positive {
                positive += 1;
                print!("✓ ");
            } else {
                negative += 1;
                print!("✗ ");
            }
            
            println!("{}: {}", index.name, index.percent_change);
        }
    }
    
    println!("\nMarket Sentiment:");
    if positive > negative {
        println!("🟢 BULLISH ({} up, {} down)", positive, negative);
    } else if negative > positive {
        println!("🔴 BEARISH ({} up, {} down)", positive, negative);
    } else {
        println!("🟡 MIXED ({} up, {} down)", positive, negative);
    }
    
    // Check VIX
    if let Ok(vix) = client.get_market_index("vix").await {
        println!("\nVolatility (VIX): {:.2}", vix.value);
        
        if vix.value > 30.0 {
            println!("⚠️  HIGH VOLATILITY");
        } else if vix.value > 20.0 {
            println!("⚠️  ELEVATED VOLATILITY");
        } else {
            println!("✓ NORMAL VOLATILITY");
        }
    }
    
    println!("\n═══════════════════════════════════════");
    
    Ok(())
}
```

### Regional Market Comparison

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let regions = vec![
        ("Americas", vec!["snp", "tsx-composite", "ibovespa"]),
        ("Europe", vec!["ftse-100", "dax", "cac-40"]),
        ("Asia", vec!["nikkei-225", "hang-seng", "sensex"]),
    ];
    
    println!("Regional Market Performance:\n");
    
    for (region_name, indices) in regions {
        println!("{}:", region_name);
        
        for index_id in indices {
            if let Ok(index) = client.get_market_index(index_id).await {
                println!("  {}: {}", index.name, index.percent_change);
            }
        }
        
        println!();
    }
    
    Ok(())
}
```

### Find Best Performing Indices

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let indices = vec![
        "snp", "djia", "nasdaq", "ftse-100", "dax", 
        "nikkei-225", "hang-seng", "sensex"
    ];
    
    let mut performances = Vec::new();
    
    for index_id in indices {
        if let Ok(index) = client.get_market_index(index_id).await {
            // Parse percent change
            let pct_str = index.percent_change
                .trim_start_matches('+')
                .trim_end_matches('%');
            
            if let Ok(pct) = pct_str.parse::<f64>() {
                performances.push((index.name.clone(), pct));
            }
        }
    }
    
    // Sort by performance
    performances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("Today's Best Performers:\n");
    
    for (i, (name, pct)) in performances.iter().take(5).enumerate() {
        println!("{}. {}: {:+.2}%", i + 1, name, pct);
    }
    
    println!("\nToday's Worst Performers:\n");
    
    for (i, (name, pct)) in performances.iter().rev().take(5).enumerate() {
        println!("{}. {}: {:+.2}%", i + 1, name, pct);
    }
    
    Ok(())
}
```

### Monitor Volatility Indices

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    println!("Volatility Indices:\n");
    
    // US VIX
    if let Ok(vix) = client.get_market_index("vix").await {
        println!("US (VIX): {:.2}", vix.value);
        
        let level = if vix.value < 12.0 {
            "Very Low"
        } else if vix.value < 20.0 {
            "Low"
        } else if vix.value < 30.0 {
            "Elevated"
        } else if vix.value < 40.0 {
            "High"
        } else {
            "Extreme"
        };
        
        println!("  Level: {}", level);
        println!("  Change: {}", vix.percent_change);
    }
    
    // India VIX
    if let Ok(india_vix) = client.get_market_index("india-vix").await {
        println!("\nIndia (India VIX): {:.2}", india_vix.value);
        println!("  Change: {}", india_vix.percent_change);
    }
    
    Ok(())
}
```

### Currency Index Tracker

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    println!("Currency Indices:\n");
    
    let currencies = vec![
        ("usd", "US Dollar Index"),
        ("gbp", "British Pound"),
        ("euro", "Euro"),
        ("yen", "Japanese Yen"),
        ("australian", "Australian Dollar"),
    ];
    
    for (index_id, name) in currencies {
        if let Ok(index) = client.get_market_index(index_id).await {
            println!("{}: {:.2} ({})", 
                name, 
                index.value, 
                index.percent_change
            );
        }
    }
    
    Ok(())
}
```

### Calculate Index Correlation

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let sp500 = client.get_market_index("snp").await?;
    let nasdaq = client.get_market_index("nasdaq").await?;
    
    // Parse percent changes
    let sp_change = sp500.percent_change
        .trim_start_matches('+')
        .trim_end_matches('%')
        .parse::<f64>()?;
    
    let nq_change = nasdaq.percent_change
        .trim_start_matches('+')
        .trim_end_matches('%')
        .parse::<f64>()?;
    
    println!("Index Comparison:\n");
    println!("S&P 500: {:+.2}%", sp_change);
    println!("NASDAQ: {:+.2}%", nq_change);
    
    // Check if moving in same direction
    if (sp_change > 0.0 && nq_change > 0.0) || 
       (sp_change < 0.0 && nq_change < 0.0) {
        println!("\n✓ Indices moving in same direction");
    } else {
        println!("\n⚠️  Indices diverging");
    }
    
    // Compare magnitude
    if nq_change.abs() > sp_change.abs() * 1.5 {
        println!("NASDAQ showing higher volatility");
    }
    
    Ok(())
}
```


## JSON Response Format

### Complete Index Response

```json
{
  "name": "S&P 500",
  "value": 4567.89,
  "change": "+23.45",
  "percentChange": "+0.52%",
  "fiveDaysReturn": "+1.23%",
  "oneMonthReturn": "+3.45%",
  "threeMonthReturn": "+8.76%",
  "sixMonthReturn": "+12.34%",
  "ytdReturn": "+18.92%",
  "yearReturn": "+22.45%",
  "threeYearReturn": "+45.67%",
  "fiveYearReturn": "+78.90%",
  "tenYearReturn": "+156.78%",
  "maxReturn": "+345.67%"
}
```

### Minimal Response (Recent Index)

```json
{
  "name": "NASDAQ Composite",
  "value": 14234.56,
  "change": "-45.67",
  "percentChange": "-0.32%",
  "fiveDaysReturn": "-0.89%",
  "oneMonthReturn": "+2.34%",
  "ytdReturn": "+15.67%"
}
```

### Negative Performance

```json
{
  "name": "FTSE 100",
  "value": 7456.78,
  "change": "-34.21",
  "percentChange": "-0.46%",
  "fiveDaysReturn": "-1.23%",
  "oneMonthReturn": "-2.45%",
  "threeMonthReturn": "-3.67%",
  "ytdReturn": "+5.43%",
  "yearReturn": "+8.76%"
}
```

## Field Details

### Value Format

- `value`: Current index level as floating point number
- No thousands separators in JSON
- Example: `4567.89` (not `4,567.89`)

### Change Format

- `change`: Point change with sign
- Format: `"+23.45"` or `"-12.34"`
- Always includes sign (+ or -)
- String type to preserve formatting

### Percent Change Format

- `percent_change`: Percentage with sign and % symbol
- Format: `"+0.52%"` or `"-0.32%"`
- Always includes sign and % symbol
- String type to preserve formatting

### Return Periods

All return fields are optional and formatted as percentages:
- Include sign (+ or -)
- Include % symbol
- String type
- May be `null` if data unavailable

### Data Availability

- Current value and daily change: Always available
- Short-term returns (5d, 1m, 3m): Usually available
- Long-term returns (3y, 5y, 10y): May be unavailable for newer indices
- `maxReturn`: May be unavailable for some indices

## Index Identifiers

### US Indices
- `snp` - S&P 500
- `djia` - Dow Jones Industrial Average
- `nasdaq` - NASDAQ Composite
- `nyse-composite` - NYSE Composite
- `nyse-amex` - NYSE American
- `rut` - Russell 2000
- `vix` - CBOE Volatility Index

### European Indices
- `ftse-100` - FTSE 100 (UK)
- `dax` - DAX (Germany)
- `cac-40` - CAC 40 (France)
- `euro-stoxx-50` - Euro Stoxx 50
- `euronext-100` - Euronext 100
- `ftse-mib` - FTSE MIB (Italy)
- `ibex-35` - IBEX 35 (Spain)
- `smi` - Swiss Market Index
- `aex` - AEX (Netherlands)

### Asian Indices
- `nikkei-225` - Nikkei 225 (Japan)
- `hang-seng` - Hang Seng (Hong Kong)
- `shanghai` - Shanghai Composite (China)
- `sensex` - BSE Sensex (India)
- `nifty-50` - Nifty 50 (India)
- `kospi` - KOSPI (South Korea)
- `twse` - Taiwan TAIEX
- `sti` - Straits Times (Singapore)

### Other Regions
- `ibovespa` - Ibovespa (Brazil)
- `ipc-mexico` - IPC (Mexico)
- `tsx-composite` - TSX Composite (Canada)
- `asx-200` - ASX 200 (Australia)
- `nzx-50` - NZX 50 (New Zealand)

### Special Indices
- `vix` - US Volatility Index
- `india-vix` - India Volatility Index
- `usd` - US Dollar Index
- `msci-world` - MSCI World Index

## Common Use Cases

### 1. Market Overview Dashboard

Display current state of major markets for quick assessment.

### 2. Global Market Comparison

Compare performance across different regions and markets.

### 3. Volatility Monitoring

Track VIX and other volatility indices for risk assessment.

### 4. Currency Strength Analysis

Monitor currency indices for forex trading or international exposure.

### 5. Sector Rotation Analysis

Compare performance of different regional indices to identify trends.

### 6. Risk-On/Risk-Off Indicator

Use index correlations to gauge market sentiment.

### 7. Portfolio Benchmarking

Compare portfolio performance against relevant indices.

### 8. Market Timing

Use index trends and volatility for entry/exit decisions.

## Best Practices

1. **Cache Data**: Index data updates frequently but not every second
2. **Handle Missing Returns**: Not all return periods available for all indices
3. **Parse Carefully**: Change and percent fields are strings with formatting
4. **Time Zones**: Be aware of different market hours globally
5. **Update Frequency**: Refresh during market hours, less frequently after close
6. **Error Handling**: Some indices may be temporarily unavailable
7. **Regional Context**: Consider local market hours and holidays
8. **Currency Awareness**: Some indices are in local currency
9. **Historical Context**: Compare current levels to historical ranges
10. **Multiple Timeframes**: Look at multiple return periods for complete picture

## Important Notes

### Market Hours

- US markets: 9:30 AM - 4:00 PM ET
- European markets: Various, typically 9:00 AM - 5:30 PM local
- Asian markets: Various, typically 9:00 AM - 3:00 PM local
- Indices update in real-time during market hours
- After-hours: Last closing value shown

### Data Timeliness

- Real-time during market hours (may have 15-minute delay)
- End-of-day values after market close
- Historical returns calculated from closing prices
- Some indices may have delayed data

### Index Calculation

- Price-weighted (e.g., Dow Jones)
- Market-cap weighted (e.g., S&P 500)
- Equal-weighted (e.g., some sector indices)
- Different methodologies affect interpretation

### Limitations

- Not all indices available in all regions
- Some indices require special data subscriptions
- Historical data availability varies
- Currency conversions may apply for international indices

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_market_index("snp").await {
        Ok(index) => {
            println!("{}: {:.2}", index.name, index.value);
        }
        Err(YahooError::NotFound) => {
            println!("Index not found");
        }
        Err(YahooError::ParseError(msg)) => {
            println!("Failed to parse index data: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## Parsing Helper Functions

### Parse Percent Change

```rust
fn parse_percent_change(pct_str: &str) -> Option<f64> {
    pct_str
        .trim_start_matches('+')
        .trim_end_matches('%')
        .parse::<f64>()
        .ok()
}

// Usage
let change = parse_percent_change(&index.percent_change);
```

### Parse Point Change

```rust
fn parse_point_change(change_str: &str) -> Option<f64> {
    change_str
        .trim_start_matches('+')
        .parse::<f64>()
        .ok()
}

// Usage
let points = parse_point_change(&index.change);
```

### Determine Market Direction

```rust
fn market_direction(index: &MarketIndex) -> &str {
    if index.percent_change.starts_with('+') {
        "UP"
    } else if index.percent_change.starts_with('-') {
        "DOWN"
    } else {
        "FLAT"
    }
}
```

## Related APIs

- **Quote API**: Get individual stock quotes
- **Historical API**: Get historical index data
- **Sectors API**: Get sector performance
- **Market Summary**: Get broader market overview

## Performance Tips

- Batch requests for multiple indices
- Cache index data (updates every few seconds during market hours)
- Use async/await for concurrent fetching
- Consider rate limiting for frequent updates
- Store historical data locally for trend analysis

## Regional Index Groups

### Americas
```rust
let americas = vec!["snp", "djia", "nasdaq", "tsx-composite", "ibovespa", "ipc-mexico"];
```

### Europe
```rust
let europe = vec!["ftse-100", "dax", "cac-40", "euro-stoxx-50", "ftse-mib", "ibex-35"];
```

### Asia-Pacific
```rust
let asia_pacific = vec!["nikkei-225", "hang-seng", "shanghai", "sensex", "asx-200", "kospi"];
```

### Volatility
```rust
let volatility = vec!["vix", "india-vix"];
```

### Currency
```rust
let currency = vec!["usd", "gbp", "euro", "yen", "australian"];
```

## Market Sentiment Indicators

### Risk-On Indicators
- Rising equity indices
- Falling VIX
- Outperformance of small-caps (Russell 2000)
- Emerging market strength

### Risk-Off Indicators
- Falling equity indices
- Rising VIX (>20)
- Underperformance of small-caps
- Flight to quality (US Dollar Index rising)

### Divergence Signals
- US indices up, international down (or vice versa)
- Large-cap outperforming small-cap significantly
- High volatility despite rising prices

