# Historical Price Data API

The Historical Data API provides access to historical price and volume data with support for various time ranges, intervals, and technical indicators.

## Overview

This module provides:

- **OHLCV Data**: Open, High, Low, Close, Volume for any time period
- **Adjusted Close**: Split and dividend-adjusted closing prices
- **Technical Indicators**: SMA (Simple Moving Average) and EMA (Exponential Moving Average)
- **Flexible Time Ranges**: From 1 day to maximum available history
- **Multiple Intervals**: From 1-minute to monthly data

## Data Structures

### HistoricalData

Individual price bar/candle data.

```rust
pub struct HistoricalData {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub adj_close: Option<f64>,
    pub sma: Option<HashMap<String, f64>>,
    pub ema: Option<HashMap<String, f64>>,
}
```

**Fields:**
- `open`: Opening price for the period
- `high`: Highest price during the period
- `low`: Lowest price during the period
- `close`: Closing price for the period
- `volume`: Trading volume (number of shares)
- `adj_close`: Adjusted closing price (accounts for splits/dividends)
- `sma`: Simple moving averages (optional, keyed by period)
- `ema`: Exponential moving averages (optional, keyed by period)

### HistoricalResponse

Response containing historical data points.

```rust
pub struct HistoricalResponse {
    pub data: HashMap<String, HistoricalData>,
}
```

**Fields:**
- `data`: Map of timestamps to historical data points


### TimeRange

Predefined time range options.

```rust
pub enum TimeRange {
    Day,           // "1d"
    FiveDays,      // "5d"
    OneMonth,      // "1mo"
    ThreeMonths,   // "3mo"
    SixMonths,     // "6mo"
    Year,          // "1y"
    TwoYears,      // "2y"
    FiveYears,     // "5y"
    TenYears,      // "10y"
    Ytd,           // "ytd" (Year to date)
    Max,           // "max" (All available data)
}
```

### Interval

Data granularity/interval options.

```rust
pub enum Interval {
    OneMinute,         // "1m"
    ThreeMinutes,      // "3m"
    FiveMinutes,       // "5m"
    TenMinutes,        // "10m"
    FifteenMinutes,    // "15m"
    TwentyMinutes,     // "20m"
    ThirtyMinutes,     // "30m"
    SixtyFiveMinutes,  // "65m"
    NinetyFiveMinutes, // "95m"
    OneHour,           // "1h"
    Daily,             // "1d"
    Weekly,            // "1wk"
    Monthly,           // "1mo"
}
```

### IndicatorType

Technical indicator types.

```rust
pub enum IndicatorType {
    SMA,  // Simple Moving Average
    EMA,  // Exponential Moving Average
}
```

## Usage Examples

### Get Daily Historical Data

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Get 1 year of daily data
    let history = client.get_historical("AAPL", "1y", "1d").await?;
    
    println!("Historical data points: {}", history.data.len());
    
    // Display first few data points
    let mut dates: Vec<_> = history.data.keys().collect();
    dates.sort();
    
    for date in dates.iter().take(5) {
        if let Some(data) = history.data.get(*date) {
            println!("{}: O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}", 
                date,
                data.open,
                data.high,
                data.low,
                data.close,
                data.volume
            );
        }
    }
    
    Ok(())
}
```

### Get Intraday Data

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Get 5 days of 5-minute data
    let history = client.get_historical("TSLA", "5d", "5m").await?;
    
    println!("5-minute bars: {}", history.data.len());
    
    // Find highest and lowest prices
    let mut high_price = 0.0;
    let mut low_price = f64::MAX;
    
    for data in history.data.values() {
        if data.high > high_price {
            high_price = data.high;
        }
        if data.low < low_price {
            low_price = data.low;
        }
    }
    
    println!("Period high: ${:.2}", high_price);
    println!("Period low: ${:.2}", low_price);
    println!("Range: ${:.2}", high_price - low_price);
    
    Ok(())
}
```

### Calculate Returns

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_historical("NVDA", "1y", "1d").await?;
    
    // Sort dates
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    if dates.len() >= 2 {
        // Get first and last prices
        let first_price = history.data.get(&dates[0])
            .map(|d| d.close)
            .unwrap_or(0.0);
        
        let last_price = history.data.get(dates.last().unwrap())
            .map(|d| d.close)
            .unwrap_or(0.0);
        
        // Calculate return
        let total_return = ((last_price - first_price) / first_price) * 100.0;
        
        println!("Period: {} to {}", dates[0], dates.last().unwrap());
        println!("Starting price: ${:.2}", first_price);
        println!("Ending price: ${:.2}", last_price);
        println!("Total return: {:.2}%", total_return);
    }
    
    Ok(())
}
```

### Analyze Volume Trends

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_historical("AAPL", "3mo", "1d").await?;
    
    // Calculate average volume
    let total_volume: i64 = history.data.values()
        .map(|d| d.volume)
        .sum();
    
    let avg_volume = total_volume / history.data.len() as i64;
    
    println!("Average daily volume: {}", avg_volume);
    
    // Find days with above-average volume
    let mut high_volume_days = Vec::new();
    
    for (date, data) in &history.data {
        if data.volume > avg_volume * 2 {
            high_volume_days.push((date.clone(), data.volume));
        }
    }
    
    high_volume_days.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\nTop 5 high volume days:");
    for (date, volume) in high_volume_days.iter().take(5) {
        let pct_above = ((volume - avg_volume) as f64 / avg_volume as f64) * 100.0;
        println!("{}: {} ({:.0}% above average)", date, volume, pct_above);
    }
    
    Ok(())
}
```

### Calculate Volatility

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_historical("TSLA", "1y", "1d").await?;
    
    // Calculate daily returns
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    let mut returns = Vec::new();
    
    for i in 1..dates.len() {
        let prev_close = history.data.get(&dates[i-1])
            .map(|d| d.close)
            .unwrap_or(0.0);
        
        let curr_close = history.data.get(&dates[i])
            .map(|d| d.close)
            .unwrap_or(0.0);
        
        if prev_close > 0.0 {
            let daily_return = (curr_close - prev_close) / prev_close;
            returns.push(daily_return);
        }
    }
    
    // Calculate standard deviation (volatility)
    if !returns.is_empty() {
        let mean: f64 = returns.iter().sum::<f64>() / returns.len() as f64;
        
        let variance: f64 = returns.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / returns.len() as f64;
        
        let std_dev = variance.sqrt();
        
        // Annualized volatility (assuming 252 trading days)
        let annual_volatility = std_dev * (252.0_f64).sqrt() * 100.0;
        
        println!("Daily volatility: {:.2}%", std_dev * 100.0);
        println!("Annualized volatility: {:.2}%", annual_volatility);
    }
    
    Ok(())
}
```

### Find Support and Resistance Levels

```rust
use finance_query_core::YahooClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_historical("AAPL", "6mo", "1d").await?;
    
    // Round prices to nearest dollar and count occurrences
    let mut price_levels: HashMap<i32, usize> = HashMap::new();
    
    for data in history.data.values() {
        let high_level = data.high.round() as i32;
        let low_level = data.low.round() as i32;
        
        *price_levels.entry(high_level).or_insert(0) += 1;
        *price_levels.entry(low_level).or_insert(0) += 1;
    }
    
    // Find most common price levels (potential support/resistance)
    let mut levels: Vec<_> = price_levels.iter().collect();
    levels.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("Potential Support/Resistance Levels:\n");
    
    for (price, count) in levels.iter().take(10) {
        println!("${}: touched {} times", price, count);
    }
    
    Ok(())
}
```

### Calculate Moving Averages Manually

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_historical("MSFT", "3mo", "1d").await?;
    
    // Sort dates
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    let period = 20; // 20-day SMA
    
    println!("20-Day Simple Moving Average:\n");
    
    for i in period..dates.len() {
        // Calculate SMA for last 'period' days
        let sum: f64 = dates[i-period..i]
            .iter()
            .filter_map(|date| history.data.get(date))
            .map(|d| d.close)
            .sum();
        
        let sma = sum / period as f64;
        let current_price = history.data.get(&dates[i])
            .map(|d| d.close)
            .unwrap_or(0.0);
        
        println!("{}: Price ${:.2}, SMA ${:.2}", 
            dates[i], 
            current_price, 
            sma
        );
        
        // Check for crossover
        if i > period {
            let prev_price = history.data.get(&dates[i-1])
                .map(|d| d.close)
                .unwrap_or(0.0);
            
            let prev_sum: f64 = dates[i-period-1..i-1]
                .iter()
                .filter_map(|date| history.data.get(date))
                .map(|d| d.close)
                .sum();
            
            let prev_sma = prev_sum / period as f64;
            
            if prev_price < prev_sma && current_price > sma {
                println!("  ⬆️ BULLISH CROSSOVER");
            } else if prev_price > prev_sma && current_price < sma {
                println!("  ⬇️ BEARISH CROSSOVER");
            }
        }
    }
    
    Ok(())
}
```

### Compare Multiple Stocks

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN"];
    
    println!("YTD Performance Comparison:\n");
    
    for symbol in symbols {
        match client.get_historical(symbol, "ytd", "1d").await {
            Ok(history) => {
                let mut dates: Vec<_> = history.data.keys().cloned().collect();
                dates.sort();
                
                if dates.len() >= 2 {
                    let first = history.data.get(&dates[0])
                        .map(|d| d.close)
                        .unwrap_or(0.0);
                    
                    let last = history.data.get(dates.last().unwrap())
                        .map(|d| d.close)
                        .unwrap_or(0.0);
                    
                    let return_pct = ((last - first) / first) * 100.0;
                    
                    println!("{}: {:+.2}%", symbol, return_pct);
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

### Detect Price Gaps

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_historical("TSLA", "3mo", "1d").await?;
    
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    println!("Price Gaps Detected:\n");
    
    for i in 1..dates.len() {
        let prev_data = history.data.get(&dates[i-1]);
        let curr_data = history.data.get(&dates[i]);
        
        if let (Some(prev), Some(curr)) = (prev_data, curr_data) {
            // Gap up: today's low > yesterday's high
            if curr.low > prev.high {
                let gap_size = ((curr.low - prev.high) / prev.high) * 100.0;
                println!("{}: GAP UP {:.2}%", dates[i], gap_size);
                println!("  Previous high: ${:.2}", prev.high);
                println!("  Current low: ${:.2}", curr.low);
            }
            
            // Gap down: today's high < yesterday's low
            if curr.high < prev.low {
                let gap_size = ((prev.low - curr.high) / prev.low) * 100.0;
                println!("{}: GAP DOWN {:.2}%", dates[i], gap_size);
                println!("  Previous low: ${:.2}", prev.low);
                println!("  Current high: ${:.2}", curr.high);
            }
        }
    }
    
    Ok(())
}
```

### Export to CSV

```rust
use finance_query_core::YahooClient;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let history = client.get_historical("AAPL", "1y", "1d").await?;
    
    let mut file = File::create("historical_data.csv")?;
    
    // Write header
    writeln!(file, "Date,Open,High,Low,Close,Volume,Adj Close")?;
    
    // Sort dates
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    // Write data rows
    for date in dates {
        if let Some(data) = history.data.get(&date) {
            writeln!(
                file,
                "{},{},{},{},{},{},{}",
                date,
                data.open,
                data.high,
                data.low,
                data.close,
                data.volume,
                data.adj_close.unwrap_or(data.close)
            )?;
        }
    }
    
    println!("Exported to historical_data.csv");
    
    Ok(())
}
```


## JSON Response Formats

### Basic Historical Data Response

```json
{
  "data": {
    "2024-01-02": {
      "open": 185.64,
      "high": 186.95,
      "low": 184.35,
      "close": 185.92,
      "volume": 82488200,
      "adj_close": 185.92
    },
    "2024-01-03": {
      "open": 184.22,
      "high": 185.88,
      "low": 183.43,
      "close": 184.25,
      "volume": 58414500,
      "adj_close": 184.25
    },
    "2024-01-04": {
      "open": 182.15,
      "high": 183.12,
      "low": 180.88,
      "close": 181.91,
      "volume": 77663600,
      "adj_close": 181.91
    }
  }
}
```

### With Technical Indicators

```json
{
  "data": {
    "2024-01-02": {
      "open": 185.64,
      "high": 186.95,
      "low": 184.35,
      "close": 185.92,
      "volume": 82488200,
      "adj_close": 185.92,
      "sma": {
        "20": 183.45,
        "50": 181.23,
        "200": 175.89
      },
      "ema": {
        "12": 184.67,
        "26": 182.91
      }
    }
  }
}
```

### Intraday Data (5-minute intervals)

```json
{
  "data": {
    "2024-12-01T09:30:00Z": {
      "open": 185.50,
      "high": 186.20,
      "low": 185.30,
      "close": 186.00,
      "volume": 1234567
    },
    "2024-12-01T09:35:00Z": {
      "open": 186.00,
      "high": 186.45,
      "low": 185.80,
      "close": 186.25,
      "volume": 987654
    },
    "2024-12-01T09:40:00Z": {
      "open": 186.25,
      "high": 186.50,
      "low": 185.95,
      "close": 186.10,
      "volume": 876543
    }
  }
}
```

### Minimal Response (No Adjusted Close)

```json
{
  "data": {
    "2024-01-02": {
      "open": 185.64,
      "high": 186.95,
      "low": 184.35,
      "close": 185.92,
      "volume": 82488200
    }
  }
}
```

## Time Range and Interval Combinations

### Valid Combinations

| Time Range | Valid Intervals | Notes |
|------------|----------------|-------|
| 1d | 1m, 3m, 5m, 10m, 15m, 30m, 1h | Intraday only |
| 5d | 1m, 3m, 5m, 10m, 15m, 30m, 1h, 1d | Mix of intraday and daily |
| 1mo | 5m, 15m, 30m, 1h, 1d | Recent month |
| 3mo | 1d, 1wk | Quarterly data |
| 6mo | 1d, 1wk | Half-year data |
| 1y | 1d, 1wk, 1mo | Annual data |
| 2y | 1d, 1wk, 1mo | Two years |
| 5y | 1d, 1wk, 1mo | Five years |
| 10y | 1d, 1wk, 1mo | Ten years |
| ytd | 1d, 1wk, 1mo | Year to date |
| max | 1d, 1wk, 1mo | All available |

### Interval Limitations

- **1-minute data**: Only available for last 7 days
- **5-minute data**: Only available for last 60 days
- **Hourly data**: Only available for last 730 days
- **Daily data**: Available for entire history
- **Weekly/Monthly**: Available for entire history

## Field Details

### OHLCV Fields

**Open**: First trade price of the period
**High**: Highest trade price during the period
**Low**: Lowest trade price during the period
**Close**: Last trade price of the period
**Volume**: Total number of shares traded

### Adjusted Close

- Accounts for corporate actions (splits, dividends)
- Used for accurate return calculations
- May be `None` for very recent data
- Always use adjusted close for historical analysis

### Technical Indicators

**SMA (Simple Moving Average)**:
- Arithmetic mean of prices over N periods
- Common periods: 20, 50, 100, 200 days
- Smooths out price action
- Lags current price

**EMA (Exponential Moving Average)**:
- Weighted average giving more weight to recent prices
- Common periods: 12, 26 days (for MACD)
- More responsive than SMA
- Less lag than SMA

### Timestamps

- Daily data: Date in `YYYY-MM-DD` format
- Intraday data: ISO 8601 timestamp with timezone
- All times in UTC for intraday data
- Market hours: 14:30-21:00 UTC (9:30 AM - 4:00 PM ET)

## Common Patterns

### Calculate Daily Returns

```rust
fn calculate_returns(history: &HistoricalResponse) -> Vec<(String, f64)> {
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    let mut returns = Vec::new();
    
    for i in 1..dates.len() {
        let prev = history.data.get(&dates[i-1]).unwrap();
        let curr = history.data.get(&dates[i]).unwrap();
        
        let return_pct = ((curr.close - prev.close) / prev.close) * 100.0;
        returns.push((dates[i].clone(), return_pct));
    }
    
    returns
}
```

### Find Highest Volume Day

```rust
fn find_highest_volume(history: &HistoricalResponse) -> Option<(String, i64)> {
    history.data
        .iter()
        .max_by_key(|(_, data)| data.volume)
        .map(|(date, data)| (date.clone(), data.volume))
}
```

### Calculate Average True Range (ATR)

```rust
fn calculate_atr(history: &HistoricalResponse, period: usize) -> f64 {
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    let mut true_ranges = Vec::new();
    
    for i in 1..dates.len() {
        let prev = history.data.get(&dates[i-1]).unwrap();
        let curr = history.data.get(&dates[i]).unwrap();
        
        let tr = (curr.high - curr.low)
            .max((curr.high - prev.close).abs())
            .max((curr.low - prev.close).abs());
        
        true_ranges.push(tr);
    }
    
    // Average of last 'period' true ranges
    if true_ranges.len() >= period {
        let sum: f64 = true_ranges.iter().rev().take(period).sum();
        sum / period as f64
    } else {
        0.0
    }
}
```

### Detect Trend Direction

```rust
fn detect_trend(history: &HistoricalResponse, sma_period: usize) -> String {
    let mut dates: Vec<_> = history.data.keys().cloned().collect();
    dates.sort();
    
    if dates.len() < sma_period {
        return "Insufficient data".to_string();
    }
    
    // Calculate SMA
    let recent_prices: Vec<f64> = dates.iter()
        .rev()
        .take(sma_period)
        .filter_map(|d| history.data.get(d))
        .map(|data| data.close)
        .collect();
    
    let sma: f64 = recent_prices.iter().sum::<f64>() / recent_prices.len() as f64;
    
    // Current price vs SMA
    let current_price = history.data.get(dates.last().unwrap()).unwrap().close;
    
    if current_price > sma * 1.02 {
        "Strong Uptrend".to_string()
    } else if current_price > sma {
        "Uptrend".to_string()
    } else if current_price < sma * 0.98 {
        "Strong Downtrend".to_string()
    } else if current_price < sma {
        "Downtrend".to_string()
    } else {
        "Sideways".to_string()
    }
}
```

## Best Practices

1. **Use Adjusted Close**: Always use `adj_close` for return calculations
2. **Sort Timestamps**: Data may not be in chronological order
3. **Handle Missing Data**: Check for `None` values in optional fields
4. **Respect Rate Limits**: Cache historical data locally
5. **Choose Appropriate Interval**: Match interval to analysis timeframe
6. **Validate Data**: Check for gaps, outliers, and anomalies
7. **Time Zone Awareness**: Intraday data is in UTC
8. **Volume Analysis**: Consider volume alongside price
9. **Corporate Actions**: Adjusted close accounts for splits/dividends
10. **Data Quality**: Verify data against multiple sources for critical applications

## Important Notes

### Data Availability

- Historical data availability varies by symbol
- Older stocks have more historical data
- New listings have limited history
- Some exchanges have delayed data
- Intraday data has retention limits

### Market Hours

- Regular hours: 9:30 AM - 4:00 PM ET (14:30-21:00 UTC)
- Pre-market: 4:00 AM - 9:30 AM ET (9:00-14:30 UTC)
- After-hours: 4:00 PM - 8:00 PM ET (21:00-01:00 UTC)
- Extended hours data may be limited

### Data Quality

- Prices are as-reported by exchanges
- Volume may include pre/post-market for daily data
- Adjusted close calculated by data provider
- Historical data may be restated
- Always validate critical data

### Performance Considerations

- Large time ranges return more data
- Intraday data is more granular (larger response)
- Consider pagination for very large datasets
- Cache data locally to reduce API calls
- Use appropriate intervals for your use case

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_historical("AAPL", "1y", "1d").await {
        Ok(history) => {
            if history.data.is_empty() {
                println!("No historical data available");
            } else {
                println!("Loaded {} data points", history.data.len());
            }
        }
        Err(YahooError::NotFound) => {
            println!("Symbol not found");
        }
        Err(YahooError::InvalidInterval) => {
            println!("Invalid time range or interval combination");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## Related APIs

- **Quote API**: Get current real-time price
- **Actions API**: Get dividend and split history
- **Technical Analysis**: Calculate indicators from historical data
- **Fundamentals API**: Combine with financial data for valuation

## Common Use Cases

### Backtesting Strategies

Use historical data to test trading strategies before deploying with real money.

### Technical Analysis

Calculate indicators like RSI, MACD, Bollinger Bands from OHLCV data.

### Risk Analysis

Calculate volatility, beta, and other risk metrics from historical returns.

### Portfolio Optimization

Use historical correlations to optimize portfolio allocation.

### Price Alerts

Monitor historical patterns to set intelligent price alerts.

### Charting

Display candlestick charts, line charts, and technical overlays.

### Machine Learning

Train predictive models using historical price and volume data.

### Correlation Analysis

Compare multiple stocks to find correlations and diversification opportunities.

