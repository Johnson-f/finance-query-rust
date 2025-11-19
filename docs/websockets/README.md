# WebSocket API

## Overview

The Finance Query API provides real-time updates via WebSocket connections. This allows clients to receive continuous data updates without polling.

## Connection Details

- **Base URL:** `wss://finance-query.onrender.com/v1/ws` (or `ws://localhost:8080/v1/ws` for local development)
- **Protocol:** WebSocket (RFC 6455)

## Features

- **Real-time Updates:** Data is pushed to the client as soon as it's available (updated every 5 seconds).
- **Efficient:** Reduces network overhead compared to polling.
- **Resilient:** Supports reconnection handling.

## Heartbeats & Connection Health

- **Ping/Pong:** The server sends standard WebSocket PING frames to ensure the connection is alive. Clients should respond with PONG frames automatically (handled by most WebSocket libraries).
- **Timeouts:** If no data or heartbeat is received for 60 seconds, the client should assume the connection is lost and attempt to reconnect.

## Error Handling

- **Connection Errors:** Standard WebSocket close codes are used.
- **Invalid Requests:** If a client sends an invalid message (e.g., malformed JSON for quotes), the server may close the connection with a specific error code or ignore the message depending on severity.

## Authentication

Optional authentication via `x-api-key` query parameter or header (depending on client support, usually query parameter `?token=YOUR_KEY` is easiest for WebSockets).

## Available Channels

| Channel | Endpoint | Description |
|---------|----------|-------------|
| [Quotes](quotes.md) | `/quotes` | Real-time stock prices for specific symbols |
| [Profile](profile.md) | `/profile/{symbol}` | Comprehensive single-stock data (quote, news, etc.) |
| [Indices](indices.md) | `/indices` | Major market indices (S&P 500, NASDAQ, etc.) |
| [News](news.md) | `/news` | General market news stream |
| [Sectors](sectors.md) | `/sectors` | Sector performance updates |
| [Movers](movers.md) | `/movers` | Market actives, gainers, and losers |
| [Hours](hours.md) | `/hours` | Market open/close status |

