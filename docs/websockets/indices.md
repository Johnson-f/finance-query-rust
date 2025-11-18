# Indices Stream

## Overview

**Endpoint:** `/v1/ws/indices`

Streams real-time performance data for major US market indices: S&P 500, Dow Jones Industrial Average, and NASDAQ Composite.

## Protocol

1. **Connect:** Establish a WebSocket connection to the endpoint.
2. **Receive:** The server will automatically start streaming updates every 5 seconds.

## Server Response

A JSON array of market index objects.

```json
[
  {
    "name": "S&P 500",
    "value": 4300.0,
    "change": "+10.00",
    "percentChange": "+0.23%",
    "fiveDaysReturn": "-1.5%",
    "oneMonthReturn": "+2.0%"
  },
  {
    "name": "Dow Jones Industrial Average",
    "value": 34000.0,
    "change": "+150.00",
    "percentChange": "+0.44%"
  },
  {
    "name": "NASDAQ Composite",
    "value": 13500.0,
    "change": "-50.00",
    "percentChange": "-0.37%"
  }
]
```

## Schema

See [MarketIndex Schema](../api/indices.md#marketindex-schema) for detailed field definitions.

