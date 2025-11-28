# Historical Prices

## GET https://api.tradstry.com/v1/historical/{symbol}

### Overview

**Purpose:** Historical stock price data retrieval with optional technical indicators  
**Base URL:** `https://api.tradstry.com`  
**Response Format:** Object containing time-series OHLCV data keyed by RFC3339 timestamp, optionally including SMA and EMA indicators

### Path Parameters

| Parameter | Type   | Required | Description         | Example |
|-----------|--------|:--------:|---------------------|---------|
| `symbol`  | string |    ✓     | Stock ticker symbol | `NVDA`  |

### Request Parameters

| Parameter   | Type    | Required | Description                                    | Example        |
|-------------|---------|:--------:|------------------------------------------------|----------------|
| `range`     | string  |    ✓     | Historical time range                          | `1y`           |
| `interval`  | string  |    ✓     | Data point frequency                           | `1d`           |
| `indicators`| string  |          | Comma-separated indicator types                | `sma,ema`      |
| `period`    | string  |          | Comma-separated periods for indicators         | `10,20,50`     |

#### Available Range Options
`1d`, `5d`, `1mo`, `3mo`, `6mo`, `ytd`, `1y`, `2y`, `5y`, `10y`, `max`

#### Available Interval Options
`1m`, `3m`, `5m`, `10m`, `15m`, `20m`, `30m`, `65m`, `95m`, `1h`, `1d`, `1wk`, `1mo`

!!! note "Resampled Intervals"
    Intervals `3m`, `10m`, `20m`, and `65m` are not natively supported by Yahoo Finance. 
    The API automatically resamples from 1-minute data to provide these intervals.

#### Indicator Options
- `sma` - Simple Moving Average
- `ema` - Exponential Moving Average

#### Period Parameter
- Single period: `period=20` (default if not specified)
- Multiple periods: `period=10,20,50` (comma-separated)
- Each period must be a positive integer
- Periods are interpreted in terms of the requested interval (e.g., `period=20` with `interval=1d` means 20 days)

!!! warning "Interval and Range Compatibility"
    | Interval | Compatible Ranges                                   |
    |----------|-----------------------------------------------------|
    | `1m`, `3m`, `5m`, `10m`, `15m`, `20m`, `30m`, `65m` | `1d`, `5d` only |
    | `95m`    | All ranges                                          |
    | `1h`     | `1d`, `5d`, `1mo`, `3mo`, `6mo`, `ytd`, `1y` only   |
    | `1d`     | All ranges (uses optimized chunking for `max`)      |
    | `1wk`    | All ranges (uses optimized chunking for `max`)      |
    | `1mo`    | All ranges (required for `max` range)               |
    
    Attempting incompatible combinations will return a 400 error with a descriptive message.

**Responses:**

- **200 OK**  
  - **Content-Type:** `application/json`  
  - **Schema:** [`HistoricalResponse`](#historicalresponse-schema)
  
  **Example (200) - Basic Request:**
    ```json
    {
      "data": {
        "2024-01-01T00:00:00Z": {
          "open": 300.0,
          "high": 305.0,
          "low": 295.0,
          "close": 302.0,
          "volume": 1500000,
          "adj_close": 302.0
        },
        "2024-01-02T00:00:00Z": {
          "open": 302.0,
          "high": 310.0,
          "low": 300.0,
          "close": 308.0,
          "volume": 1600000,
          "adj_close": 308.0
        }
      }
    }
    ```
  
  **Example (200) - With Indicators (Single Period):**
    ```json
    {
      "data": {
        "2024-01-01T00:00:00Z": {
          "open": 300.0,
          "high": 305.0,
          "low": 295.0,
          "close": 302.0,
          "volume": 1500000,
          "adj_close": 302.0,
          "sma": {
            "20": 298.5
          },
          "ema": {
            "20": 299.2
          }
        }
      }
    }
    ```
  
  **Example (200) - With Indicators (Multiple Periods):**
    ```json
    {
      "data": {
        "2024-01-01T00:00:00Z": {
          "open": 300.0,
          "high": 305.0,
          "low": 295.0,
          "close": 302.0,
          "volume": 1500000,
          "adj_close": 302.0,
          "ema": {
            "10": 300.5,
            "20": 299.8,
            "50": 298.2
          }
        }
      }
    }
    ```

- **400 Bad Request**  
  ```json
  { "detail": "Invalid time range" }
  ```
  or
  ```json
  { "detail": "Invalid interval" }
  ```
  or
  ```json
  { "detail": "The interval '3m' can only be used with ranges '1d' or '5d'. Please use one of these ranges or choose a different interval." }
  ```
  or
  ```json
  { "detail": "Invalid indicator type. Supported: sma, ema" }
  ```
  or
  ```json
  { "detail": "Invalid period value: 'abc'. Periods must be positive integers." }
  ```

## Usage Examples

### Basic Request
```bash
curl 'https://api.tradstry.com/v1/historical/TSLA?range=1y&interval=1d'
```

### Request with Single Indicator
```bash
# SMA with default 20-period
curl 'https://api.tradstry.com/v1/historical/TSLA?range=1y&interval=1d&indicators=sma'

# EMA with custom period
curl 'https://api.tradstry.com/v1/historical/TSLA?range=1y&interval=1d&indicators=ema&period=50'
```

### Request with Multiple Periods
```bash
# Multiple EMA periods
curl 'https://api.tradstry.com/v1/historical/TSLA?range=1d&interval=20m&indicators=ema&period=10,20,50'

# Multiple SMA periods
curl 'https://api.tradstry.com/v1/historical/TSLA?range=5d&interval=10m&indicators=sma&period=5,10,20'

# Both SMA and EMA with multiple periods
curl 'https://api.tradstry.com/v1/historical/TSLA?range=1mo&interval=1h&indicators=sma,ema&period=10,20,50'
```

### Resampled Intervals
```bash
# 3m interval (resampled from 1m)
curl 'https://api.tradstry.com/v1/historical/TSLA?range=5d&interval=3m'

# 10m interval with indicators
curl 'https://api.tradstry.com/v1/historical/TSLA?range=5d&interval=10m&indicators=ema&period=20'

# 20m interval with multiple periods
curl 'https://api.tradstry.com/v1/historical/TSLA?range=1d&interval=20m&indicators=sma,ema&period=10,20,50'
```

### Max Range (Full History)
```bash
# Daily data for entire history
curl 'https://api.tradstry.com/v1/historical/TSLA?range=max&interval=1d'

# Weekly data for entire history with indicators
curl 'https://api.tradstry.com/v1/historical/TSLA?range=max&interval=1wk&indicators=sma,ema&period=20,50,200'
```

## Schema References

### HistoricalResponse Schema

| Field | Type                                    | Description                            | Required |
|-------|-----------------------------------------|----------------------------------------|:--------:|
| data  | Map<string, [HistoricalData](#historicaldata-schema)> | Map of RFC3339 timestamp (string) to price data |    ✓     |

### HistoricalData Schema

| Field       | Type                                    | Description                                    | Required |
|:------------|:----------------------------------------|:-----------------------------------------------|:--------:|
| `open`      | number                                  | Opening price                                  |    ✓     |
| `high`      | number                                  | Highest price                                  |    ✓     |
| `low`       | number                                  | Lowest price                                   |    ✓     |
| `close`     | number                                  | Closing price                                  |    ✓     |
| `volume`    | integer                                 | Volume traded                                  |    ✓     |
| `adj_close` | number                                  | Adjusted closing price                         |          |
| `sma`       | Map<string, number>                    | Simple Moving Average values keyed by period  |          |
| `ema`       | Map<string, number>                    | Exponential Moving Average values keyed by period |      |

#### Indicator Fields

When `indicators` parameter is provided, the response includes `sma` and/or `ema` fields as HashMaps:
- **Keys**: Period strings (e.g., `"10"`, `"20"`, `"50"`)
- **Values**: Indicator values for that period
- **Omitted**: If not requested or insufficient data for calculation

**Note:** The first `period-1` data points will not have indicator values (null/omitted) as there isn't enough historical data to calculate them yet.

## Technical Details

### Resampling Logic

For intervals `3m`, `10m`, `20m`, and `65m`:
1. The API fetches 1-minute data from Yahoo Finance
2. Groups data into buckets based on the target interval
3. Aggregates OHLCV:
   - **Open**: First open price in the bucket
   - **High**: Maximum high price in the bucket
   - **Low**: Minimum low price in the bucket
   - **Close**: Last close price in the bucket
   - **Volume**: Sum of all volumes in the bucket
   - **Adj_close**: Last adjusted close in the bucket

### Indicator Calculation

- Indicators are calculated on the **close** prices
- Periods are interpreted in terms of the requested interval:
  - `period=20` with `interval=1d` = 20-day moving average
  - `period=20` with `interval=1h` = 20-hour moving average
  - `period=20` with `interval=20m` = 20 × 20-minute = 400-minute moving average
- Multiple periods are calculated efficiently in a single pass
- Works with both native and resampled intervals

### Timestamp Format

All timestamps in the response are in **RFC3339** format (ISO 8601):
- Example: `"2024-01-01T00:00:00Z"`
- Timezone: UTC (Z suffix)