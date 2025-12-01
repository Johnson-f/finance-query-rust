# Movers Model

## Overview

The movers model provides data structures for market movers - stocks with significant activity or price movements. This includes most active stocks by volume, top gainers, and top losers.

## Models

### MarketMover

Represents a single stock that is moving significantly in the market.

```rust
pub struct MarketMover {
    pub symbol: String,
    pub name: String,
    pub price: String,
    pub change: String,
    pub percent_change: String,
}
```

#### Fields

- `symbol` - Stock ticker symbol (e.g., "AAPL", "TSLA")
- `name` - Company name (e.g., "Apple Inc.")
- `price` - Current market price formatted as string (e.g., "145.00")
- `change` - Absolute price change with sign (e.g., "+1.50", "-2.30")
- `percent_change` - Percentage change with sign (e.g., "+1.05%", "-1.58%")

### MoverCount

Enum to specify how many movers to fetch per category.

```rust
pub enum MoverCount {
    TwentyFive,  // Top 25 movers
    Fifty,       // Top 50 movers (default)
    Hundred,     // Top 100 movers
}
```

#### Methods

```rust
impl MoverCount {
    pub fn as_str(&self) -> &'static str;
    pub fn from_str(s: &str) -> Option<Self>;
}
```

## Usage

### Fetching Movers

Use the `YahooFinanceClient` to fetch market movers:

```rust
use finance_query_core::{YahooFinanceClient, MoverCount};

let client = /* ... initialize client ... */;

// Fetch top 50 movers in each category
let (actives, gainers, losers) = client.get_movers(MoverCount::Fifty).await?;

// Display top gainers
println!("Top Gainers:");
for mover in gainers.iter().take(10) {
    println!(
        "{} ({}): {} {}",
        mover.symbol,
        mover.name,
        mover.price,
        mover.percent_change
    );
}
```

### Streaming Movers

For real-time updates, use the `MoversStream`:

```rust
use finance_query_core::{MoversStream, MoverCount};
use futures_util::StreamExt;
use std::time::Duration;

let mut stream = MoversStream::new(
    client,
    MoverCount::Fifty,
    Duration::from_secs(5)
);

while let Some(Ok(update)) = stream.next().await {
    println!("Actives: {}", update.actives.len());
    println!("Gainers: {}", update.gainers.len());
    println!("Losers: {}", update.losers.len());
}
```

## Categories

### Most Active

Stocks with the highest trading volume. These are the most traded stocks by number of shares.

```rust
let (actives, _, _) = client.get_movers(MoverCount::Fifty).await?;

for stock in actives {
    println!("{}: {} shares traded", stock.symbol, stock.name);
}
```

### Top Gainers

Stocks with the largest percentage price increases.

```rust
let (_, gainers, _) = client.get_movers(MoverCount::Fifty).await?;

for stock in gainers {
    println!("{}: up {}", stock.symbol, stock.percent_change);
}
```

### Top Losers

Stocks with the largest percentage price decreases.

```rust
let (_, _, losers) = client.get_movers(MoverCount::Fifty).await?;

for stock in losers {
    println!("{}: down {}", stock.symbol, stock.percent_change);
}
```

## Filtering

The movers data is automatically filtered to include only US stocks:
- Symbols without dots (e.g., "AAPL", "TSLA")
- Symbols with US exchange suffixes (e.g., ".OB", ".PK")

This excludes international stocks and ADRs with country-specific suffixes.

## Examples

### Find Extreme Movers

```rust
let (_, gainers, losers) = client.get_movers(MoverCount::Hundred).await?;

// Find stocks up more than 10%
let extreme_gainers: Vec<_> = gainers
    .iter()
    .filter(|m| {
        m.percent_change
            .trim_start_matches('+')
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0) > 10.0
    })
    .collect();

println!("Stocks up more than 10%:");
for stock in extreme_gainers {
    println!("  {}: {}", stock.symbol, stock.percent_change);
}
```

### Compare Price Ranges

```rust
let (actives, _, _) = client.get_movers(MoverCount::Fifty).await?;

// Categorize by price
let under_10: Vec<_> = actives.iter()
    .filter(|m| m.price.parse::<f64>().unwrap_or(0.0) < 10.0)
    .collect();

let over_100: Vec<_> = actives.iter()
    .filter(|m| m.price.parse::<f64>().unwrap_or(0.0) > 100.0)
    .collect();

println!("Active stocks under $10: {}", under_10.len());
println!("Active stocks over $100: {}", over_100.len());
```

### Track Watchlist

```rust
let watchlist = vec!["AAPL", "TSLA", "NVDA", "AMD"];
let (actives, gainers, losers) = client.get_movers(MoverCount::Hundred).await?;

// Combine all movers
let all_movers: Vec<_> = actives
    .iter()
    .chain(gainers.iter())
    .chain(losers.iter())
    .collect();

// Check if watchlist stocks are moving
for symbol in watchlist {
    if let Some(mover) = all_movers.iter().find(|m| m.symbol == symbol) {
        println!(
            "{} is a mover today: {} ({})",
            symbol,
            mover.percent_change,
            mover.price
        );
    }
}
```

## Serialization

The `MarketMover` struct implements `Serialize` and `Deserialize` for easy JSON conversion:

```rust
use serde_json;

let mover = MarketMover {
    symbol: "AAPL".to_string(),
    name: "Apple Inc.".to_string(),
    price: "145.00".to_string(),
    change: "+1.50".to_string(),
    percent_change: "+1.04%".to_string(),
};

// Serialize to JSON
let json = serde_json::to_string(&mover)?;
println!("{}", json);
// Output: {"symbol":"AAPL","name":"Apple Inc.","price":"145.00","change":"+1.50","percentChange":"+1.04%"}

// Deserialize from JSON
let parsed: MarketMover = serde_json::from_str(&json)?;
```

Note: The `percent_change` field is serialized as `percentChange` in JSON (camelCase).

## See Also

- [MoversStream](../streaming/movers-streaming.md) - Real-time movers streaming
- [Quote Model](./quote.md) - Detailed quote information
- [Market Model](./market.md) - Market status and summary
- [WebSocket Movers](../../websockets/movers.md) - WebSocket endpoint for movers
