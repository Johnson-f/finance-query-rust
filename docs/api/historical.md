# Historical Prices

## GET /v1/historical/{symbol}

### Overview

**Purpose:** Historical stock price data retrieval  
**Response Format:** Object containing time-series OHLCV data keyed by epoch timestamp

### Path Parameters

| Parameter | Type   | Required | Description         | Example |
|-----------|--------|:--------:|---------------------|---------|
| `symbol`  | string |    ✓     | Stock ticker symbol | `NVDA`  |

### Request Parameters

| Parameter  | Type    | Required | Description          | Example |
|------------|---------|:--------:|----------------------|---------|
| `range`    | string  |    ✓     | Historical time range| `1y`    |
| `interval` | string  |    ✓     | Data point frequency | `1d`    |

#### Available Range Options
`1d`, `5d`, `1mo`, `3mo`, `6mo`, `ytd`, `1y`, `2y`, `5y`, `10y`, `max`

#### Available Interval Options
`1m`, `5m`, `15m`, `30m`, `1h`, `1d`, `1wk`, `1mo`

!!! warning "Interval and Range Compatibility"
    | Interval | Compatible Ranges                                   |
    |----------|-----------------------------------------------------|
    | `1m`     | `1d`, `5d` only                                     |
    | `5m`     | `1d`, `5d`, `1mo` only                              |
    | `15m`    | `1d`, `5d`, `1mo` only                              |
    | `30m`    | `1d`, `5d`, `1mo` only                              |
    | `1h`     | `1d`, `5d`, `1mo`, `3mo`, `6mo`, `ytd`, `1y` only   |
    | `1mo`    | Required for `max` range                            |
    
    Attempting incompatible combinations may result in a provider error.

**Responses:**

- **200 OK**  
  - **Content-Type:** `application/json`  
  - **Schema:** [`HistoricalResponse`](#historicalresponse-schema)
  - **Example (200):**
    ```json
    {
      "data": {
        "1696167000": {
          "open": 300.0,
          "high": 305.0,
          "low": 295.0,
          "close": 302.0,
          "volume": 1500000,
          "adj_close": 302.0
        },
        "1696253400": {
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

- **400 Bad Request**  
  ```json
  { "detail": "Invalid time range" }
  ```
  or
  ```json
  { "detail": "Invalid interval" }
  ```

## Schema References

### HistoricalResponse Schema

| Field | Type                                    | Description                            | Required |
|-------|-----------------------------------------|----------------------------------------|:--------:|
| data  | Map<string, [HistoricalData](#historicaldata-schema)> | Map of timestamp (string) to price data |    ✓     |

### HistoricalData Schema

| Field       | Type    | Description            | Required |
|:------------|:--------|:-----------------------|:--------:|
| `open`      | number  | Opening price          |    ✓     |
| `high`      | number  | Highest price          |    ✓     |
| `low`       | number  | Lowest price           |    ✓     |
| `close`     | number  | Closing price          |    ✓     |
| `volume`    | integer | Volume traded          |    ✓     |
| `adj_close` | number  | Adjusted closing price |          |

