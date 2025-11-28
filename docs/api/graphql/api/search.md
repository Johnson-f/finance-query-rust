## Search

### `search`

Search for stocks by symbol or company name.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `query`  | `String!`      |    ✓     | Search query                    | `"Apple"`        |
| `hits`   | `Int`          |          | Maximum results (default: 6)    | `10`             |

**Example Query:**
```graphql
query {
  search(query: "Apple", hits: 10) {
    results {
      symbol
      name
      exchange
      type
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "search": {
      "results": [
        {
          "symbol": "AAPL",
          "name": "Apple Inc.",
          "exchange": "NMS",
          "type": "EQUITY"
        }
      ]
    }
  }
}
```