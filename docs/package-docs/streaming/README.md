# Streaming API

The finance-query-core library provides async streaming capabilities for real-time financial data. All streams are framework-agnostic and can be integrated with any async runtime or WebSocket server.

## Available Streams

### QuoteStream
Stream real-time stock quotes for multiple symbols.

- **Use case**: Monitor stock prices in real-time
- **Update frequency**: Configurable (default: 5 seconds)
- **Documentation**: [Quote Streaming](./quote-streaming.md)

```rust
use finance_query_core::QuoteStream;
use std::time::Duration;

let symbols = vec!["AAPL".to_string(), "GOOGL".to_string()];
let stream = QuoteStream::new(client, symbols, Duration::from_secs(5));
```

### IndexStream
Stream real-time market index data.

- **Use case**: Monitor major market indices (S&P 500, Dow Jones, NASDAQ, etc.)
- **Update frequency**: Configurable (default: 5 seconds)
- **Documentation**: [Index Streaming](./index-streaming.md)

```rust
use finance_query_core::IndexStream;
use std::time::Duration;

// Stream major US indices
let stream = IndexStream::us_major_indices(client, Duration::from_secs(5));

// Or stream custom indices
let symbols = vec!["^GSPC".to_string(), "^FTSE".to_string()];
let stream = IndexStream::new(client, symbols, Duration::from_secs(5));
```

### MoversStream
Stream real-time market movers (most active, top gainers, top losers).

- **Use case**: Track stocks with significant price movements or trading volume
- **Update frequency**: Configurable (default: 5 seconds)
- **Count options**: 25, 50, or 100 stocks per category
- **Documentation**: [Movers Streaming](./movers-streaming.md)

```rust
use finance_query_core::{MoversStream, MoverCount};
use std::time::Duration;

// Stream top 50 movers in each category
let stream = MoversStream::new(client, MoverCount::Fifty, Duration::from_secs(5));

// Or use defaults (50 movers, 5 seconds)
let stream = MoversStream::with_defaults(client);
```

## Common Patterns

### Basic Streaming

```rust
use futures_util::StreamExt;

let mut stream = QuoteStream::with_default_interval(client, symbols);

while let Some(result) = stream.next().await {
    match result {
        Ok(update) => {
            // Process update
            println!("Received {} quotes", update.quotes.len());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

### Error Handling

```rust
use finance_query_core::YahooError;

while let Some(result) = stream.next().await {
    match result {
        Ok(update) => { /* process */ }
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

### Multiple Concurrent Streams

```rust
use tokio::select;

let mut quote_stream = QuoteStream::with_default_interval(client.clone(), symbols);
let mut index_stream = IndexStream::us_major_indices(client.clone(), Duration::from_secs(5));
let mut movers_stream = MoversStream::with_defaults(client.clone());

loop {
    select! {
        Some(Ok(quotes)) = quote_stream.next() => {
            println!("Quote update: {} symbols", quotes.quotes.len());
        }
        Some(Ok(indices)) = index_stream.next() => {
            println!("Index update: {} indices", indices.len());
        }
        Some(Ok(movers)) = movers_stream.next() => {
            println!("Movers update: {} gainers", movers.gainers.len());
        }
    }
}
```

### WebSocket Integration

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
    let mut stream = MoversStream::with_defaults(client);
    
    while let Some(Ok(update)) = stream.next().await {
        let json = serde_json::to_string(&update).unwrap();
        if socket.send(json.into()).await.is_err() {
            break;
        }
    }
}
```

## Performance Considerations

### Polling Intervals

- **Minimum recommended**: 5 seconds
- **Default**: 5 seconds  
- **Production**: 10-30 seconds to avoid rate limiting

Shorter intervals increase the risk of hitting Yahoo Finance rate limits.

### Rate Limiting

Yahoo Finance has undocumented rate limits. Best practices:

1. Use reasonable polling intervals (≥5 seconds)
2. Implement exponential backoff on errors
3. Monitor for 429 (Rate Limited) errors
4. Cache data when possible

### Resource Usage

- Each stream creates a background task that polls at the specified interval
- Multiple streams can run concurrently
- Streams automatically clean up when dropped

## Examples

See the `examples/` directory for complete implementations:

- `stream_quotes.rs` - Basic quote streaming
- `stream_indices.rs` - Index streaming
- `stream_movers.rs` - Market movers streaming

Run examples:

```bash
cargo run --example stream_quotes
cargo run --example stream_indices
cargo run --example stream_movers
```

## See Also

- [Quote Streaming](./quote-streaming.md) - Detailed quote streaming documentation
- [Index Streaming](./index-streaming.md) - Detailed index streaming documentation
- [Movers Streaming](./movers-streaming.md) - Detailed movers streaming documentation
- [WebSocket Types](../../websockets/) - WebSocket message formats
- [Error Handling](../error-handling.md) - Handle streaming errors
