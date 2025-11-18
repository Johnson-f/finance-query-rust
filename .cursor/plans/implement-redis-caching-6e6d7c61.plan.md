<!-- 6e6d7c61-3935-4454-a0c6-148b13780588 8c8d6666-1db9-4bd0-ad16-47838ceb32fa -->
# Implement Redis Caching for Finance Query API

## Overview

Add Redis caching layer to reduce external API calls and improve response times for slow-changing financial data. Cache eligible endpoints with endpoint-specific TTLs, while excluding fast-changing price data.

## Implementation Plan

### 1. Add Redis Dependency

- Add `redis` crate to `Cargo.toml` with async features
- Add `tokio-compat-02` or use `redis::aio::ConnectionManager` for async support

### 2. Create Cache Service Module

Create `src/service/caching/mod.rs` with:

- `CacheService` struct wrapping Redis connection
- `get()` method to retrieve cached data
- `set()` method to store data with TTL
- `CacheKey` type for structured key generation
- Helper functions for key generation per endpoint type

### 3. Update AppState

Modify `src/main.rs`:

- Add `cache_service: Arc<CacheService>` to `AppState`
- Initialize Redis connection from `REDIS_URL` environment variable (default: `redis://127.0.0.1:6379`)
- Handle Redis connection errors gracefully (log warning, continue without cache)

### 4. Implement Cache Key Strategy

Create cache keys with query parameters:

- `earnings_transcript:{symbol}:calls` or `earnings_transcript:{symbol}:transcript`
- `financials:{symbol}:{statement_type}:{frequency}`
- `holders:{symbol}:{type}` (major, institutional, mutualfund, insider-transactions, insider-purchases, insider-roster)
- `news:{symbol}` or `news:general` (if no symbol)
- `analysts:{symbol}:{analysis_type}` (recommendations, upgrades-downgrades, price-targets, earnings-estimate, revenue-estimate, earnings-history)

### 5. Update Route Handlers

#### Earnings Transcript (`src/routes/earnings_transcript.rs`)

- Check cache before calling service
- Cache response with 90 days TTL (7,776,000 seconds)
- Key format: `earnings_transcript:{symbol}:{type}`

#### Financials (`src/routes/financials.rs`)

- Check cache before calling service
- Cache response with 90 days TTL
- Key format: `financials:{symbol}:{statement}:{frequency}`

#### Holders (`src/routes/holders.rs`)

- Check cache before calling service
- Cache response with 1 day TTL (86,400 seconds)
- Key format: `holders:{symbol}:{holder_type}`

#### News (`src/routes/news.rs`)

- Check cache before calling service
- Cache response with 6 hours TTL (21,600 seconds)
- Key format: `news:{symbol}` or `news:general`

#### Analysts (`src/routes/analysts.rs`)

- Check cache before calling service
- Cache response with 1 week TTL (604,800 seconds)
- Key format: `analysts:{symbol}:{analysis_type}`

### 6. Ensure Non-Cached Endpoints

Verify these endpoints are NOT cached:

- `/v1/quotes`, `/v1/simple-quotes`, `/v1/detailed-quotes` (quotes.rs)
- `/v1/historical/{symbol}` (historical.rs)
- `/v1/actives`, `/v1/gainers`, `/v1/losers` (movers.rs)
- `/v1/sectors/*` (sectors.rs)
- `/v1/indices` (indices.rs)

### 7. Error Handling

- On cache miss: Fetch from external API, cache result, return response
- On Redis connection error: Log warning, bypass cache, fetch from API
- On cache deserialization error: Log error, fetch fresh from API
- Never serve stale data on errors

### 8. Testing Considerations

- Test cache hit/miss scenarios
- Test TTL expiration
- Test Redis connection failures (graceful degradation)
- Test cache key generation with various query parameters

## Files to Modify

1. `Cargo.toml` - Add redis dependency
2. `src/main.rs` - Add CacheService to AppState
3. `src/service/caching/mod.rs` - Implement CacheService
4. `src/routes/earnings_transcript.rs` - Add caching (90 days TTL)
5. `src/routes/financials.rs` - Add caching (90 days TTL)
6. `src/routes/holders.rs` - Add caching (1 day TTL)
7. `src/routes/news.rs` - Add caching (6 hours TTL)
8. `src/routes/analysts.rs` - Add caching (1 week TTL)

## Environment Variables

- `REDIS_URL` (optional): Redis connection string (default: `redis://127.0.0.1:6379`)

## TTL Constants

- Earnings Transcript: 90 days (7,776,000 seconds)
- Financials: 90 days (7,776,000 seconds)
- Holders: 1 day (86,400 seconds)
- News: 6 hours (21,600 seconds)
- Analysts: 1 week (604,800 seconds)

### To-dos

- [ ] Add redis crate to Cargo.toml with async features
- [ ] Create CacheService in src/service/caching/mod.rs with get/set methods and key generation
- [ ] Add CacheService to AppState in src/main.rs with Redis connection initialization
- [ ] Add caching to earnings_transcript routes with 90 days TTL
- [ ] Add caching to financials routes with 90 days TTL
- [ ] Add caching to holders routes with 1 day TTL
- [ ] Add caching to news routes with 6 hours TTL
- [ ] Add caching to analysts routes with 1 week TTL