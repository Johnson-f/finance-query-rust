## Indices

### `indices`

TODO: Add region & name filtering 

Get US market indices (DJIA, NASDAQ, S&P 500).

# Short response
**Example Query:**
```graphql
query {
  indices {
    name
    value
    change
    percentChange
  }
}
```

**Response:**
```json

```

# Long response
**Example Query:**
```graphql
query {
  indices {
    name
    value
    change
    percentChange
    fiveDaysReturn
    oneMonthReturn
    threeMonthReturn
    sixMonthReturn
    ytdReturn
    yearReturn
    threeYearReturn
    fiveYearReturn
    tenYearReturn
    maxReturn
  }
}
```