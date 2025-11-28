<!-- 757ef308-0d55-42c9-a3a2-63839597327d 6737458d-7ef9-4d9f-ba9a-317d34369e5e -->
# Convert WebSocket Endpoints to GraphQL Subscriptions

## Overview

Convert all existing WebSocket endpoints to GraphQL subscriptions using `async-graphql`'s subscription support. This will eliminate the custom WebSocket implementation (routes, services, connection manager) and provide a unified GraphQL API for both queries and real-time subscriptions.

## Current WebSocket Endpoints to Convert

1. `/ws/profile/{symbol}` - Profile data (quote, similar, sector, news)
2. `/ws/quotes` - Simple quotes for multiple symbols
3. `/ws/indices` - US market indices (DJIA, NASDAQ, S&P 500)
4. `/ws/news` - General market news
5. `/ws/sectors` - Sector performance data
6. `/ws/movers` - Actives, gainers, losers
7. `/ws/hours` - Market status/hours
8. `/ws/moving-average` - Real-time moving averages

## Implementation Steps

### 1. Add GraphQL Subscription Dependencies

**File:** `Cargo.toml`

Add `futures` dependency if not already present (for Stream support):

- Ensure `futures-util = "0.3"` is present (already exists)
- Ensure `tokio-stream = "0.1"` is present (already exists)

### 2. Create GraphQL Subscription Type

**File:** `src/graphql/schema.rs`

- Replace `EmptySubscription` with a new `Subscription` struct
- Implement subscription resolvers for each WebSocket endpoint:
```rust
pub struct Subscription;

#[Subscription]
impl Subscription {
    // Profile subscription - streams quote, similar, sector, news for a symbol
    async fn profile_updates(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> impl Stream<Item = Result<ProfileUpdate>> {
        // Stream implementation using tokio_stream
    }

    // Quotes subscription - streams simple quotes for symbols
    async fn quote_updates(
        &self,
        ctx: &Context<'_>,
        symbols: Vec<String>,
    ) -> impl Stream<Item = Result<SimpleQuote>> {
        // Stream implementation
    }

    // Indices subscription
    async fn indices_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MarketIndex>> {
        // Stream implementation
    }

    // News subscription
    async fn news_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<News>> {
        // Stream implementation
    }

    // Sectors subscription
    async fn sectors_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MarketSector>> {
        // Stream implementation
    }

    // Movers subscription
    async fn movers_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MoversUpdate>> {
        // Stream implementation
    }

    // Market hours subscription
    async fn market_hours_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MarketHours>> {
        // Stream implementation
    }

    // Moving average subscription
    async fn moving_average_updates(
        &self,
        ctx: &Context<'_>,
        symbol: String,
        indicator_type: String, // "sma" or "ema"
        period: i32,
    ) -> impl Stream<Item = Result<MovingAverageUpdate>> {
        // Stream implementation
    }
}
```

- Update schema type: `pub type AppSchema = Schema<Query, EmptyMutation, Subscription>;`

### 3. Create GraphQL Subscription Types

**File:** `src/graphql/types/subscriptions.rs` (new)

Create GraphQL types for subscription responses:

- `ProfileUpdate` - Contains quote, similar, sector, news
- `MoversUpdate` - Contains actives, gainers, losers
- `MarketHours` - Contains status, reason, timestamp
- `MovingAverageUpdate` - Contains symbol, type, period, value, timestamp

### 4. Update GraphQL Types Module

**File:** `src/graphql/types/mod.rs`

- Add `pub mod subscriptions;`
- Re-export subscription types

### 5. Implement Subscription Streams

**File:** `src/graphql/schema.rs`

For each subscription resolver:

- Use `tokio_stream::wrappers::IntervalStream` or `tokio_stream::StreamExt` for periodic updates
- Poll data every 5 seconds (matching current `REFRESH_INTERVAL`)
- Use existing service functions to fetch data
- Convert service results to GraphQL types
- Handle errors gracefully in the stream

Example pattern:

```rust
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use tokio::time::{interval, Duration};

async fn quote_updates(...) -> impl Stream<Item = Result<SimpleQuote>> {
    let mut interval = interval(Duration::from_secs(5));
    let stream = IntervalStream::new(interval)
        .then(move |_| {
            // Fetch quotes using service
            // Return Result<SimpleQuote>
        });
    stream
}
```

### 6. Update GraphQL Handler for WebSocket Support

**File:** `src/graphql/handlers.rs`

- Add WebSocket handler for GraphQL subscriptions
- Use `async-graphql-actix-web`'s WebSocket support
- Configure GraphQL subscription protocol (graphql-ws or graphql-transport-ws)

### 7. Update Main Application

**File:** `src/main.rs`

- Remove `ConnectionManager` actor initialization
- Remove `connection_manager` from `AppState`
- Remove WebSocket route registrations from `routes::configure_routes`
- Add GraphQL WebSocket route: `.route("/graphql-ws", web::get().to(graphql::handlers::graphql_ws_handler))`
- Update schema creation to use `Subscription` instead of `EmptySubscription`

### 8. Remove WebSocket Routes

**Files to Delete:**

- `src/routes/websocket/mod.rs`
- `src/routes/websocket/quotes.rs`
- `src/routes/websocket/profile.rs`
- `src/routes/websocket/indices.rs`
- `src/routes/websocket/news.rs`
- `src/routes/websocket/sectors.rs`
- `src/routes/websocket/movers.rs`
- `src/routes/websocket/hours.rs`
- `src/routes/websocket/moving_average.rs`
- `src/routes/websocket/common.rs`

**File:** `src/routes/mod.rs`

- Remove `pub mod websocket;`
- Remove all `/ws/*` route registrations
- Keep only REST routes (if still needed) or remove entirely if migrating fully to GraphQL

### 9. Remove WebSocket Services

**Files to Delete:**

- `src/service/websocket/mod.rs`
- `src/service/websocket/connection_manager.rs`
- `src/service/websocket/session.rs`
- `src/service/websocket/quotes_session.rs`
- `src/service/websocket/moving_average_session.rs`
- `src/service/websocket/indicator/` (if only used by WebSocket)

**Note:** Keep `src/service/websocket/indicator/` if it contains reusable indicator calculation logic used elsewhere.

### 10. Update Service Module

**File:** `src/service/mod.rs`

- Remove `pub mod websocket;` if present

### 11. Clean Up Imports

**Files to Update:**

- `src/main.rs` - Remove `ConnectionManager` and WebSocket-related imports
- Any other files importing WebSocket modules

### 12. Update Documentation

**File:** `docs/development/GRAPHQL_EXAMPLES.md`

- Add GraphQL subscription examples
- Show how to subscribe to real-time updates
- Include examples for all 8 subscription types

## Key Implementation Details

### Stream Pattern

Each subscription should:

1. Create an interval stream (5-second updates)
2. Fetch data using existing service functions
3. Convert to GraphQL types
4. Yield results or errors
5. Handle client disconnection gracefully

### Error Handling

- Use `Result<T>` in stream items
- Log errors but continue streaming
- Don't terminate stream on single fetch failure

### Performance

- Reuse existing service layer functions
- Maintain 5-second refresh interval
- Consider connection pooling for multiple subscribers

### Testing

- Test subscriptions in GraphQL Playground
- Verify data updates every 5 seconds
- Test multiple simultaneous subscriptions
- Test subscription cancellation

## Files to Create

1. `src/graphql/types/subscriptions.rs` - Subscription-specific GraphQL types

## Files to Modify

1. `Cargo.toml` - Verify dependencies
2. `src/graphql/schema.rs` - Add Subscription type and resolvers
3. `src/graphql/types/mod.rs` - Export subscription types
4. `src/graphql/handlers.rs` - Add WebSocket handler
5. `src/main.rs` - Remove ConnectionManager, update routes
6. `src/routes/mod.rs` - Remove WebSocket routes
7. `docs/development/GRAPHQL_EXAMPLES.md` - Add subscription examples

## Files to Delete

1. `src/routes/websocket/` - Entire directory
2. `src/service/websocket/` - Entire directory (except indicator if reusable)

## Notes

- GraphQL subscriptions use the standard GraphQL subscription protocol over WebSocket
- Clients can use any GraphQL client library that supports subscriptions
- The GraphQL Playground supports testing subscriptions
- All existing service layer functions are reused - no changes needed
- The 5-second refresh interval is maintained for consistency

### To-dos

- [x] Add async-graphql and async-graphql-actix-web dependencies to Cargo.toml
- [x] Create src/graphql/ directory structure with mod.rs, schema.rs, handlers.rs, and types/ subdirectory
- [x] Create GraphQL types for Quote, SimpleQuote, DetailedQuote in src/graphql/types/quote.rs
- [x] Create GraphQL types for HistoricalData, HistoricalResponse, TimeRange, Interval, IndicatorType enums in src/graphql/types/historical.rs
- [x] Create GraphQL types for all remaining models (news, search, financials, earnings, movers, indices, holders, analysts, sectors, similar, health)
- [x] Create AppContext struct in schema.rs that wraps AppState for GraphQL resolvers
- [x] Implement GraphQL resolvers for quotes, simpleQuotes, detailedQuotes, and similar endpoints
- [x] Implement GraphQL resolver for historical endpoint with TimeRange/Interval validation and indicator support
- [x] Implement GraphQL resolvers for all remaining endpoints (search, news, financials, earnings, movers, indices, holders, analysts, sectors, health)
- [x] Create GraphQL route handlers (graphql_handler and graphql_playground_handler) in src/graphql/handlers.rs
- [x] Update src/main.rs to create GraphQL schema, register GraphQL routes, and pass AppState context
- [x] Ensure all resolvers properly convert service errors to GraphQL errors with appropriate error messages
- [ ] Add async-graphql and async-graphql-actix-web dependencies to Cargo.toml
- [ ] Create src/graphql/ directory structure with mod.rs, schema.rs, handlers.rs, and types/ subdirectory
- [ ] Create GraphQL types for Quote, SimpleQuote, DetailedQuote in src/graphql/types/quote.rs
- [ ] Create GraphQL types for HistoricalData, HistoricalResponse, TimeRange, Interval, IndicatorType enums in src/graphql/types/historical.rs
- [ ] Create GraphQL types for all remaining models (news, search, financials, earnings, movers, indices, holders, analysts, sectors, similar, health)