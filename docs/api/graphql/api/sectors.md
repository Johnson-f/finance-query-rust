## Sectors

### `sectors`

Get all sector performance data.

**Example Query:**
```graphql
query {
  sectors {
    name
    change
    percentChange
    lastUpdated
  }
}
```

### `sectorForSymbol`

Get sector information for a specific symbol.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  sectorForSymbol(symbol: "AAPL") {
    name
    change
    percentChange
    lastUpdated
  }
}
```

### `sectorDetails`

Get detailed sector performance data.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `sector` | `String!`      |    ✓     | Sector name                     | `"Technology"`   |

**Example Query:**
```graphql
query {
  sectorDetails(sector: "Technology") {
    name
    change
    percentChange
    lastUpdated
    components {
      symbol
      name
      price
      change
    }
  }
}
```