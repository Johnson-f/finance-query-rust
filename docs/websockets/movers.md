# Movers Stream

## Overview

**Endpoint:** `/v1/ws/movers`

Streams real-time lists of market movers: most active by volume, top gainers, and top losers. 

**Note:** This stream filters for US stocks only (symbols without dots or with US exchange suffixes like .OB, .PK).

## Protocol

1. **Connect:** Establish a WebSocket connection to the endpoint.
2. **Receive:** The server will automatically start streaming updates every 5 seconds.

## Server Response

A JSON object containing three lists.

```json
{
  "actives": [
    {
      "symbol": "AAPL",
      "name": "Apple Inc.",
      "price": "145.00",
      "change": "+1.00",
      "percentChange": "+0.69%"
    }
  ],
  "gainers": [
    {
      "symbol": "TSLA",
      "name": "Tesla Inc.",
      "price": "250.00",
      "change": "+10.00",
      "percentChange": "+4.16%"
    }
  ],
  "losers": [
    {
      "symbol": "AMD",
      "name": "Advanced Micro Devices",
      "price": "100.00",
      "change": "-5.00",
      "percentChange": "-4.76%"
    }
  ]
}
```

## Schema

Each item in the lists follows the [MarketMover Schema](../api/movers.md#marketmover-schema).

