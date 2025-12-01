# Index Streaming

## Overview

The `IndexStream` provides real-time streaming of market index data from Yahoo Finance. It continuously polls for updates at configurable intervals and yields index data including price, change, and percent change.

## Basic Usage

### Stream Major US Indices

The simplest way to stream major US market indices (S&P 500, Dow Jones, NASDAQ):

```rust
use finance_query_core::{IndexStream, YahooFinanceClient};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Arc::new(/* ... initialize YahooFinanceClient ... */);
    
    // Create stream for major US indices
    let mut stream = IndexStream::us_major_indices(
        client.clone(),
        Duration::from_secs(5)
    );
    
    // Process updates
    while let Some(result) = stream.next().await {
        match result {
            Ok(indices) => {
                for index in indices {
                    println!(
                        "{}: {:.2} ({:+})",
                        index.name,
                        index.value,
                        index.percent_change
                    );
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

### Stream Custom Indices

Stream any market indices by providing their Yahoo Finance symbols:

```rust
use finance_query_core::IndexStream;

// Define custom indices to stream
let symbols = vec![
    "^GSPC".to_string(),  // S&P 500
    "^FTSE".to_string(),  // FTSE 100
    "^N225".to_string(),  // Nikkei 225
    "^GDAXI".to_string(), // DAX
];

// Create stream with 10-second interval
let mut stream = IndexStream::new(
    client,
    symbols,
    Duration::from_secs(10)
);

while let Some(Ok(indices)) = stream.next().await {
    for index in indices {
        println!("{}: {:.2}", index.name, index.value);
    }
}
```

### Default Interval

Use the default 5-second polling interval:

```rust
let symbols = vec!["^GSPC".to_string(), "^DJI".to_string()];
let mut stream = IndexStream::with_default_interval(client, symbols);
```

## Index Symbols

Common market index symbols for Yahoo Finance:

### United States
- `^GSPC` - S&P 500
- `^DJI` - Dow Jones Industrial Average
- `^IXIC` - NASDAQ Composite
- `^RUT` - Russell 2000
- `^VIX` - CBOE Volatility Index

### Europe
- `^FTSE` - FTSE 100 (UK)
- `^GDAXI` - DAX (Germany)
- `^FCHI` - CAC 40 (France)
- `^STOXX50E` - Euro Stoxx 50

### Asia
- `^N225` - Nikkei 225 (Japan)
- `^HSI` - Hang Seng (Hong Kong)
- `^STI` - Straits Times (Singapore)
- `^KS11` - KOSPI (South Korea)

### Other Regions
- `^GSPTSE` - S&P/TSX Composite (Canada)
- `^AXJO` - ASX 200 (Australia)
- `^BVSP` - Bovespa (Brazil)

## MarketIndex Data Structure

Each update yields a `Vec<MarketIndex>` with the following fields:

```rust
pub struct MarketIndex {
    pub name: String,              // Index name (e.g., "S&P 500")
    pub value: f64,                // Current value
    pub change: String,            // Change (e.g., "+10.50")
    pub percent_change: String,    // Percent change (e.g., "+0.25%")
    
    // Optional performance metrics
    pub five_days_return: Option<String>,
    pub one_month_return: Option<String>,
    pub ytd_return: Option<String>,
    pub year_return: Option<String>,
    // ... more return periods
}
```

## Advanced Examples

### Filter by Change

Only process indices with significant changes:

```rust
while let Some(Ok(indices)) = stream.next().await {
    for index in indices {
        // Parse percent change
        let pct = index.percent_change
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0);
        
        // Only show if change > 1%
        if pct.abs() > 1.0 {
            println!("⚠️  {}: {:+.2}%", index.name, pct);
        }
    }
}
```

### Log to File

Stream and log index data to a file:

```rust
use std::fs::OpenOptions;
use std::io::Write;

let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("index_data.log")?;

while let Some(Ok(indices)) = stream.next().await {
    let timestamp = chrono::Utc::now();
    for index in indices {
        writeln!(
            file,
            "{},{},{},{}",
            timestamp,
            index.name,
            index.value,
            index.percent_change
        )?;
    }
}
```

### Multiple Streams

Run multiple streams concurrently:

```rust
use tokio::select;

let mut us_stream = IndexStream::us_major_indices(
    client.clone(),
    Duration::from_secs(5)
);

let eu_symbols = vec!["^FTSE".to_string(), "^GDAXI".to_string()];
let mut eu_stream = IndexStream::new(
    client.clone(),
    eu_symbols,
    Duration::from_secs(5)
);

loop {
    select! {
        Some(Ok(indices)) = us_stream.next() => {
            println!("US Update: {} indices", indices.len());
        }
        Some(Ok(indices)) = eu_stream.next() => {
            println!("EU Update: {} indices", indices.len());
        }
    }
}
```

## Error Handling

The stream yields `Result<Vec<MarketIndex>, YahooError>`:

```rust
while let Some(result) = stream.next().await {
    match result {
        Ok(indices) => {
            // Process successful update
            for index in indices {
                println!("{}: {:.2}", index.name, index.value);
            }
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
let stream = IndexStream::new(
    client,
    symbols,
    Duration::from_secs(30)
);
```

### Number of Indices

- Yahoo Finance allows multiple symbols in one request
- Recommended: 10-20 indices per stream
- For more indices, consider multiple streams

### Rate Limiting

Yahoo Finance has undocumented rate limits. Best practices:

1. Use reasonable polling intervals (≥5 seconds)
2. Implement exponential backoff on errors
3. Cache data when possible
4. Monitor for 429 (Rate Limited) errors

## Integration with WebSocket Servers

Use `IndexStream` to power WebSocket endpoints:

```rust
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    let client = Arc::new(/* ... */);
    let mut stream = IndexStream::us_major_indices(
        client,
        Duration::from_secs(5)
    );
    
    while let Some(Ok(indices)) = stream.next().await {
        let json = serde_json::to_string(&indices).unwrap();
        if socket.send(json.into()).await.is_err() {
            break;
        }
    }
}
```

## Examples

See the examples directory for complete implementations:

- `stream_indices.rs` - Basic index streaming
- `stream_custom_indices.rs` - Custom indices with CLI arguments

Run examples:

```bash
# Stream major US indices
cargo run --example stream_indices

# Stream custom indices
cargo run --example stream_custom_indices ^GSPC ^FTSE ^N225
```

## See Also

- [QuoteStream](./quote-streaming.md) - Stream stock quotes
- [MarketIndex Model](../models/indices.md) - Index data structure
- [Error Handling](../error-handling.md) - Handle streaming errors
