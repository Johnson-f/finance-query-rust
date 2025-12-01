# Options Model

The Options model provides comprehensive options chain data for stocks, including calls, puts, strikes, Greeks, and expiration dates.

## Overview

The options module contains three main structures:
- `OptionContract` - Represents a single option contract (call or put)
- `OptionChain` - Complete option chain for a specific expiration date
- `OptionExpirations` - List of available expiration dates for a symbol

These models enable options trading analysis, strategy development, and risk assessment.

## Data Structures

### OptionContract

Represents a single option contract with pricing and Greeks data.

```rust
pub struct OptionContract {
    pub contract_symbol: String,
    pub last_trade_date: DateTime<Utc>,
    pub strike: f64,
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub change: f64,
    pub percent_change: f64,
    pub volume: Option<u64>,
    pub open_interest: Option<u64>,
    pub implied_volatility: f64,
    pub in_the_money: bool,
    pub contract_size: String,
    pub currency: String,
}
```

**Fields:**
- `contract_symbol` - Unique identifier for the contract (e.g., "AAPL240119C00150000")
- `last_trade_date` - Timestamp of the last trade
- `strike` - Strike price of the option
- `last_price` - Last traded price
- `bid` - Current bid price
- `ask` - Current ask price
- `change` - Price change from previous close
- `percent_change` - Percentage change from previous close
- `volume` - Trading volume (may be None if no trades)
- `open_interest` - Number of open contracts (may be None)
- `implied_volatility` - Implied volatility as a decimal (e.g., 0.25 = 25%)
- `in_the_money` - Whether the option is currently in the money
- `contract_size` - Contract size (typically "REGULAR")
- `currency` - Currency denomination (typically "USD")

### OptionChain

Complete option chain for a specific expiration date.

```rust
pub struct OptionChain {
    pub symbol: String,
    pub expiration_date: String,
    pub calls: Vec<OptionContract>,
    pub puts: Vec<OptionContract>,
    pub underlying_price: Option<f64>,
}
```

**Fields:**
- `symbol` - Stock ticker symbol
- `expiration_date` - Expiration date in YYYY-MM-DD format
- `calls` - List of all call option contracts
- `puts` - List of all put option contracts
- `underlying_price` - Current price of the underlying stock

### OptionExpirations

List of available expiration dates for a symbol.

```rust
pub struct OptionExpirations {
    pub symbol: String,
    pub expirations: Vec<String>,
}
```

**Fields:**
- `symbol` - Stock ticker symbol
- `expirations` - List of expiration dates in YYYY-MM-DD format

## JSON Format

### OptionContract Example

```json
{
  "contractSymbol": "AAPL240119C00150000",
  "lastTradeDate": "2024-01-15T19:45:23Z",
  "strike": 150.0,
  "lastPrice": 28.50,
  "bid": 28.30,
  "ask": 28.70,
  "change": 1.25,
  "percentChange": 4.59,
  "volume": 1523,
  "openInterest": 8945,
  "impliedVolatility": 0.2847,
  "inTheMoney": true,
  "contractSize": "REGULAR",
  "currency": "USD"
}
```

### OptionChain Example

```json
{
  "symbol": "AAPL",
  "expirationDate": "2024-01-19",
  "underlyingPrice": 178.50,
  "calls": [
    {
      "contractSymbol": "AAPL240119C00150000",
      "lastTradeDate": "2024-01-15T19:45:23Z",
      "strike": 150.0,
      "lastPrice": 28.50,
      "bid": 28.30,
      "ask": 28.70,
      "change": 1.25,
      "percentChange": 4.59,
      "volume": 1523,
      "openInterest": 8945,
      "impliedVolatility": 0.2847,
      "inTheMoney": true,
      "contractSize": "REGULAR",
      "currency": "USD"
    },
    {
      "contractSymbol": "AAPL240119C00175000",
      "lastTradeDate": "2024-01-15T19:30:12Z",
      "strike": 175.0,
      "lastPrice": 5.20,
      "bid": 5.10,
      "ask": 5.30,
      "change": 0.45,
      "percentChange": 9.47,
      "volume": 2341,
      "openInterest": 12456,
      "impliedVolatility": 0.3125,
      "inTheMoney": true,
      "contractSize": "REGULAR",
      "currency": "USD"
    }
  ],
  "puts": [
    {
      "contractSymbol": "AAPL240119P00150000",
      "lastTradeDate": "2024-01-15T18:22:45Z",
      "strike": 150.0,
      "lastPrice": 0.15,
      "bid": 0.12,
      "ask": 0.18,
      "change": -0.03,
      "percentChange": -16.67,
      "volume": 456,
      "openInterest": 3421,
      "impliedVolatility": 0.4523,
      "inTheMoney": false,
      "contractSize": "REGULAR",
      "currency": "USD"
    }
  ]
}
```

### OptionExpirations Example

```json
{
  "symbol": "AAPL",
  "expirations": [
    "2024-01-19",
    "2024-01-26",
    "2024-02-02",
    "2024-02-16",
    "2024-03-15",
    "2024-06-21",
    "2024-09-20",
    "2025-01-17",
    "2026-01-16"
  ]
}
```

## Usage Examples

### Getting Available Expirations

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let expirations = client.get_option_expirations("AAPL").await?;
    
    println!("Available expirations for {}:", expirations.symbol);
    for (i, date) in expirations.expirations.iter().enumerate() {
        println!("{}. {}", i + 1, date);
    }
    
    Ok(())
}
```

### Fetching an Option Chain

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Get the nearest expiration
    let expirations = client.get_option_expirations("TSLA").await?;
    let nearest_exp = expirations.expirations.first().unwrap();
    
    // Fetch the option chain
    let chain = client.get_option_chain("TSLA", Some(nearest_exp)).await?;
    
    println!("Option Chain for {} expiring {}", chain.symbol, chain.expiration_date);
    if let Some(price) = chain.underlying_price {
        println!("Underlying Price: ${:.2}", price);
    }
    println!("Calls: {} contracts", chain.calls.len());
    println!("Puts: {} contracts", chain.puts.len());
    
    Ok(())
}
```

### Analyzing At-The-Money Options

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let chain = client.get_option_chain("NVDA", None).await?;
    
    if let Some(underlying) = chain.underlying_price {
        println!("Underlying: ${:.2}\n", underlying);
        
        // Find ATM call (strike closest to underlying price)
        let atm_call = chain.calls
            .iter()
            .min_by_key(|c| ((c.strike - underlying).abs() * 100.0) as i64);
        
        if let Some(call) = atm_call {
            println!("ATM Call:");
            println!("  Strike: ${:.2}", call.strike);
            println!("  Last: ${:.2}", call.last_price);
            println!("  Bid/Ask: ${:.2} / ${:.2}", call.bid, call.ask);
            println!("  IV: {:.2}%", call.implied_volatility * 100.0);
            println!("  Volume: {}", call.volume.unwrap_or(0));
            println!("  Open Interest: {}", call.open_interest.unwrap_or(0));
        }
        
        // Find ATM put
        let atm_put = chain.puts
            .iter()
            .min_by_key(|p| ((p.strike - underlying).abs() * 100.0) as i64);
        
        if let Some(put) = atm_put {
            println!("\nATM Put:");
            println!("  Strike: ${:.2}", put.strike);
            println!("  Last: ${:.2}", put.last_price);
            println!("  Bid/Ask: ${:.2} / ${:.2}", put.bid, put.ask);
            println!("  IV: {:.2}%", put.implied_volatility * 100.0);
            println!("  Volume: {}", put.volume.unwrap_or(0));
            println!("  Open Interest: {}", put.open_interest.unwrap_or(0));
        }
    }
    
    Ok(())
}
```

### Finding High Volume Options

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let chain = client.get_option_chain("SPY", None).await?;
    
    // Find calls with highest volume
    let mut high_volume_calls = chain.calls.clone();
    high_volume_calls.sort_by_key(|c| std::cmp::Reverse(c.volume.unwrap_or(0)));
    
    println!("Top 5 Calls by Volume:");
    for call in high_volume_calls.iter().take(5) {
        println!(
            "Strike ${:.2} | Vol: {} | OI: {} | Last: ${:.2}",
            call.strike,
            call.volume.unwrap_or(0),
            call.open_interest.unwrap_or(0),
            call.last_price
        );
    }
    
    Ok(())
}
```

### Calculating Option Spreads

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let chain = client.get_option_chain("AAPL", None).await?;
    
    if let Some(underlying) = chain.underlying_price {
        // Find two strikes for a bull call spread
        let lower_strike = underlying - 5.0;
        let upper_strike = underlying + 5.0;
        
        let buy_call = chain.calls
            .iter()
            .min_by_key(|c| ((c.strike - lower_strike).abs() * 100.0) as i64);
        
        let sell_call = chain.calls
            .iter()
            .min_by_key(|c| ((c.strike - upper_strike).abs() * 100.0) as i64);
        
        if let (Some(buy), Some(sell)) = (buy_call, sell_call) {
            let net_debit = buy.ask - sell.bid;
            let max_profit = (sell.strike - buy.strike) - net_debit;
            let max_loss = net_debit;
            
            println!("Bull Call Spread:");
            println!("  Buy ${:.2} Call @ ${:.2}", buy.strike, buy.ask);
            println!("  Sell ${:.2} Call @ ${:.2}", sell.strike, sell.bid);
            println!("  Net Debit: ${:.2}", net_debit);
            println!("  Max Profit: ${:.2}", max_profit);
            println!("  Max Loss: ${:.2}", max_loss);
            println!("  Risk/Reward: {:.2}", max_profit / max_loss);
        }
    }
    
    Ok(())
}
```

### Analyzing Implied Volatility

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let chain = client.get_option_chain("TSLA", None).await?;
    
    // Calculate average IV for calls
    let call_iv_sum: f64 = chain.calls.iter().map(|c| c.implied_volatility).sum();
    let avg_call_iv = call_iv_sum / chain.calls.len() as f64;
    
    // Calculate average IV for puts
    let put_iv_sum: f64 = chain.puts.iter().map(|p| p.implied_volatility).sum();
    let avg_put_iv = put_iv_sum / chain.puts.len() as f64;
    
    println!("Implied Volatility Analysis:");
    println!("  Average Call IV: {:.2}%", avg_call_iv * 100.0);
    println!("  Average Put IV: {:.2}%", avg_put_iv * 100.0);
    println!("  Put/Call IV Ratio: {:.2}", avg_put_iv / avg_call_iv);
    
    // Find highest IV options
    let max_iv_call = chain.calls.iter().max_by(|a, b| {
        a.implied_volatility.partial_cmp(&b.implied_volatility).unwrap()
    });
    
    if let Some(call) = max_iv_call {
        println!("\nHighest IV Call:");
        println!("  Strike: ${:.2}", call.strike);
        println!("  IV: {:.2}%", call.implied_volatility * 100.0);
    }
    
    Ok(())
}
```

### Finding In-The-Money Options

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let chain = client.get_option_chain("MSFT", None).await?;
    
    let itm_calls: Vec<_> = chain.calls
        .iter()
        .filter(|c| c.in_the_money)
        .collect();
    
    let itm_puts: Vec<_> = chain.puts
        .iter()
        .filter(|p| p.in_the_money)
        .collect();
    
    println!("In-The-Money Options:");
    println!("  ITM Calls: {}", itm_calls.len());
    println!("  ITM Puts: {}", itm_puts.len());
    
    println!("\nITM Calls with High Open Interest:");
    let mut sorted_calls = itm_calls.clone();
    sorted_calls.sort_by_key(|c| std::cmp::Reverse(c.open_interest.unwrap_or(0)));
    
    for call in sorted_calls.iter().take(5) {
        println!(
            "  ${:.2} strike | OI: {} | Last: ${:.2}",
            call.strike,
            call.open_interest.unwrap_or(0),
            call.last_price
        );
    }
    
    Ok(())
}
```

### Building an Options Scanner

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN", "TSLA"];
    
    println!("Options Scanner - High IV Opportunities\n");
    
    for symbol in symbols {
        let chain = client.get_option_chain(symbol, None).await?;
        
        // Find options with IV > 50%
        let high_iv_calls: Vec<_> = chain.calls
            .iter()
            .filter(|c| c.implied_volatility > 0.50)
            .collect();
        
        if !high_iv_calls.is_empty() {
            println!("{} - {} high IV calls", symbol, high_iv_calls.len());
            
            for call in high_iv_calls.iter().take(3) {
                println!(
                    "  ${:.2} strike | IV: {:.2}% | Last: ${:.2}",
                    call.strike,
                    call.implied_volatility * 100.0,
                    call.last_price
                );
            }
            println!();
        }
    }
    
    Ok(())
}
```

### Exporting Options Data

```rust
use finance_query_core::YahooClient;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let chain = client.get_option_chain("AAPL", None).await?;
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&chain)?;
    std::fs::write("aapl_options.json", json)?;
    
    // Export calls to CSV
    let mut csv = String::from("Strike,Last,Bid,Ask,Volume,OpenInterest,IV,ITM\n");
    for call in &chain.calls {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            call.strike,
            call.last_price,
            call.bid,
            call.ask,
            call.volume.unwrap_or(0),
            call.open_interest.unwrap_or(0),
            call.implied_volatility,
            call.in_the_money
        ));
    }
    std::fs::write("aapl_calls.csv", csv)?;
    
    println!("Options data exported successfully");
    
    Ok(())
}
```

### Calculating Put-Call Ratio

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let chain = client.get_option_chain("SPY", None).await?;
    
    // Calculate volume-based put-call ratio
    let call_volume: u64 = chain.calls
        .iter()
        .filter_map(|c| c.volume)
        .sum();
    
    let put_volume: u64 = chain.puts
        .iter()
        .filter_map(|p| p.volume)
        .sum();
    
    let volume_pcr = put_volume as f64 / call_volume as f64;
    
    // Calculate open interest-based put-call ratio
    let call_oi: u64 = chain.calls
        .iter()
        .filter_map(|c| c.open_interest)
        .sum();
    
    let put_oi: u64 = chain.puts
        .iter()
        .filter_map(|p| p.open_interest)
        .sum();
    
    let oi_pcr = put_oi as f64 / call_oi as f64;
    
    println!("Put-Call Ratio Analysis for {}:", chain.symbol);
    println!("  Volume PCR: {:.3}", volume_pcr);
    println!("  Open Interest PCR: {:.3}", oi_pcr);
    println!("\nInterpretation:");
    if volume_pcr > 1.0 {
        println!("  High put volume suggests bearish sentiment");
    } else {
        println!("  High call volume suggests bullish sentiment");
    }
    
    Ok(())
}
```

## Contract Symbol Format

Option contract symbols follow the OCC (Options Clearing Corporation) format:

```
AAPL240119C00150000
└─┬─┘└──┬──┘│└───┬───┘
  │     │   │    └─ Strike price (150.00) with 3 decimal places
  │     │   └────── Option type (C=Call, P=Put)
  │     └────────── Expiration date (YYMMDD format: Jan 19, 2024)
  └──────────────── Underlying symbol
```

## Key Concepts

### Implied Volatility (IV)
- Expressed as a decimal (0.25 = 25%)
- Higher IV indicates higher option premiums
- IV typically increases before earnings or major events
- Compare IV across strikes to identify volatility skew

### Open Interest
- Total number of outstanding contracts
- High open interest indicates liquid options
- Changes in open interest can signal new positions

### In-The-Money (ITM)
- **Calls**: Strike < Underlying Price
- **Puts**: Strike > Underlying Price
- ITM options have intrinsic value

### Bid-Ask Spread
- Difference between bid and ask prices
- Narrow spreads indicate liquid markets
- Wide spreads suggest illiquid options

## Common Strategies

### Covered Call
Buy stock + Sell call option (generates income)

### Protective Put
Buy stock + Buy put option (downside protection)

### Bull Call Spread
Buy lower strike call + Sell higher strike call (limited risk/reward)

### Iron Condor
Sell OTM call spread + Sell OTM put spread (profit from low volatility)

### Straddle
Buy ATM call + Buy ATM put (profit from large moves in either direction)

## Notes

- Options data is delayed by 15-20 minutes for most exchanges
- Volume and open interest may be None for newly listed contracts
- Implied volatility is calculated using the Black-Scholes model
- Contract size is typically "REGULAR" (100 shares per contract)
- Prices are in the currency of the underlying stock (usually USD)
- Expiration dates are typically the third Friday of the month
- Weekly options may have different expiration patterns
- LEAPS (Long-term Equity Anticipation Securities) have expirations 1+ years out

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_option_chain("AAPL", None).await {
        Ok(chain) => {
            println!("Retrieved {} calls and {} puts", 
                chain.calls.len(), chain.puts.len());
        }
        Err(YahooError::NotFound) => {
            eprintln!("No options available for this symbol");
        }
        Err(YahooError::ParseError(msg)) => {
            eprintln!("Failed to parse options data: {}", msg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## See Also

- [Quote Model](./quote.md) - For underlying stock prices
- [Historical Model](./historical.md) - For historical price data
- [Calendar Model](./calendar.md) - For earnings dates that affect options
