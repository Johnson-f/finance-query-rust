# Quotes Stream

## Overview

**Endpoint:** `/v1/ws/quotes`

Streams real-time stock price updates for a customizable list of symbols.

## Protocol

1. **Connect:** Establish a WebSocket connection to the endpoint.
2. **Subscribe:** Send a text message containing a comma-separated list of symbols you want to track.
3. **Receive:** The server will immediately send the current quotes, and then stream updates every 5 seconds.

## Message Format

### Client Request (Subscribe)

Send a simple string with comma-separated tickers:
```text
AAPL,MSFT,NVDA
```

### Server Response

A JSON array of quote objects.

```json
[
  {
    "symbol": "AAPL",
    "name": "Apple Inc.",
    "price": "145.00",
    "change": "+1.00",
    "percentChange": "+0.69%",
    "preMarketPrice": "145.50",
    "afterHoursPrice": "144.80"
  },
  {
    "symbol": "MSFT",
    "name": "Microsoft Corporation",
    "price": "300.00",
    "change": "-2.50",
    "percentChange": "-0.83%"
  }
]
```

## Schema

| Field             | Type   | Description                |
|-------------------|--------|----------------------------|
| `symbol`          | string | Stock ticker symbol        |
| `name`            | string | Company name               |
| `price`           | string | Current/Last traded price  |
| `change`          | string | Price change since open    |
| `percentChange`   | string | Percentage change          |
| `preMarketPrice`  | string | Pre-market price (optional)|
| `afterHoursPrice` | string | After-hours price (optional)|

**Note:** Unlike the REST API, logos are excluded from the WebSocket stream to optimize performance and bandwidth.

