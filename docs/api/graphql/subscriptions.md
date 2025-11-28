# GraphQL Subscriptions

## Overview

GraphQL subscriptions provide real-time data updates over WebSocket connections. All subscriptions automatically push updates every **5 seconds**, allowing you to build reactive applications that respond to market changes in real-time.

**WebSocket Endpoint:** `ws://localhost:8080/graphql` (development) or `wss://api.tradstry.com/graphql` (production)

**HTTP Endpoint:** `GET /graphql` (for WebSocket upgrade)

## How Subscriptions Work

1. **Connection:** Client establishes a WebSocket connection to `/graphql`
2. **Subscription:** Client sends a subscription query
3. **Updates:** Server pushes data updates every 5 seconds
4. **Streaming:** Updates continue until the client unsubscribes or disconnects

## Using Subscriptions

### In GraphQL Playground

1. Navigate to `http://localhost:8080/graphql-playground`
2. Enter your subscription query
3. Click the "Play" button
4. Watch real-time updates appear in the response panel

### Using JavaScript/TypeScript

```javascript
import { createClient } from 'graphql-ws';

const client = createClient({
  url: 'ws://localhost:8080/graphql',
});

client.subscribe(
  {
    query: `
      subscription {
        quoteUpdates(symbols: ["AAPL"]) {
          symbol
          price
          change
        }
      }
    `,
  },
  {
    next: (data) => console.log('Update:', data),
    error: (err) => console.error('Error:', err),
    complete: () => console.log('Complete'),
  }
);
```

## Subscription Types

### `profileUpdates`

Subscribe to comprehensive profile data for a symbol, including quote, similar stocks, sector performance, and news.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Subscription:**
```graphql
subscription {
  profileUpdates(symbol: "AAPL") {
    quote {
      symbol
      name
      price
      change
      percentChange
      marketCap
      sector
    }
    similar {
      symbol
      name
      price
      change
    }
    sectorPerformance {
      name
      change
      percentChange
    }
    news {
      title
      link
      source
      time
    }
  }
}
```

**Response Format:**
```json
{
  "data": {
    "profileUpdates": {
      "quote": {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "price": "145.00",
        "change": "+1.00",
        "percentChange": "+0.69%"
      },
      "similar": [
        {
          "symbol": "MSFT",
          "name": "Microsoft Corporation",
          "price": "350.00",
          "change": "+2.00"
        }
      ],
      "sectorPerformance": {
        "name": "Technology",
        "change": "+5.00",
        "percentChange": "+1.2%"
      },
      "news": [
        {
          "title": "Apple Reports Strong Earnings",
          "link": "https://...",
          "source": "Reuters",
          "time": "2 hours ago"
        }
      ]
    }
  }
}
```

**Update Frequency:** Every 5 seconds

### `quoteUpdates`

Subscribe to real-time quote updates for multiple symbols.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbols` | `[String!]!`  |    ✓     | Array of stock ticker symbols   | `["AAPL", "MSFT"]` |

**Example Subscription:**
```graphql
subscription {
  quoteUpdates(symbols: ["AAPL", "MSFT", "GOOG"]) {
    symbol
    name
    price
    change
    percentChange
    logo
  }
}
```

**Response Format:**
```json
{
  "data": {
    "quoteUpdates": {
      "symbol": "AAPL",
      "name": "Apple Inc.",
      "price": "145.00",
      "change": "+1.00",
      "percentChange": "+0.69%"
    }
  }
}
```

**Note:** Each symbol in the array will yield a separate update in the stream.

**Update Frequency:** Every 5 seconds

### `indicesUpdates`

Subscribe to US market indices updates (DJIA, NASDAQ, S&P 500).

**Arguments:** None

**Example Subscription:**
```graphql
subscription {
  indicesUpdates {
    symbol
    name
    price
    change
    percentChange
    lastUpdated
  }
}
```

**Response Format:**
```json
{
  "data": {
    "indicesUpdates": {
      "symbol": "^DJI",
      "name": "Dow Jones Industrial Average",
      "price": "35000.00",
      "change": "+100.00",
      "percentChange": "+0.29%",
      "lastUpdated": "2024-01-01T16:00:00Z"
    }
  }
}
```

**Note:** Each index (DJIA, NASDAQ, S&P 500) will yield a separate update in the stream.

**Update Frequency:** Every 5 seconds

### `newsUpdates`

Subscribe to general market news updates.

**Arguments:** None

**Example Subscription:**
```graphql
subscription {
  newsUpdates {
    title
    link
    source
    img
    time
  }
}
```

**Response Format:**
```json
{
  "data": {
    "newsUpdates": {
      "title": "Market Opens Higher on Strong Earnings",
      "link": "https://...",
      "source": "Bloomberg",
      "img": "https://...",
      "time": "1 hour ago"
    }
  }
}
```

**Note:** Each news article will yield a separate update in the stream.

**Update Frequency:** Every 5 seconds

### `sectorsUpdates`

Subscribe to sector performance updates.

**Arguments:** None

**Example Subscription:**
```graphql
subscription {
  sectorsUpdates {
    name
    change
    percentChange
    lastUpdated
  }
}
```

**Response Format:**
```json
{
  "data": {
    "sectorsUpdates": {
      "name": "Technology",
      "change": "+5.00",
      "percentChange": "+1.2%",
      "lastUpdated": "2024-01-01T16:00:00Z"
    }
  }
}
```

**Note:** Each sector will yield a separate update in the stream.

**Update Frequency:** Every 5 seconds

### `moversUpdates`

Subscribe to market movers updates (actives, gainers, losers).

**Arguments:** None

**Example Subscription:**
```graphql
subscription {
  moversUpdates {
    actives {
      symbol
      name
      price
      change
      percentChange
      volume
    }
    gainers {
      symbol
      name
      price
      change
      percentChange
    }
    losers {
      symbol
      name
      price
      change
      percentChange
    }
  }
}
```

**Response Format:**
```json
{
  "data": {
    "moversUpdates": {
      "actives": [
        {
          "symbol": "AAPL",
          "name": "Apple Inc.",
          "price": "145.00",
          "change": "+1.00",
          "percentChange": "+0.69%",
          "volume": 50000000
        }
      ],
      "gainers": [
        {
          "symbol": "TSLA",
          "name": "Tesla Inc.",
          "price": "250.00",
          "change": "+10.00",
          "percentChange": "+4.17%"
        }
      ],
      "losers": [
        {
          "symbol": "XYZ",
          "name": "XYZ Corp",
          "price": "50.00",
          "change": "-2.00",
          "percentChange": "-3.85%"
        }
      ]
    }
  }
}
```

**Update Frequency:** Every 5 seconds

### `marketHoursUpdates`

Subscribe to market status and hours updates.

**Arguments:** None

**Example Subscription:**
```graphql
subscription {
  marketHoursUpdates {
    status
    reason
    timestamp
  }
}
```

**Response Format:**
```json
{
  "data": {
    "marketHoursUpdates": {
      "status": "OPEN",
      "reason": null,
      "timestamp": "2024-01-01T16:00:00Z"
    }
  }
}
```

**Status Values:**
- `"OPEN"` - Market is currently open
- `"CLOSED"` - Market is currently closed
- `"PRE_MARKET"` - Pre-market trading hours
- `"AFTER_HOURS"` - After-hours trading

**Update Frequency:** Every 5 seconds

### `movingAverageUpdates`

Subscribe to real-time moving average calculations for a symbol.

**Arguments:**

| Argument        | Type           | Required | Description                     | Example          |
|-----------------|----------------|:--------:|---------------------------------|------------------|
| `symbol`        | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |
| `indicatorType` | `String!`      |    ✓     | Indicator type (`"sma"` or `"ema"`) | `"sma"`      |
| `period`        | `Int!`         |    ✓     | Period for moving average      | `20`             |

**Example Subscription:**
```graphql
subscription {
  movingAverageUpdates(
    symbol: "AAPL"
    indicatorType: "sma"
    period: 20
  ) {
    symbol
    indicatorType
    period
    value
    timestamp
  }
}
```

**Response Format:**
```json
{
  "data": {
    "movingAverageUpdates": {
      "symbol": "AAPL",
      "indicatorType": "sma",
      "period": 20,
      "value": 145.50,
      "timestamp": "2024-01-01T16:00:00Z"
    }
  }
}
```

**Indicator Types:**
- `"sma"` - Simple Moving Average
- `"ema"` - Exponential Moving Average

**Important Notes:**
- The subscription requires sufficient price history to calculate the moving average
- If there aren't enough data points, the subscription will yield an error until enough data is collected
- The price buffer accumulates data over time, so the first few updates may fail until enough history is available
- Real-time prices are fetched every 5 seconds and added to the buffer

**Update Frequency:** Every 5 seconds

## Error Handling

Subscriptions handle errors gracefully:

```json
{
  "errors": [
    {
      "message": "Failed to fetch quotes: Network error",
      "locations": [{"line": 2, "column": 3}],
      "path": ["quoteUpdates"]
    }
  ],
  "data": null
}
```

**Error Behavior:**
- Single fetch failures don't terminate the subscription
- The subscription continues streaming and will retry on the next update cycle
- If the context becomes unavailable, the subscription will terminate

## Best Practices

### 1. Field Selection

Only request the fields you need to reduce payload size:

```graphql
# Good - Only requested fields
subscription {
  quoteUpdates(symbols: ["AAPL"]) {
    symbol
    price
    change
  }
}

# Less efficient - Requesting all fields
subscription {
  quoteUpdates(symbols: ["AAPL"]) {
    symbol
    name
    price
    change
    percentChange
    open
    high
    low
    volume
    # ... many more fields
  }
}
```

### 2. Multiple Subscriptions

You can subscribe to multiple data streams simultaneously:

```graphql
subscription {
  quoteUpdates(symbols: ["AAPL"]) {
    symbol
    price
  }
  marketHoursUpdates {
    status
    timestamp
  }
}
```

### 3. Unsubscribing

Always properly unsubscribe when done to free server resources:

```javascript
const unsubscribe = client.subscribe(/* ... */);

// Later, when done
unsubscribe();
```

### 4. Reconnection Handling

Implement reconnection logic for production applications:

```javascript
const client = createClient({
  url: 'ws://localhost:8080/graphql',
  shouldRetry: () => true,
  retryAttempts: Infinity,
  retryWait: async function* () {
    for (let i = 0; i < Infinity; i++) {
      yield i * 1000; // Exponential backoff
    }
  },
});
```

## Rate Limiting

Subscriptions are subject to the same rate limiting as queries:
- Rate limits apply per IP address
- Excessive subscription connections may be rate-limited
- See the main API documentation for current rate limit details

## WebSocket Protocol

The GraphQL subscriptions use the standard GraphQL over WebSocket protocol:
- **Protocol:** `graphql-ws` or `graphql-transport-ws`
- **Connection:** WebSocket upgrade from HTTP GET request
- **Message Format:** JSON

## Testing Subscriptions

### Using GraphQL Playground

1. Open `http://localhost:8080/graphql-playground`
2. Enter a subscription query
3. Click "Play"
4. Watch updates appear in real-time

### Using cURL (Limited)

cURL doesn't support WebSocket connections well. For testing subscriptions, use:
- GraphQL Playground (recommended)
- A WebSocket client library in your preferred language
- Browser DevTools WebSocket inspector

## Production Considerations

1. **WebSocket Connections:** Each subscription maintains a persistent WebSocket connection
2. **Server Resources:** Monitor connection count and resource usage
3. **Scaling:** Consider connection pooling and load balancing for high-traffic scenarios
4. **Security:** Implement authentication/authorization for production use
5. **Monitoring:** Track subscription connection health and error rates

## Troubleshooting

### Connection Issues

- **Error:** "Could not connect to websocket endpoint"
  - **Solution:** Ensure the server is running and WebSocket support is enabled
  - **Check:** Verify the endpoint URL is correct (`ws://localhost:8080/graphql`)

### No Updates Received

- **Check:** Verify the subscription query syntax is correct
- **Check:** Ensure the WebSocket connection is established
- **Check:** Look for errors in the subscription response

### Moving Average Errors

- **Error:** "Not enough data points"
  - **Solution:** Wait for the price buffer to accumulate enough history
  - **Note:** The buffer builds up over time as prices are fetched

### High Latency

- **Check:** Network connection quality
- **Check:** Server load and resource usage
- **Note:** Updates are sent every 5 seconds by design

