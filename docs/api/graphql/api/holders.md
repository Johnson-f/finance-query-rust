## Holders

### `majorHolders`

Get major holders breakdown for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  majorHolders(symbol: "AAPL") {
    symbol
    breakdown {
      breakdownData
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "majorHolders": {
      "symbol": "AAPL",
      "breakdown": {
        "breakdownData": {
          "institutionsCount": 7066,
          "insidersPercentHeld": 0.016970001,
          "institutionsPercentHeld": 0.64364,
          "institutionsFloatPercentHeld": 0.65475
        }
      }
    }
  }
}
```

### `institutionalHolders`

Get institutional holders for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  institutionalHolders(symbol: "AAPL") {
    symbol
    holders {
      holder
      shares
      dateReported
      percentOut
      value
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "institutionalHolders": {
      "symbol": "AAPL",
      "holders": [
        {
          "holder": "Vanguard Group Inc",
          "shares": 1399427162,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0947,
          "value": 388410991730
        },
        {
          "holder": "Blackrock Inc.",
          "shares": 1146332274,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0776,
          "value": 318164508655
        },
        {
          "holder": "State Street Corporation",
          "shares": 597501113,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0404,
          "value": 165836426619
        },
        {
          "holder": "JPMORGAN CHASE & CO",
          "shares": 473311062,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.032,
          "value": 131367479480
        },
        {
          "holder": "Geode Capital Management, LLC",
          "shares": 356166414,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0241,
          "value": 98853983857
        },
        {
          "holder": "FMR, LLC",
          "shares": 303254081,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.020499999,
          "value": 84168166479
        },
        {
          "holder": "Berkshire Hathaway, Inc",
          "shares": 238212764,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0161,
          "value": 66115949740
        },
        {
          "holder": "Morgan Stanley",
          "shares": 229103384,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0155,
          "value": 63587641432
        },
        {
          "holder": "Price (T.Rowe) Associates Inc",
          "shares": 212755053,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.014400001,
          "value": 59050162363
        },
        {
          "holder": "NORGES BANK",
          "shares": 189804820,
          "dateReported": "2025-06-30T00:00:00+00:00",
          "percentOut": 0.0128,
          "value": 52680325474
        }
      ]
    }
  }
}
```

### `mutualFundHolders`

Get mutual fund holders for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  mutualFundHolders(symbol: "AAPL") {
    symbol
    holders {
      holder
      shares
      dateReported
      percentOut
      value
    }
  }
}
```

**Response:**
```json

  "data": {
    "mutualFundHolders": {
      "symbol": "AAPL",
      "holders": [
        {
          "holder": "VANGUARD INDEX FUNDS-Vanguard Total Stock Market Index Fund",
          "shares": 467135722,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.031600002,
          "value": 129653513938
        },
        {
          "holder": "VANGUARD INDEX FUNDS-Vanguard 500 Index Fund",
          "shares": 366145920,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0248,
          "value": 101623795626
        },
        {
          "holder": "Fidelity Concord Street Trust-Fidelity 500 Index Fund",
          "shares": 187913047,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0127,
          "value": 52155263900
        },
        {
          "holder": "iShares Trust-iShares Core S&P 500 ETF",
          "shares": 182136844,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0123000005,
          "value": 50552078828
        },
        {
          "holder": "SPDR S&P 500 ETF TRUST",
          "shares": 174986129,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0117999995,
          "value": 48567397967
        },
        {
          "holder": "VANGUARD INDEX FUNDS-Vanguard Growth Index Fund",
          "shares": 140904646,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0095,
          "value": 39108082777
        },
        {
          "holder": "Invesco QQQ Trust, Series 1",
          "shares": 124773851,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0084,
          "value": 34630980821
        },
        {
          "holder": "VANGUARD INSTITUTIONAL INDEX FUNDS-Vanguard Institutional Index Fund",
          "shares": 86511134,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0058999998,
          "value": 24011164185
        },
        {
          "holder": "VANGUARD WORLD FUND-Vanguard Information Technology Index Fund",
          "shares": 66163735,
          "dateReported": "2025-08-31T00:00:00+00:00",
          "percentOut": 0.0045,
          "value": 18363743841
        },
        {
          "holder": "iShares Trust-iShares Russell 1000 Growth ETF",
          "shares": 53561374,
          "dateReported": "2025-09-30T00:00:00+00:00",
          "percentOut": 0.0036000002,
          "value": 14865958699
        }
      ]
    }
  }
}
```

### `insiderTransactions`

Get insider transactions for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  insiderTransactions(symbol: "AAPL") {
    symbol
    transactions {
      insider
      position
      transaction
      shares
      value
      startDate
      ownership
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "insiderTransactions": {
      "symbol": "AAPL",
      "transactions": [
        {
          "insider": "ADAMS KATHERINE L",
          "position": "General Counsel",
          "transaction": "Stock Gift at price 0.00 per share.",
          "shares": 3750,
          "value": 0,
          "startDate": "2025-11-12T00:00:00+00:00",
          "ownership": "D"
        },
        {
          "insider": "KONDO CHRISTOPHER",
          "position": "Officer",
          "transaction": "Sale at price 271.23 per share.",
          "shares": 3752,
          "value": 1017655,
          "startDate": "2025-11-07T00:00:00+00:00",
          "ownership": "D"
        },
        {
          "insider": "PAREKH KEVAN",
          "position": "Chief Financial Officer",
          "transaction": "Sale at price 245.89 - 248.73 per share.",
          "shares": 4199,
          "value": 1038787,
          "startDate": "2025-10-16T00:00:00+00:00",
          "ownership": "D"
        },
        {
          "insider": "KONDO CHRISTOPHER",
          "position": "Officer",
          "transaction": "",
          "shares": 7371,
          "value": null,
          "startDate": "2025-10-15T00:00:00+00:00",
          "ownership": "D"
        },
        {
          "insider": "PAREKH KEVAN",
          "position": "Chief Financial Officer",
          "transaction": "",
          "shares": 16457,
          "value": null,
          "startDate": "2025-10-15T00:00:00+00:00",
          "ownership": "D"
        },
        {
          "insider": "COOK TIMOTHY D",
          "position": "Chief Executive Officer",
          "transaction": "Sale at price 254.83 - 257.57 per share.",
          "shares": 129963,
          "value": 33375723,
          "startDate": "2025-10-02T00:00:00+00:00",
          "ownership": "D"
        },
        {
          "insider": "ADAMS KATHERINE L",
          "position": "General Counsel",
          "transaction": "Sale at price 254.83 - 257.54 per share.",
          "shares": 47125,
          "value": 12101154,
          "startDate": "2025-10-02T00:00:00+00:00",
          "ownership": "D"
        },
        {
          "insider": "O'BRIEN DEIRDRE",
          "position": "Officer",
          "transaction": "Sale at price 257.36 - 258.08 per share.",
          "shares": 43013,
          "value": 11071078,
          "startDate": "2025-10-02T00:00:00+00:00",
          "ownership": "D"
        }
 ]
    }
  }
}
```

### `insiderPurchases`

Get insider purchases summary for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  insiderPurchases(symbol: "AAPL") {
    symbol
    summary {
      period
      purchasesShares
      purchasesTransactions
      salesShares
      salesTransactions
      netShares
      netTransactions
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "insiderPurchases": {
      "symbol": "AAPL",
      "summary": {
        "period": "6m",
        "purchasesShares": null,
        "purchasesTransactions": null,
        "salesShares": null,
        "salesTransactions": null,
        "netShares": null,
        "netTransactions": null
      }
    }
  }
}
```

### `insiderRoster`

Get insider roster for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  insiderRoster(symbol: "AAPL") {
    symbol
    roster {
      name
      position
      mostRecentTransaction
      latestTransactionDate
      sharesOwnedDirectly
      sharesOwnedIndirectly
      positionDirectDate
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "insiderRoster": {
      "symbol": "AAPL",
      "roster": [
        {
          "name": "ADAMS KATHERINE L",
          "position": "General Counsel",
          "mostRecentTransaction": "Stock Gift",
          "latestTransactionDate": "2025-11-12T00:00:00+00:00",
          "sharesOwnedDirectly": 175408,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-11-12T00:00:00+00:00"
        },
        {
          "name": "COOK TIMOTHY D",
          "position": "Chief Executive Officer",
          "mostRecentTransaction": "Sale",
          "latestTransactionDate": "2025-10-02T00:00:00+00:00",
          "sharesOwnedDirectly": 3280300,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-10-02T00:00:00+00:00"
        },
        {
          "name": "KHAN SABIH",
          "position": "Chief Operating Officer",
          "mostRecentTransaction": "Conversion of Exercise of derivative security",
          "latestTransactionDate": "2025-10-01T00:00:00+00:00",
          "sharesOwnedDirectly": 1074400,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-10-01T00:00:00+00:00"
        },
        {
          "name": "KONDO CHRISTOPHER",
          "position": "Officer",
          "mostRecentTransaction": "Sale",
          "latestTransactionDate": "2025-11-07T00:00:00+00:00",
          "sharesOwnedDirectly": 15098,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-11-07T00:00:00+00:00"
        },
        {
          "name": "LEVINSON ARTHUR D",
          "position": "Director",
          "mostRecentTransaction": "Sale",
          "latestTransactionDate": "2025-08-28T00:00:00+00:00",
          "sharesOwnedDirectly": 4125580,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-08-28T00:00:00+00:00"
        },
        {
          "name": "O'BRIEN DEIRDRE",
          "position": "Officer",
          "mostRecentTransaction": "Sale",
          "latestTransactionDate": "2025-10-02T00:00:00+00:00",
          "sharesOwnedDirectly": 136687,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-10-02T00:00:00+00:00"
        },
        {
          "name": "PAREKH KEVAN",
          "position": "Chief Financial Officer",
          "mostRecentTransaction": "Sale",
          "latestTransactionDate": "2025-10-16T00:00:00+00:00",
          "sharesOwnedDirectly": 8765,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-10-16T00:00:00+00:00"
        },
        {
          "name": "SUGAR RONALD D",
          "position": "Director",
          "mostRecentTransaction": "Conversion of Exercise of derivative security",
          "latestTransactionDate": "2025-01-31T00:00:00+00:00",
          "sharesOwnedDirectly": 109311,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-01-31T00:00:00+00:00"
        },
        {
          "name": "WAGNER SUSAN L",
          "position": "Director",
          "mostRecentTransaction": "Conversion of Exercise of derivative security",
          "latestTransactionDate": "2025-01-31T00:00:00+00:00",
          "sharesOwnedDirectly": 68533,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-01-31T00:00:00+00:00"
        },
        {
          "name": "WILLIAMS JEFFREY E",
          "position": "Chief Operating Officer",
          "mostRecentTransaction": "Sale",
          "latestTransactionDate": "2025-04-02T00:00:00+00:00",
          "sharesOwnedDirectly": 390059,
          "sharesOwnedIndirectly": null,
          "positionDirectDate": "2025-04-02T00:00:00+00:00"
        }
      ]
    }
  }
}
```