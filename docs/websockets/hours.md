# Hours Stream

## Overview

**Endpoint:** `/v1/ws/hours`

Streams the current status of the US stock market (Open/Closed).

## Protocol

1. **Connect:** Establish a WebSocket connection to the endpoint.
2. **Receive:** The server will automatically start streaming updates every 5 seconds.

## Server Response

A JSON object describing the market status.

```json
{
  "status": "open",
  "reason": "Regular Trading Hours",
  "timestamp": "2023-10-27T14:30:00Z"
}
```

## Schema

| Field | Type | Description |
|-------|------|-------------|
| `status` | string | "open", "closed", "pre-market", "after-hours" |
| `reason` | string | Description of the status |
| `timestamp` | string | ISO timestamp of the check |

