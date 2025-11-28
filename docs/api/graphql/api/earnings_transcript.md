## Earnings Transcript not working yet

### `earningsCalls`

Get list of earnings calls for a symbol.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  earningsCalls(symbol: "AAPL") {
    symbol
    total
    earningsCalls {
      eventId
      quarter
      year
      title
      date
    }
  }
}
```

### `earningsTranscript`

Get earnings call transcript for a specific quarter.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |
| `quarter`| `String`       |          | Quarter (`"Q1"`, `"Q2"`, `"Q3"`, `"Q4"`) | `"Q1"`    |
| `year`   | `Int`          |          | Year                            | `2024`           |

**Example Query:**
```graphql
query {
  earningsTranscript(symbol: "AAPL", quarter: "Q1", year: 2024) {
    symbol
    quarter
    year
    title
    date
    participants {
      name
      role
    }
    transcript {
      speaker
      text
    }
  }
}
```