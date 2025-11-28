## Quotes

### `quotes`

Retrieve comprehensive quote data for multiple stocks.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbols` | `[String!]!`   |    ✓     | Array of stock ticker symbols   | `["AAPL", "MSFT"]` |

**Example Query:**
```graphql
query {
  quotes(symbols: ["AAPL", "MSFT", "GOOG"]) {
    symbol
    name
    price
    change
    percentChange
    open
    high
    low
    volume
    marketCap
    pe
    eps
    sector
    industry
  }
}
```

**Response:**
```json
{
  "data": {
    "quotes": [
      {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "price": "145.00",
        "change": "+1.00",
        "percentChange": "+0.69%",
        "open": "144.00",
        "high": "146.00",
        "low": "143.00",
        "volume": 1000000,
        "marketCap": "2.5T",
        "pe": "30.00",
        "eps": "4.50",
        "sector": "Technology",
        "industry": "Consumer Electronics"
      }
    ]
  }
}
```

### `simpleQuotes`

Retrieve simplified quote data for multiple stocks (basic price and change information).

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbols` | `[String!]!`   |    ✓     | Array of stock ticker symbols   | `["AAPL", "MSFT"]` |

**Example Query:**
```graphql
query {
  simpleQuotes(symbols: ["TSLA", "NVDA"]) {
    symbol
    name
    price
    change
    percentChange
    logo
  }
}
```

### `detailedQuotes`

Retrieve comprehensive quote data with camelCase field names (legacy format support).

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbols` | `[String!]!`   |    ✓     | Array of stock ticker symbols   | `["AAPL", "MSFT"]` |

**Example Query:**
```graphql
query {
  detailedQuotes(symbols: ["AAPL"]) {
    symbol
    name
    price
    preMarketPrice
    afterHoursPrice
    change
    percentChange
    yearHigh
    yearLow
    marketCap
    open
    high
    low
    volume
    avgVolume
    beta
    pe
    eps
    dividend
    expenseRatio
    category
    lastCapitalGain
    morningstarRating
    morningstarRiskRating
    holdingsTurnover
    earningsDate
    lastDividend
    inceptionDate
    sector
    industry
    about
    employees
    fiveDaysReturn
    oneMonthReturn
    three_month_return
    six_month_return
    ytd_return
    year_return
    threeYearReturn
    fiveYearReturn
    tenYearReturn
    maxReturn
    logo
  }
}
```

**Response:**
```json
{
  "data": {
    "detailedQuotes": [
      {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "price": "277.55",
        "preMarketPrice": "278.87",
        "afterHoursPrice": "277.90",
        "change": "0.58",
        "percentChange": "0.21%",
        "yearHigh": "280.38",
        "yearLow": "169.21",
        "marketCap": "4.12T"
      }
    ]
  }
}
```

### `similar`

Find stocks similar to a specific ticker.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Base stock for comparison       | `"AAPL"`         |
| `limit`  | `Int`          |          | Maximum results (default: 10, max: 20) | `15`     |

**Example Query:**
```graphql
query {
  similar(symbol: "AAPL", limit: 15) {
    symbol
    name
    price
    change
    percentChange
  }
}
```