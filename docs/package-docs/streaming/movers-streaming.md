# Movers Streaming

## Overview

The `MoversStream` provides real-time streaming of market movers data from Yahoo Finance. It continuously polls for updates at configurable intervals and yields three categories of movers: most active stocks by volume, top gainers, and top losers.

**Note:** This stream automatically filters for US stocks only (symbols without dots or with US exchange suffixes like .OB, .PK).

## Basic Usage

### Stream Market Movers with Defaults

The simplest way to stream market movers with default settings (50 stocks per category, 5-second interval):

```rust
use finance_query_core::{MoversStream, YahooFinanceClient};
use futures_util::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Arc::new(/* ... initialize YahooFinanceClient ... */);
    
    // Create stream with defaults
    let mut stream = MoversStream::with_defaults(client.clone());
    
    // Process updates
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                println!("Actives: {}", update.actives.len());
                println!("Gainers: {}", update.gainers.len());
                println!("Losers: {}", update.losers.len());
                
                // Show top gainer
                if let Some(top) = update.gainers.first() {
                    println!(
                        "Top gainer: {} ({}) {}",
                        top.symbol,
                        top.name,
                        top.percent_change
                    );
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

### Stream with Custom Count

Control how many movers to fetch per category (25, 50, or 100):

```rust
use finance_query_core::{MoversStream, MoverCount};
use std::time::Duration;

// Get top 100 movers in each category
let mut stream = MoversStream::new(
    client,
    MoverCount::Hundred,
    Duration::from_secs(10)
);

while let Some(Ok(update)) = stream.next().await {
    println!("Fetched {} actives", update.actives.len());
    println!("Fetched {} gainers", update.gainers.len());
    println!("Fetched {} losers", update.losers.len());
}
```

### Default Interval

Use the default 5-second polling interval with custom count:

```rust
let mut stream = MoversStream::with_default_interval(
    client,
    MoverCount::TwentyFive
);
```

## MoverCount Options

The `MoverCount` enum controls how many stocks to fetch per category:

```rust
pub enum MoverCount {
    TwentyFive,  // Top 25 in each category
    Fifty,       // Top 50 in each category (default)
    Hundred,     // Top 100 in each category
}
```

## MoversUpdate Data Structure

Each update yields a `MoversUpdate` with the following structure:

```rust
pub struct MoversUpdate {
    pub actives: Vec<MarketMover>,   // Most active by volume
    pub gainers: Vec<MarketMover>,   // Top gainers by percent
    pub losers: Vec<MarketMover>,    // Top losers by percent
    pub timestamp: DateTime<Utc>,    // When data was fetched
}
```

### MarketMover Schema

Each mover contains:

```rust
pub struct MarketMover {
    pub symbol: String,           // Stock symbol (e.g., "AAPL")
    pub name: String,             // Company name
    pub price: String,            // Current price (e.g., "145.00")
    pub change: String,           // Price change (e.g., "+1.00")
    pub percent_change: String,   // Percent change (e.g., "+0.69%")
}
```

## Advanced Examples

### Display Top Movers

Show the top 5 from each category:

```rust
while let Some(Ok(update)) = stream.next().await {
    println!("\n📈 Top 5 Gainers:");
    for (i, mover) in update.gainers.iter().take(5).enumerate() {
        println!(
            "  {}. {} ({}): {} {}",
            i + 1,
            mover.symbol,
            mover.name,
            mover.price,
            mover.percent_change
        );
    }

    println!("\n📉 Top 5 Losers:");
    for (i, mover) in update.losers.iter().take(5).enumerate() {
        println!(
            "  {}. {} ({}): {} {}",
            i + 1,
            mover.symbol,
            mover.name,
            mover.price,
            mover.percent_change
        );
    }

    println!("\n🔥 Top 5 Most Active:");
    for (i, mover) in update.actives.iter().take(5).enumerate() {
        println!(
            "  {}. {} ({}): {} {}",
            i + 1,
            mover.symbol,
            mover.name,
            mover.price,
            mover.percent_change
        );
    }
}
```

### Filter by Threshold

Only process stocks with significant moves:

```rust
while let Some(Ok(update)) = stream.next().await {
    // Filter gainers with > 5% increase
    let big_gainers: Vec<_> = update.gainers
        .iter()
        .filter(|m| {
            m.percent_change
                .trim_start_matches('+')
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(0.0) > 5.0
        })
        .collect();
    
    if !big_gainers.is_empty() {
        println!("🚀 Stocks up more than 5%:");
        for mover in big_gainers {
            println!("  {} ({}): {}", mover.symbol, mover.name, mover.percent_change);
        }
    }
}
```

### Log to File

Stream and log movers data to a CSV file:

```rust
use std::fs::OpenOptions;
use std::io::Write;

let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("movers_data.csv")?;

// Write header
writeln!(file, "timestamp,category,symbol,name,price,change,percent_change")?;

while let Some(Ok(update)) = stream.next().await {
    let timestamp = update.timestamp;
    
    // Log gainers
    for mover in &update.gainers {
        writeln!(
            file,
            "{},gainer,{},{},{},{},{}",
            timestamp,
            mover.symbol,
            mover.name,
            mover.price,
            mover.change,
            mover.percent_change
        )?;
    }
    
    // Log losers
    for mover in &update.losers {
        writeln!(
            file,
            "{},loser,{},{},{},{},{}",
            timestamp,
            mover.symbol,
            mover.name,
            mover.price,
            mover.change,
            mover.percent_change
        )?;
    }
    
    // Log actives
    for mover in &update.actives {
        writeln!(
            file,
            "{},active,{},{},{},{},{}",
            timestamp,
            mover.symbol,
            mover.name,
            mover.price,
            mover.change,
            mover.percent_change
        )?;
    }
}
```

### Alert on Extreme Moves

Send alerts when stocks have extreme price movements:

```rust
while let Some(Ok(update)) = stream.next().await {
    // Check for extreme gainers (>10%)
    for mover in &update.gainers {
        if let Ok(pct) = mover.percent_change
            .trim_start_matches('+')
            .trim_end_matches('%')
            .parse::<f64>()
        {
            if pct > 10.0 {
                println!(
                    "🚨 ALERT: {} is up {:.2}% to ${}",
                    mover.symbol,
                    pct,
                    mover.price
                );
                // Send notification, email, etc.
            }
        }
    }
    
    // Check for extreme losers (<-10%)
    for mover in &update.losers {
        if let Ok(pct) = mover.percent_change
            .trim_end_matches('%')
            .parse::<f64>()
        {
            if pct < -10.0 {
                println!(
                    "🚨 ALERT: {} is down {:.2}% to ${}",
                    mover.symbol,
                    pct.abs(),
                    mover.price
                );
            }
        }
    }
}
```

### Track Specific Symbols

Monitor if specific symbols appear in movers:

```rust
let watchlist = vec!["AAPL", "TSLA", "NVDA", "AMD"];

while let Some(Ok(update)) = stream.next().await {
    // Check all categories
    let all_movers: Vec<_> = update.actives
        .iter()
        .chain(update.gainers.iter())
        .chain(update.losers.iter())
        .collect();
    
    for symbol in &watchlist {
        if let Some(mover) = all_movers.iter().find(|m| m.symbol == *symbol) {
            println!(
                "📍 {} is moving: {} ({})",
                symbol,
                mover.percent_change,
                mover.price
            );
        }
    }
}
```

## Error Handling

The stream yields `Result<MoversUpdate, YahooError>`:

```rust
while let Some(result) = stream.next().await {
    match result {
        Ok(update) => {
            // Process successful update
            println!("Received {} gainers", update.gainers.len());
        }
        Err(YahooError::RateLimited) => {
            eprintln!("Rate limited - waiting...");
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
        Err(YahooError::NetworkError(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## Performance Considerations

### Polling Interval

- **Minimum recommended**: 5 seconds
- **Default**: 5 seconds
- **For production**: 10-30 seconds to avoid rate limiting

```rust
// Conservative interval for production
let stream = MoversStream::new(
    client,
    MoverCount::Fifty,
    Duration::from_secs(30)
);
```

### Count Selection

- **TwentyFive**: Fastest, least data
- **Fifty**: Good balance (default)
- **Hundred**: Most comprehensive, slower

Choose based on your needs:
- Use `TwentyFive` for quick updates on top movers
- Use `Fifty` for general monitoring
- Use `Hundred` for comprehensive market analysis

### Rate Limiting

Yahoo Finance has undocumented rate limits. Best practices:

1. Use reasonable polling intervals (≥5 seconds)
2. Implement exponential backoff on errors
3. Cache data when possible
4. Monitor for 429 (Rate Limited) errors

## Integration with WebSocket Servers

Use `MoversStream` to power WebSocket endpoints:

```rust
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};

async fn ws_movers_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| handle_movers_socket(socket))
}

async fn handle_movers_socket(mut socket: WebSocket) {
    let client = Arc::new(/* ... */);
    let mut stream = MoversStream::with_defaults(client);
    
    while let Some(Ok(update)) = stream.next().await {
        // Send only the data (without timestamp if not needed)
        let message = serde_json::json!({
            "actives": update.actives,
            "gainers": update.gainers,
            "losers": update.losers
        });
        
        let json = serde_json::to_string(&message).unwrap();
        if socket.send(json.into()).await.is_err() {
            break;
        }
    }
}
```

## Examples

See the examples directory for complete implementations:

- `stream_movers.rs` - Basic movers streaming with top 5 display

Run the example:

```bash
# Stream market movers
cargo run --example stream_movers
```

## See Also

- [QuoteStream](./quote-streaming.md) - Stream stock quotes
- [IndexStream](./index-streaming.md) - Stream market indices
- [MarketMover Model](../models/movers.md) - Mover data structure
- [WebSocket Movers Endpoint](../../websockets/movers.md) - WebSocket implementation
- [Error Handling](../error-handling.md) - Handle streaming errors
