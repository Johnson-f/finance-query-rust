<!-- bcd050c6-42dc-4422-a03e-880e4fc454fe 1e84e809-48d5-4188-b25b-299f658b7e23 -->
# Implement Full Yahoo Finance Web Server in Rust with Actix-web

## Overview

Replicate the Python FinanceQuery architecture in Rust: full web server with client layer, service layer (API + HTML scraping fallback), routes, and models. Uses Actix-web web framework with automatic authentication refresh and dual-fetching strategy.

## Implementation Steps

### 1. Setup Dependencies (`Cargo.toml`)

Add required dependencies:

- `actix-web` (with `macros` feature) - Web framework
- `tokio` (with `full` feature) - Async runtime
- `reqwest` (with `cookies`, `json` features) - HTTP client
- `serde` + `serde_json` - JSON serialization
- `scraper` - HTML parsing for scraping fallback
- `regex` - For parsing HTML/URLs (earnings calls, XPath-like selectors)
- `chrono` - Time handling
- `anyhow` + `thiserror` - Error handling
- `actix-cors` - CORS middleware
- `tracing` + `tracing-actix-web` - Logging
- `once_cell` - Lazy static initialization

### 2. Error Types (`client/error.rs`)

Create `YahooError` enum:

- `AuthFailed`, `NotFound`, `RateLimited`, `HttpError`, `ParseError`, `NetworkError`
- Implement `std::error::Error` trait
- Implement `actix_web::ResponseError` trait to convert to HTTP responses
- Map error variants to appropriate HTTP status codes

### 3. Base HTTP Client (`client/fetch_client.rs`)

Implement `FetchClient`:

- Wraps `reqwest::Client` with Chrome-like headers
- Default headers matching Python version
- Proxy support (optional)
- Cookie jar via `reqwest::cookie::Jar`
- Methods: `request()`, `fetch()` (async)
- HTML fetching for scraping fallback

### 4. Yahoo Authentication (`client/yahoo_auth.rs`)

Implement `YahooAuthManager`:

- `Arc<Mutex<AuthState>>` with crumb, cookie jar, last_update
- `refresh()`: Same flow as Python (fc.yahoo.com → getcrumb → CSRF fallback if needed)
- `get_or_refresh()`: 30-second refresh interval with mutex lock
- Automatic refresh on app startup (similar to Python lifespan)

### 5. Yahoo Finance API Client (`client/yahoo_client.rs`)

Implement `YahooFinanceClient`:

- All API methods from Python version:
  - `get_quote()`, `get_simple_quotes()`, `get_chart()`, `search()`
  - `get_similar_quotes()`, `get_fundamentals_timeseries()`, `get_quote_summary()`
  - `get_quote_type()`, `get_earnings_calls_list()`, `get_earnings_transcript()`
- `_yahoo_request()` helper with auth injection
- `_json()` helper for JSON parsing

### 6. HTML Scraper (`client/scraper.rs`)

Implement scraping fallback:

- `scrape_quote(symbol: &str)` - Scrapes `https://finance.yahoo.com/quote/{symbol}/`
- Uses `scraper` crate with CSS selectors (equivalent to XPath in Python)
- Extracts: price data, general info, company info, performance metrics
- `scrape_simple_quote(symbol: &str)` - Simplified version
- `scrape_earnings_calls_list(symbol: &str)` - HTML parsing for earnings calls page
- Parse HTML in async context (similar to Python's thread pool)

### 7. Service Layer (`service/`)

Implement dual-fetching services:

#### `service/quotes.rs`

- `get_quotes(symbols: &[&str])` - Try API first, fallback to scraping
- `get_simple_quotes(symbols: &[&str])` - Same pattern
- Retry logic: API call → if fails → scrape
- Parallel processing for multiple symbols

#### `service/historical.rs`

- `get_historical(symbol, time_range, interval)` - Uses API only (chart endpoint)

#### `service/search.rs`

- `search(query: &str)` - Uses API only

#### `service/financials.rs`

- `get_financial_statement(symbol, statement_type, frequency)` - Uses API

#### `service/news.rs`

- `scrape_news_for_quote(symbol: &str)` - Scrapes StockAnalysis.com (like Python)
- `scrape_general_news()` - General news scraping

#### `service/earnings.rs`

- `get_earnings_calls_list(symbol)` - API with scraping fallback
- `get_earnings_transcript(symbol, event_id)` - API call

### 8. Models (`models/`)

Create request/response models with `serde`:

#### `models/quote.rs`

- `Quote`, `SimpleQuote` structs
- Match Python Pydantic models structure
- `#[serde(rename_all = "snake_case")]` for JSON

#### `models/historical.rs`

- `HistoricalData`, `TimeRange`, `Interval` enums
- `HistoricalResponse` with timestamped data

#### `models/news.rs`

- `News` struct with title, link, source, img, time

#### `models/search.rs`

- `SearchResult`, `SearchResponse`

#### `models/financials.rs`

- `FinancialStatement`, `StatementType`, `Frequency` enums

### 9. Routes (`routes/`)

Implement Actix-web handlers:

#### `routes/quotes.rs`

- `GET /v1/quotes?symbols=NVDA,AAPL` - Full quotes
- `GET /v1/quotes/simple?symbols=NVDA,AAPL` - Simple quotes
- Use `web::Query` for query params, `web::Data<AppState>` for shared state
- Call service layer, return `HttpResponse` with JSON

#### `routes/historical.rs`

- `GET /v1/historical/{symbol}?range=1y&interval=1d` - Historical prices
- Use `web::Path` for path params, `web::Query` for query params

#### `routes/search.rs`

- `GET /v1/search?q=NVDA` - Symbol search
- Use `web::Query` for query parameters

#### `routes/news.rs`

- `GET /v1/news/{symbol}` - News for symbol
- `GET /v1/news` - General news
- Use `web::Path` for symbol, `web::Data<AppState>` for state

#### `routes/financials.rs`

- `GET /v1/financials/{symbol}?statement=income&frequency=annual` - Financial statements
- Use `web::Path` and `web::Query` for params

#### `routes/earnings.rs`

- `GET /v1/earnings/{symbol}/calls` - Earnings calls list
- `GET /v1/earnings/{symbol}/transcript/{event_id}` - Transcript
- Use `web::Path` for path parameters

#### `routes/health.rs`

- `GET /ping` - Simple health check
- `GET /health` - Comprehensive health check (test all services)
- Return JSON responses

### 10. Application State (`main.rs`)

Set up Actix-web app:

- Shared state: `AppState` struct with `YahooAuthManager`, `FetchClient`, optional Redis
- Wrap in `web::Data<AppState>` for handler access
- Initialize auth manager on startup (prime authentication)
- Register all route handlers using `App::new().service()`
- Add middleware: CORS via `actix-cors`, request logging via `tracing-actix-web`
- Server startup with `HttpServer::new()` and `.bind().await?.run().await`

### 11. Module Structure

```
src/
├── client/
│   ├── mod.rs
│   ├── error.rs
│   ├── fetch_client.rs
│   ├── yahoo_auth.rs
│   ├── yahoo_client.rs
│   └── scraper.rs          # HTML scraping
├── service/
│   ├── mod.rs
│   ├── quotes.rs           # Dual-fetching service
│   ├── historical.rs
│   ├── search.rs
│   ├── financials.rs
│   ├── news.rs
│   └── earnings.rs
├── models/
│   ├── mod.rs
│   ├── quote.rs
│   ├── historical.rs
│   ├── news.rs
│   ├── search.rs
│   └── financials.rs
├── routes/
│   ├── mod.rs
│   ├── quotes.rs
│   ├── historical.rs
│   ├── search.rs
│   ├── news.rs
│   ├── financials.rs
│   ├── earnings.rs
│   └── health.rs
└── main.rs                  # App setup & server
```

## Key Implementation Details

### Dual-Fetching Pattern

Service layer implements:

```rust
async fn get_quotes(symbols: &[&str]) -> Result<Vec<Quote>> {
    // Try API first
    match yahoo_client.get_simple_quotes(symbols).await {
        Ok(data) => parse_quotes(data),
        Err(_) => {
            // Fallback to scraping
            scrape_quotes(symbols).await
        }
    }
}
```

### Authentication Lifecycle

- Auth manager initialized in `main()` before server starts
- Prime authentication on startup (like Python lifespan)
- Automatic refresh every 30 seconds via background task or on-demand
- Shared `Arc<YahooAuthManager>` in app state

### HTML Scraping

- Use `scraper` crate with CSS selectors
- Parse HTML asynchronously (no blocking)
- Extract data using selectors matching Python XPath patterns
- Handle parsing errors gracefully

### Error Handling

- Service layer returns `Result<T, YahooError>`
- Routes use `?` operator to propagate errors
- Implement `ResponseError` trait for `YahooError` to convert to HTTP responses
- Use `thiserror` for error chaining

### Actix-web Specific Patterns

- Use `#[actix_web::get]` and `#[actix_web::post]` macros for route handlers
- Extract path params with `web::Path<T>`
- Extract query params with `web::Query<T>`
- Access app state with `web::Data<AppState>`
- Return `HttpResponse` with `.json()` for JSON responses
- Use `App::new().app_data(web::Data::new(app_state))` for shared state

### Caching (Optional)

- Consider in-memory cache with TTL (like Python's `@cache`)
- Use `moka` or `cached` crate
- Market-aware expiration (shorter when market open)

## Testing Strategy

- Unit tests for client methods (mock HTTP responses)
- Integration tests for service layer
- Test dual-fetching fallback behavior
- Test authentication refresh logic

### To-dos

- [ ] Add required dependencies to Cargo.toml (reqwest, tokio, serde, regex, chrono, anyhow, once_cell)
- [ ] Create YahooError enum in client/error.rs with variants for different error types
- [ ] Implement FetchClient in client/fetch_client.rs with Chrome-like headers and cookie support
- [ ] Implement YahooAuthManager in client/yahoo_auth.rs with refresh logic and CSRF fallback
- [ ] Implement YahooFinanceClient in client/yahoo_client.rs with all API methods
- [ ] Set up module exports in client/mod.rs to expose public API
- [ ] Add example usage in main.rs demonstrating basic client operations