# FinanceQuery Rust API Architecture

## Overview

FinanceQuery Rust is an Actix-Web-based financial data API that aggregates data from Yahoo Finance API and web scraping. The architecture follows a service-oriented design with clear separation of concerns, emphasizing modularity, asynchronous operations, and resilience. Built with Rust's type system and async runtime (Tokio), it provides high performance and memory safety.

## Core Components

### Actix-Web Application (`src/main.rs`)

The main application initializes and configures:

- **Application State**: Shared resources stored in `AppState` struct:
  - `yahoo_auth_manager`: Yahoo Finance authentication
  - `fetch_client`: General-purpose HTTP client
  - `yahoo_client`: Yahoo Finance API client
  - `connection_manager`: WebSocket connection manager (Actix actor)
  - `cache_service`: Redis-based caching
  - `rate_limit_manager`: Rate limiting service

- **Middleware Stack**: Applied in order:
  1. CORS middleware (allows cross-origin requests)
  2. TracingLogger (request/response logging)
  3. RateLimitMiddleware (IP-based rate limiting)

- **Error Handling**: Custom `YahooError` enum with `ResponseError` implementation
- **Routing**: API endpoints organized under `/v1` scope
- **Health Checks**: `/ping` (basic) and `/health` (comprehensive service validation)
- **WebSocket Support**: Real-time data streaming via `actix-ws`

### Data Models (`src/models/`)

Rust structs with Serde provide:

- **API Contract Definition**: Consistent request/response structures
- **Input Validation**: Type-safe parameter handling via Actix-Web extractors
- **Serialization**: JSON serialization/deserialization with `serde_json`
- **Type Safety**: Compile-time type checking prevents runtime errors

Key model modules:
- `quote.rs` - Quote and SimpleQuote structures
- `historical.rs` - Historical price data
- `indices.rs` - Market index data
- `sectors.rs` - Sector information
- `movers.rs` - Market movers data
- `news.rs` - Financial news articles
- `search.rs` - Search results
- `financials.rs` - Financial statements
- `holders.rs` - Ownership data structures
- `analysts.rs` - Analyst recommendations and estimates
- `earnings_transcripts.rs` - Earnings call transcripts

### Routing System (`src/routes/mod.rs`)

Routes are configured in `configure_routes()` function:

**REST API Endpoints** (under `/v1`):
- `/v1/quotes` - Detailed stock quotes
- `/v1/simple-quotes` - Simplified quotes
- `/v1/detailed-quotes` - Extended quote information
- `/v1/similar` - Similar securities lookup
- `/v1/historical/{symbol}` - Historical price data
- `/v1/search` - Symbol search
- `/v1/news` - Financial news
- `/v1/financials/{symbol}` - Financial statements
- `/v1/earnings/{symbol}/calls` - Earnings calls
- `/v1/earnings/{symbol}/transcript` - Earnings transcripts
- `/v1/actives` - Most active stocks
- `/v1/gainers` - Top gainers
- `/v1/losers` - Top losers
- `/v1/indices` - Market indices
- `/v1/holders/{symbol}/*` - Various holder endpoints (major, institutional, mutualfund, insider-*)
- `/v1/analysis/{symbol}/*` - Analyst data (recommendations, upgrades-downgrades, price-targets, estimates)
- `/v1/sectors` - Sector performance
- `/v1/sectors/symbol/{symbol}` - Sector for symbol
- `/v1/sectors/details/{sector}` - Sector details

**WebSocket Endpoints** (under `/v1/ws`):
- `/v1/ws/quotes` - Real-time quotes
- `/v1/ws/profile/{symbol}` - Real-time profile updates
- `/v1/ws/indices` - Real-time indices
- `/v1/ws/news` - Real-time news
- `/v1/ws/sectors` - Real-time sectors
- `/v1/ws/movers` - Real-time movers
- `/v1/ws/hours` - Market hours updates

**Health Endpoints**:
- `/ping` - Basic health check
- `/health` - Comprehensive health check

## Logging & Middleware Architecture

### Logging System

Comprehensive logging using the `tracing` ecosystem:

- **Environment Configuration**:
  - `RUST_LOG`: Controls log level (trace/debug/info/warn/error, default: info)
  - Uses `tracing-subscriber` for log formatting
  - Initialized in `main()` with `tracing_subscriber::fmt::init()`

- **Tracing Integration**: Uses `tracing` crate for structured logging throughout the codebase
- **Request Logging**: `TracingLogger` middleware logs all HTTP requests/responses
- **Structured Logging**: Logs include contextual information (request paths, status codes, timing)
- **Performance Monitoring**: Automatic request timing via middleware

### Middleware Stack

Middleware is applied in order using Actix-Web's `.wrap()` method:

#### CORS Middleware (`actix-cors`)

- **Configuration**: Allows any origin, method, and header
- **Max Age**: 3600 seconds for preflight cache
- **Development-Friendly**: Permissive CORS for development and API access

#### TracingLogger (`tracing-actix-web`)

- **Request/Response Logging**: Logs all HTTP requests and responses
- **Timing Information**: Includes request duration in logs
- **Error Logging**: Captures and logs errors with context
- **Default Configuration**: Uses default settings from `TracingLogger::default()`

#### RateLimitMiddleware (`src/middleware/rate_limit.rs`)

Implements Actix-Web's `Transform` trait to wrap services:

- **IP-Based Limiting**: Configurable daily request limits per client IP (default: 10,000/day)
- **Admin Key Bypass**: `ADMIN_API_KEY` header bypasses all rate limits
- **Open Paths**: `/ping` and `/health` endpoints exempt from limits
- **Redis Integration**: Uses Redis for distributed rate limiting (graceful degradation if unavailable)
- **Response Headers**: Rate limit status in `X-RateLimit-*` headers
- **Implementation**: Uses `Rc<S>` to wrap services (no Clone requirement)
- **TTL**: 24-hour (86,400 seconds) rate limit window

## Service Layer Architecture

### Service Organization (`src/service/`)

Services implement business logic with async functions:

```
src/service/
├── quotes.rs              # Quote fetching and parsing
├── historical.rs          # Historical data retrieval
├── indices.rs             # Market indices data
├── sectors.rs             # Sector performance data
├── movers.rs              # Market movers (gainers, losers, actives)
├── news.rs                 # Financial news aggregation
├── search.rs               # Symbol search functionality
├── financials.rs           # Financial statements (income, balance, cashflow)
├── holders.rs              # Ownership data (institutional, insider, etc.)
├── analysts.rs             # Analyst recommendations and estimates
├── earnings_transcript.rs   # Earnings call transcripts
├── earnings_calendar.rs    # Earnings calendar data
├── logo.rs                 # Logo fetching from external services
├── market.rs               # Market utilities and helpers
├── caching/
│   └── mod.rs              # Redis cache service
└── websocket/
    ├── connection_manager.rs  # Actix actor for WebSocket management
    ├── session.rs             # WebSocket session handling
    └── quotes_session.rs      # Quote-specific WebSocket sessions
```

### Service Pattern

Services follow a consistent async pattern:
- Accept `AppState` or specific clients via Actix-Web extractors
- Use `YahooFinanceClient` for API calls
- Use `FetchClient` for web scraping fallback
- Return `Result<T, YahooError>` for error handling
- Integrate with `CacheService` for caching

### Multi-Source Strategy

Services implement dual-fetching for resilience:

1. **Primary Source** (`YahooFinanceClient`): Yahoo Finance API calls
2. **Fallback Source** (`FetchClient` + scraper): Web scraping when API fails

Services handle errors gracefully and fall back to scraping when API calls fail.

## Client Architecture

### HTTP Client Architecture

#### FetchClient (`src/client/fetch_client.rs`)

General-purpose async HTTP client using `reqwest`:

- **reqwest Client**: Async HTTP client with cookie store enabled
- **Proxy Support**: Optional proxy configuration via `PROXY_URL` environment variable
- **Cookie Management**: Uses `Arc<Jar>` for shared cookie storage
- **Timeout Configuration**: Default 10-second timeout for requests
- **User Agent**: Browser-like user agent for web scraping
- **Public API**: 
  - `new(proxy: Option<String>) -> Result<Self, YahooError>`
  - `client() -> &Client` - Access to underlying reqwest client
  - `cookie_jar() -> &Arc<Jar>` - Access to cookie jar
  - `fetch(url: &str) -> Result<String, YahooError>` - Simple fetch
  - `fetch_with_timeout(url: &str, timeout: Duration)` - Fetch with custom timeout
  - `fetch_response(url: &str) -> Result<Response, YahooError>` - Get raw response

#### YahooFinanceClient (`src/client/yahoo_client.rs`)

Specialized Yahoo Finance API client:

- **Dependencies**: Requires `YahooAuthManager` and `FetchClient`
- **Auto-Authentication**: Uses auth manager to inject cookies/crumb automatically
- **Error Handling**: Converts HTTP status codes to `YahooError` variants
- **API Methods**: 
  - `get_quote(symbol: &str)` - Quote summary data
  - `get_simple_quotes(symbols: &[&str])` - Simple quote data
  - `get_chart(symbol, interval, range)` - Chart data
  - `search(query, hits)` - Symbol search
  - `get_similar_quotes(symbol, limit)` - Similar securities
  - `get_quote_summary(symbol, modules)` - Detailed quote summary
  - `get_quote_type(symbol)` - Quote type information
  - `make_request(url, params)` - Generic API request
- **Retry Logic**: Handles 401 errors by refreshing authentication and retrying

#### YahooAuthManager (`src/client/yahoo_auth.rs`)

Manages Yahoo Finance API authentication:

- **Cookie/Crumb Management**: Handles CSRF tokens (`crumb`) and session cookies
- **Shared Cookie Jar**: Uses `Arc<Jar>` from `FetchClient` for consistency
- **Automatic Refresh**: `refresh()` method fetches new crumb when needed
- **Thread Safety**: Uses async locks to prevent concurrent refresh attempts
- **Priming**: Authentication is primed on application startup
- **Public API**:
  - `new(proxy: Option<String>, cookie_jar: Arc<Jar>) -> Self`
  - `refresh() -> Result<(Arc<Jar>, String), YahooError>` - Get or refresh crumb
  - `get_or_refresh() -> Result<(Arc<Jar>, String), YahooError>` - Cached refresh

## Data Persistence & Caching

### Caching Strategy (`src/service/caching/mod.rs`)

Redis-based caching with graceful degradation:

- **Redis Integration**: Uses `redis::aio::ConnectionManager` for async operations
- **Connection Management**: Optional Redis connection (None if Redis unavailable)
- **Graceful Degradation**: Returns `None` on cache misses or Redis errors (no in-memory fallback)
- **Serialization**: Uses `serde_json` for value serialization/deserialization
- **TTL Management**: Configurable time-to-live per cache operation

### CacheService API

```rust
pub struct CacheService {
    connection: Option<Arc<ConnectionManager>>,
}

impl CacheService {
    pub async fn new(redis_url: Option<String>) -> Self
    pub async fn get<T>(&self, key: &str) -> Option<T> where T: Deserialize
    pub async fn set<T>(&self, key: &str, value: &T, ttl_seconds: u64) where T: Serialize
}
```

### Cache Key Helpers

Predefined cache key generation functions:
- `earnings_transcript_key(symbol, transcript_type)` - TTL: 90 days
- `financials_key(symbol, statement, frequency)` - TTL: 90 days
- `holders_key(symbol, holder_type)` - TTL: 1 day
- `news_key(symbol: Option<&str>)` - TTL: 6 hours
- `analysts_key(symbol, analysis_type)` - TTL: 1 week

### Cache TTL Constants

- `TTL_EARNINGS_TRANSCRIPT`: 7,776,000 seconds (90 days)
- `TTL_FINANCIALS`: 7,776,000 seconds (90 days)
- `TTL_HOLDERS`: 86,400 seconds (1 day)
- `TTL_NEWS`: 21,600 seconds (6 hours)
- `TTL_ANALYSTS`: 604,800 seconds (1 week)

## Real-Time Data Architecture

### WebSocket Connection Management

#### ConnectionManager (`src/service/websocket/connection_manager.rs`)

Actix actor for managing WebSocket connections:

- **Actor-Based**: Implements `Actor` trait for concurrent message handling
- **Channel-Based**: Groups connections by channel name (e.g., symbol, topic)
- **Session Tracking**: Maps channel names to `SessionEntry` structs containing:
  - Unique session ID
  - Unbounded sender channel (`mpsc::UnboundedSender<Value>`)
- **Task Management**: Background tasks per channel for data fetching
- **Message Types**:
  - `Connect` - Register a new WebSocket session
  - `Disconnect` - Remove a WebSocket session
  - `BroadcastMessage` - Send message to all sessions in a channel
  - `StartTask` - Start background task for a channel
- **Auto-Cleanup**: Removes sessions and cancels tasks when channels become empty

#### WebSocket Sessions (`src/service/websocket/session.rs`)

Individual WebSocket session handling:

- **Session Function**: `handle_websocket_session()` manages individual connections
- **Heartbeat**: 30-second ping interval to keep connections alive
- **Client Timeout**: 60-second timeout for inactive clients
- **Message Channels**: Uses `mpsc::UnboundedChannel` for receiving broadcast messages
- **Message Handling**: Processes incoming WebSocket messages
- **Error Recovery**: Handles connection errors and cleanup gracefully
- **Concurrent Tasks**: Spawns separate tasks for sending and receiving

#### WebSocket Route Handlers (`src/routes/websocket/`)

Route-specific WebSocket handlers:
- `quotes.rs` - Real-time quote updates
- `profile.rs` - Profile updates for specific symbols
- `indices.rs` - Market indices updates
- `news.rs` - News feed updates
- `sectors.rs` - Sector performance updates
- `movers.rs` - Market movers updates
- `hours.rs` - Market hours status updates

## Security & Rate Limiting

### RateLimitManager (`src/middleware/rate_limit.rs`)

Centralized rate limiting service:

- **Configuration**: 
  - `RATE_LIMIT_PER_DAY` environment variable (default: 10,000 requests/day)
  - `REDIS_URL` for distributed rate limiting
- **Redis Integration**: Uses `redis::aio::ConnectionManager` for async operations
- **Graceful Degradation**: Allows all requests if Redis unavailable (no in-memory fallback)
- **IP-Based Tracking**: Tracks requests per IP address using Redis keys (`rate_limit:{ip}`)
- **TTL**: 24-hour (86,400 seconds) rate limit window
- **Public API**:
  - `new(redis_url: Option<String>, limit_per_day: Option<u64>) -> Self`
  - `check_and_increment(ip: &str) -> Result<RateLimitResult, RateLimitError>`
  - `limit_per_day() -> u64` - Get current limit

### RateLimitMiddleware (`src/middleware/rate_limit.rs`)

Actix-Web middleware implementing `Transform` trait:

- **Service Wrapping**: Wraps services using `Rc<S>` (no Clone requirement)
- **IP Extraction**: Extracts client IP from request headers or peer address
- **Admin Key Bypass**: Checks `x-api-key` header against `ADMIN_API_KEY` environment variable
- **Open Paths**: `/ping` and `/health` endpoints bypass rate limiting
- **Response Headers**: Adds `X-RateLimit-*` headers:
  - `X-RateLimit-Limit`: Daily limit
  - `X-RateLimit-Remaining`: Remaining requests
  - `X-RateLimit-Reset`: Seconds until reset
- **Error Handling**: Returns 429 Too Many Requests when limit exceeded

### RateLimitResult

```rust
pub struct RateLimitResult {
    pub allowed: bool,
    pub count: u64,
    pub remaining: u64,
    pub reset_in: u64,
}
```

## Application State (`src/main.rs`)

The `AppState` struct holds shared resources:

```rust
pub struct AppState {
    pub yahoo_auth_manager: Arc<YahooAuthManager>,
    pub fetch_client: Arc<FetchClient>,
    pub yahoo_client: Arc<YahooFinanceClient>,
    pub connection_manager: web::Data<ConnectionManagerAddr>,
    pub cache_service: Arc<CacheService>,
    pub rate_limit_manager: Arc<RateLimitManager>,
}
```

### State Initialization

1. **Environment Loading**: `.env` file loaded via `dotenv`
2. **Tracing Setup**: `tracing_subscriber::fmt::init()` for logging
3. **Client Initialization**: 
   - `FetchClient` created with optional proxy
   - `YahooAuthManager` created with shared cookie jar
   - Authentication primed on startup
   - `YahooFinanceClient` created with auth manager and fetch client
4. **WebSocket Manager**: `ConnectionManager` started as Actix actor
5. **Cache Service**: `CacheService` initialized with Redis URL
6. **Rate Limiting**: `RateLimitManager` initialized with Redis URL and limit

### State Access

All routes access `AppState` via Actix-Web's `web::Data<AppState>` extractor:
```rust
async fn handler(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let client = &app_state.yahoo_client;
    // ...
}
```

## Error Handling

### Error Types (`src/client/error.rs`)

Custom error enum using `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum YahooError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Rate limit exceeded")]
    RateLimited,
    
    #[error("HTTP error: {0}")]
    HttpError(u16, String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
```

### ResponseError Implementation

`YahooError` implements Actix-Web's `ResponseError` trait:
- **AuthFailed** → 401 Unauthorized
- **NotFound** → 404 Not Found
- **RateLimited** → 429 Too Many Requests
- **HttpError** → Custom status code
- **ParseError** → 500 Internal Server Error
- **NetworkError** → 502 Bad Gateway

### Error Propagation

- Use `?` operator for error propagation in async functions
- Errors automatically convert to HTTP responses via `ResponseError`
- JSON error responses include `error` and `message` fields
- Errors are logged via `tracing` macros

## Development Workflow

### Package Management

- **Cargo**: Rust's package manager and build system
- **Commands**: `cargo build`, `cargo test`, `cargo run`
- **Dependencies**: Managed in `Cargo.toml`

### Development Commands

```bash
cargo build          # Build the project
cargo test           # Run tests
cargo run            # Run the development server
cargo fmt            # Format code
cargo clippy         # Run linter
cargo check          # Check compilation without building
```

### Testing

- **Unit Tests**: In same file with `#[cfg(test)]`
- **Integration Tests**: In `tests/` directory
- **Async Tests**: Use `#[tokio::test]` for async test functions

## Deployment Architecture

- **Docker**: Multi-stage builds with optimized images
- **Environment Configuration**: 12-factor app principles with comprehensive environment variables
- **Health Monitoring**: Comprehensive health checks for all services
- **Logging Integration**: Structured logs for observability platforms

### Key Environment Variables

| Variable | Purpose | Default | Required | Usage |
|----------|---------|---------|----------|------|
| `REDIS_URL` | Redis connection string | None | No | Used for caching and rate limiting |
| `RATE_LIMIT_PER_DAY` | Daily request limit per IP | 10,000 | No | Rate limiting configuration |
| `ADMIN_API_KEY` | Admin key bypassing rate limits | None | No | Set in `x-api-key` header |
| `PROXY_URL` | Proxy server URL for requests | None | No | Used by FetchClient and YahooAuthManager |
| `RUST_LOG` | Logging level (trace/debug/info/warn/error) | info | No | Controls tracing output level |

**Note**: The Rust implementation uses `RUST_LOG` for logging configuration (via `tracing-subscriber`), not separate `LOG_FORMAT` or `PERFORMANCE_THRESHOLD_MS` variables. Logo fetching and circuit breaker configurations are not currently implemented in the Rust version.

### Docker Configuration

The `Dockerfile` supports all environment variables:

- **Runtime**: All variables can be set when running containers
- **Compose Ready**: All variables work with docker-compose configurations
- **Multi-stage Builds**: Optimized for production deployments

## Rust-Specific Patterns

### Async/Await

- **Tokio Runtime**: All async operations use Tokio runtime (`#[actix_web::main]`)
- **Arc for Shared State**: `Arc<T>` used extensively for shared immutable data:
  - `Arc<YahooAuthManager>`, `Arc<FetchClient>`, `Arc<YahooFinanceClient>`
  - `Arc<CacheService>`, `Arc<RateLimitManager>`
  - `Arc<ConnectionManager>` for Redis connections
- **Send + Sync Bounds**: All shared data must be `Send + Sync` for async contexts
- **Rc for Middleware**: `Rc<S>` used in middleware to avoid Clone requirement

### Error Handling

- **Result Types**: All fallible operations return `Result<T, YahooError>`
- **thiserror**: Custom error types derive `thiserror::Error`
- **Error Propagation**: `?` operator used throughout for concise error handling
- **ResponseError**: Errors implement `ResponseError` for automatic HTTP conversion
- **Error Logging**: Errors logged via `tracing::error!` macro

### Concurrency

- **Actix Actors**: WebSocket management uses Actix actor system (`ConnectionManager`)
- **Message Passing**: Actors communicate via message types (`Connect`, `Disconnect`, `BroadcastMessage`)
- **Arc for Immutability**: `Arc<T>` for shared immutable data across threads
- **Mutex/RwLock**: Not currently used (prefer message passing in actors)
- **Async Channels**: `mpsc::UnboundedChannel` for WebSocket message broadcasting

### Memory Management

- **Ownership System**: Rust's ownership prevents data races at compile time
- **Arc Usage**: Reference-counted shared ownership for application state
- **Rc Usage**: Single-threaded reference counting in middleware (`Rc<S>`)
- **Clone Minimization**: Avoid unnecessary clones; use references and Arc
- **Zero-Cost Abstractions**: Rust abstractions compile to efficient code

## Performance Considerations

- **Zero-Cost Abstractions**: Rust's abstractions compile away
- **Async I/O**: Non-blocking I/O for high concurrency
- **Connection Pooling**: Reuse HTTP connections
- **Caching**: Reduce external API calls
- **Compilation**: Optimize with `--release` flag for production

## Testing Strategy

- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test API endpoints end-to-end
- **Mock External Services**: Use test doubles for Yahoo Finance API
- **Property-Based Testing**: Consider using `proptest` for complex logic

