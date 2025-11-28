<!-- 2f0055d2-8b8b-4023-a38c-2bd403da78c8 15f5191d-57f7-4609-87db-588c80a816af -->
# Implement WebSocket Moving Averages Endpoint

## Overview

Create a WebSocket endpoint `/v1/ws/moving-average` that allows clients to subscribe to real-time moving averages (SMA/EMA) for stock symbols. The system will maintain price buffers per symbol and calculate moving averages as new prices arrive.

## Implementation Steps

### 1. Create Price Buffer Manager

**File:** `src/service/websocket/indicator/price_buffer.rs` (new file)

- Create `PricePoint` struct with `price: f64` and `timestamp: i64`
- Create `PriceBufferManager` struct with:
  - `buffers: Arc<RwLock<HashMap<String, Vec<PricePoint>>>>`
  - `max_size: usize` (set to 1000 to support 500-period MAs with buffer)
- Implement methods:
  - `new(max_size: usize) -> Self`
  - `add_price(symbol: &str, price: f64) -> async`
  - `get_prices(symbol: &str) -> async Vec<PricePoint>`
  - `initialize_from_historical(symbol: &str, prices: Vec<PricePoint>) -> async`

### 2. Implement Moving Average Calculations

**File:** `src/service/websocket/indicator/moving_average/sma.rs`

- Implement `calculate(prices: &[f64], period: usize) -> Option<f64>`
- Simple average of last `period` prices

**File:** `src/service/websocket/indicator/moving_average/ema.rs`

- Implement `calculate(prices: &[f64], period: usize) -> Option<f64>`
- Exponential moving average using multiplier `2.0 / (period + 1.0)`

**File:** `src/service/websocket/indicator/moving_average/mod.rs`

- Define `MovingAverageType` enum (SMA, EMA)
- Define `MovingAverage` trait (optional, for extensibility)
- Implement `calculate_ma(prices: &[PricePoint], ma_type: MovingAverageType, period: usize) -> Option<f64>`
- Re-export `sma` and `ema` modules

### 3. Create Moving Average Session Handler

**File:** `src/service/websocket/moving_average_session.rs` (new file)

- Define `SubscriptionRequest` struct (Deserialize):
  - `symbol: String`
  - `type: String` (sma/ema)
  - `period: usize`
- Define `MovingAverageUpdate` struct (Serialize):
  - `symbol: String`
  - `type: String`
  - `period: usize`
  - `value: Option<f64>`
  - `price: f64`
  - `timestamp: String`
- Implement `handle_moving_average_websocket_session()`:
  - Parse subscription requests from client
  - Create channel per subscription set
  - Initialize price buffers from historical data
  - Send initial MA values
  - Start background task that:
    - Fetches current prices every 5 seconds
    - Updates price buffers
    - Calculates MAs
    - Broadcasts updates
- Helper functions:
  - `initialize_buffers()` - fetch historical data and populate buffers
  - `calculate_all_mas()` - calculate MAs for all subscriptions

### 4. Create Route Handler

**File:** `src/routes/websocket/moving_average.rs` (new file)

- Implement `moving_average_handler()` function
- Extract `PriceBufferManager` from `AppState`
- Spawn `handle_moving_average_websocket_session()` task
- Follow pattern from `src/routes/websocket/profile.rs`

### 5. Update AppState

**File:** `src/main.rs`

- Add `price_buffer_manager: Arc<PriceBufferManager>` to `AppState` struct
- Initialize `PriceBufferManager::new(1000)` before creating `AppState`
- Pass to `AppState` constructor

### 6. Register Route

**File:** `src/routes/mod.rs`

- Add `pub mod moving_average;` to module declarations
- Add route in `configure_routes()`:
  ```rust
  .route("/ws/moving-average", web::get().to(websocket::moving_average::moving_average_handler))
  ```


### 7. Update Module Exports

**File:** `src/service/websocket/mod.rs`

- Add `pub mod moving_average_session;`
- Re-export if needed: `pub use moving_average_session::handle_moving_average_websocket_session;`

**File:** `src/service/websocket/indicator/mod.rs`

- Add `pub mod price_buffer;`
- Add `pub mod moving_average;` (already exists, ensure it's exported)

**File:** `src/routes/websocket/mod.rs`

- Add `pub mod moving_average;`

### 8. Historical Data Integration

**File:** `src/service/websocket/moving_average_session.rs` (in `initialize_buffers`)

- Use `service::get_historical()` to fetch historical data
- Determine time range based on max period needed (e.g., 500 days = ~2 years)
- Convert `HistoricalResponse` to `Vec<PricePoint>` using `close` prices
- Handle timestamps from the HashMap keys (string timestamps)

## Technical Details

- **Buffer Size:** 1000 prices per symbol (supports up to 500-period MAs with buffer)
- **Update Frequency:** 5 seconds (matches `REFRESH_INTERVAL`)
- **Channel Naming:** `ma:{symbol1}:{type1}:{period1},{symbol2}:{type2}:{period2},...`
- **Initialization:** Fetch historical data on first subscription per symbol
- **Error Handling:** Gracefully handle missing data, invalid periods, calculation errors

## Testing Considerations

- Test with single subscription
- Test with multiple subscriptions (same symbol, different MAs)
- Test with multiple symbols
- Test buffer overflow (1000+ prices)
- Test with insufficient historical data
- Test WebSocket disconnection cleanup

## Dependencies

- Uses existing: `service::get_historical()`, `service::get_simple_quotes()`
- Uses existing: `ConnectionManager`, `BroadcastMessage` patterns
- New: `chrono` for timestamps (if not already in Cargo.toml)

### To-dos

- [ ] Create PriceBufferManager in src/service/websocket/indicator/price_buffer.rs with methods to manage price buffers per symbol
- [ ] Implement SMA calculation in src/service/websocket/indicator/moving_average/sma.rs
- [ ] Implement EMA calculation in src/service/websocket/indicator/moving_average/ema.rs
- [ ] Update src/service/websocket/indicator/moving_average/mod.rs to export SMA/EMA and add calculate_ma function
- [ ] Create handle_moving_average_websocket_session in src/service/websocket/moving_average_session.rs with subscription handling and background task
- [ ] Create moving_average_handler in src/routes/websocket/moving_average.rs
- [ ] Add PriceBufferManager to AppState in src/main.rs and initialize with max_size 1000
- [ ] Register /v1/ws/moving-average route in src/routes/mod.rs
- [ ] Update module exports in src/service/websocket/mod.rs, src/service/websocket/indicator/mod.rs, and src/routes/websocket/mod.rs