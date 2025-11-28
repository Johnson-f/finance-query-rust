## Health

### `ping`

Basic health check.

**Example Query:**
```graphql
query {
  ping
}
```

**Response:**
```json
{
  "data": {
    "ping": "healthy"
  }
}
```

### `health`

Comprehensive health check with timestamp and service status.

**Example Query:**
```graphql
query {
  health {
    status
    timestamp
    services {
      status
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "health": {
      "status": "healthy",
      "timestamp": "2024-01-01T12:00:00Z",
      "services": {
        "status": "all_operational"
      }
    }
  }
}
```