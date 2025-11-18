# Sectors Stream

## Overview

**Endpoint:** `/v1/ws/sectors`

Streams real-time performance data for all market sectors.

## Protocol

1. **Connect:** Establish a WebSocket connection to the endpoint.
2. **Receive:** The server will automatically start streaming updates every 5 seconds.

## Server Response

A JSON array of sector performance objects.

```json
[
  {
    "sector": "Technology",
    "dayReturn": "+1.5%",
    "ytdReturn": "+20.0%",
    "yearReturn": "+35.0%",
    "threeYearReturn": "+50.0%",
    "fiveYearReturn": "+150.0%"
  },
  {
    "sector": "Energy",
    "dayReturn": "-0.5%",
    "ytdReturn": "+5.0%",
    "yearReturn": "+10.0%",
    "threeYearReturn": "+25.0%",
    "fiveYearReturn": "+40.0%"
  }
]
```

## Schema

See [MarketSector Schema](../api/sectors.md#marketsector-schema) for detailed field definitions.

