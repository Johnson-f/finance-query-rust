# WebSocket Streams

## Overview

The API provides real-time updates via WebSocket connections. The base path for all WebSocket endpoints is `/v1/ws/`.

## Connection Handling

- **Protocol:** `ws://` (or `wss://` for secure connections)
- **Heartbeat:** The server may send ping frames. Clients should respond with pong frames or handle connection timeouts.
- **Reconnection:** Clients are responsible for reconnecting if the connection is dropped.

## Endpoints

### /v1/ws/quotes

**Purpose:** Stream real-time stock quotes for specific symbols.

**Protocol:**
1. Connect to `/v1/ws/quotes`.
2. Send a text message with comma-separated symbols (e.g., `AAPL,MSFT,NVDA`).
3. Receive periodic JSON updates containing an array of `SimpleQuote` objects.

**Example Message:**
```json
[
  {
    "symbol": "AAPL",
    "name": "Apple Inc.",
    "price": "145.00",
    "change": "+1.00",
    "percent_change": "+0.69%",
    "logo": "https://..."
  }
]
```

### /v1/ws/profile/{symbol}

**Purpose:** Stream comprehensive profile data for a specific symbol (quote, similar stocks, sector performance, news).

**Protocol:**
1. Connect to `/v1/ws/profile/{symbol}` (e.g., `/v1/ws/profile/AAPL`).
2. Receive periodic JSON updates.

**Example Message:**
```json
{
  "quote": { ... },
  "similar": [ ... ],
  "sectorPerformance": { ... },
  "news": [ ... ]
}
```

### /v1/ws/indices

**Purpose:** Stream major market indices (S&P 500, Dow Jones, NASDAQ).

**Protocol:**
1. Connect to `/v1/ws/indices`.
2. Receive periodic JSON updates containing an array of `MarketIndex` objects.

### /v1/ws/news

**Purpose:** Stream latest general financial news.

**Protocol:**
1. Connect to `/v1/ws/news`.
2. Receive periodic JSON updates containing an array of `News` objects.

### /v1/ws/sectors

**Purpose:** Stream performance of all market sectors.

**Protocol:**
1. Connect to `/v1/ws/sectors`.
2. Receive periodic JSON updates containing an array of `MarketSector` objects.

### /v1/ws/movers

**Purpose:** Stream market movers (actives, gainers, losers).

**Protocol:**
1. Connect to `/v1/ws/movers`.
2. Receive periodic JSON updates containing `actives`, `gainers`, and `losers` lists.

### /v1/ws/hours

**Purpose:** Stream current market status (open/closed).

**Protocol:**
1. Connect to `/v1/ws/hours`.
2. Receive periodic JSON updates containing market status.

**Example Message:**
```json
{
  "status": "open",
  "reason": "Regular Trading Hours",
  "timestamp": "..."
}
```

