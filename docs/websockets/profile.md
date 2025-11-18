# Profile Stream

## Overview

**Endpoint:** `/v1/ws/profile/{symbol}`

Streams comprehensive data for a single stock symbol, including its current quote, similar stocks, sector performance, and related news. This is useful for populating a detailed stock profile page.

## Protocol

1. **Connect:** Establish a WebSocket connection to the endpoint with the target symbol in the path (e.g., `/v1/ws/profile/AAPL`).
2. **Receive:** The server will automatically start streaming updates every 5 seconds.

## Server Response

A JSON object containing grouped data.

```json
{
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
      "price": "300.00",
      "change": "-2.50",
      "percentChange": "-0.83%"
    }
  ],
  "sectorPerformance": {
    "sector": "Technology",
    "dayReturn": "+1.2%",
    "ytdReturn": "+15.4%"
  },
  "news": [
    {
      "title": "Apple releases new iPhone",
      "link": "https://...",
      "source": "TechCrunch",
      "time": "2 hours ago"
    }
  ]
}
```

## Schema

| Field | Type | Description |
|-------|------|-------------|
| `quote` | object | Current [SimpleQuote](../api/quotes.md#simplequote-schema) data |
| `similar` | array | List of similar stocks ([SimpleQuote](../api/quotes.md#simplequote-schema)) |
| `sectorPerformance` | object | Performance of the stock's sector ([MarketSector](../api/sectors.md#marketsector-schema)) |
| `news` | array | Recent news articles ([News](../api/news.md#news-schema)) |

