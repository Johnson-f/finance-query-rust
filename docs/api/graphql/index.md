# GraphQL API

## Overview

The GraphQL API provides a flexible, type-safe interface for querying financial data. Unlike REST endpoints, GraphQL allows you to request exactly the data you need in a single request, reducing over-fetching and enabling efficient data retrieval.

**Base URL:** `http://localhost:8080` (development) or `https://api.tradstry.com` (production)

**Endpoints:**
- **POST `/graphql`** - Execute GraphQL queries and mutations
- **GET `/graphql`** - WebSocket endpoint for GraphQL subscriptions
- **GET `/graphql-playground`** - Interactive GraphQL Playground UI

## Getting Started

### Using GraphQL Playground

1. Navigate to `http://localhost:8080/graphql-playground` in your browser
2. Use the interactive interface to explore the schema and test queries
3. For subscriptions, the Playground automatically handles WebSocket connections

### Using cURL

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { quotes(symbols: [\"AAPL\"]) { symbol name price } }"
  }'
```

### Using JavaScript/TypeScript

```javascript
const response = await fetch('http://localhost:8080/graphql', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    query: `
      query {
        quotes(symbols: ["AAPL", "MSFT"]) {
          symbol
          name
          price
          change
          percentChange
        }
      }
    `
  })
});

const data = await response.json();
```

## GraphQL Features

### Queries

Query operations allow you to fetch data. All queries are read-only and do not modify server state.

**Example:**
```graphql
query {
  quotes(symbols: ["AAPL", "MSFT"]) {
    symbol
    name
    price
    change
  }
}
```

See [Queries Documentation](api/graphql/api/) for complete query reference.

### Subscriptions

Subscription operations provide real-time data updates over WebSocket connections. Subscriptions automatically push updates every 5 seconds.

**Example:**
```graphql
subscription {
  quoteUpdates(symbols: ["AAPL"]) {
    symbol
    price
    change
  }
}
```

See [Subscriptions Documentation](./subscriptions.md) for complete subscription reference.

### Mutations

Currently, the API does not support mutations. All operations are read-only queries or subscriptions.

## Response Format

### Success Response

```json
{
  "data": {
    "quotes": [
      {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "price": "145.00"
      }
    ]
  }
}
```

### Error Response

```json
{
  "errors": [
    {
      "message": "Invalid symbol",
      "locations": [{"line": 2, "column": 3}],
      "path": ["quotes"]
    }
  ],
  "data": null
}
```

## Schema Introspection

You can query the schema itself using GraphQL introspection:

```graphql
query {
  __schema {
    types {
      name
      kind
    }
  }
}
```

Or use the GraphQL Playground's schema explorer to browse all available types, queries, and subscriptions.

## Rate Limiting

The GraphQL API uses the same rate limiting as the REST API:
- Rate limits are applied per IP address
- See the main API documentation for current rate limit details

## CORS

CORS is enabled for all origins, allowing browser-based clients to access the API directly.

## Next Steps

- [Queries Documentation](./queries.md) - Complete reference for all query operations
- [Subscriptions Documentation](./subscriptions.md) - Real-time data subscriptions
- [REST API Documentation](../) - Alternative REST endpoints for the same data

