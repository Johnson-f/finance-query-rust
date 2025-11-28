## News

### `news`

Get financial news (general or for a specific symbol).

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String`       |          | Ticker symbol (optional, returns general news if omitted) | `"AAPL"` |

**Example Query (General News):**
```graphql
query {
  news {
    title
    link
    source
    img
    time
  }
}
```

**Example Query (Symbol-Specific News):**
```graphql
query {
  news(symbol: "AAPL") {
    title
    link
    source
    time
  }
}
```

### `newsBySymbol`

Get news for a specific symbol (explicit symbol parameter).

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Ticker symbol                   | `"AAPL"`         |

**Example Query:**
```graphql
query {
  newsBySymbol(symbol: "AAPL") {
    title
    link
    source
    img
    time
  }
}
```