# GraphQL Queries

## Combining Queries

GraphQL allows you to combine multiple queries in a single request:

```graphql
query {
  quotes(symbols: ["AAPL", "MSFT"]) {
    symbol
    name
    price
  }
  indices {
    symbol
    name
    price
  }
  actives {
    symbol
    name
    price
  }
}
```

This returns all three datasets in a single response, reducing network round trips.

