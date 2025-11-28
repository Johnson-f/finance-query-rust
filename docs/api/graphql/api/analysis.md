## Analysts

### `recommendations`

Get analyst recommendations for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  recommendations(symbol: "AAPL") {
    symbol
    recommendations {
      period
      strongBuy
      buy
      hold
      sell
      strongSell
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "recommendations": {
      "symbol": "AAPL",
      "recommendations": [
        {
          "period": "0m",
          "strongBuy": 5,
          "buy": 24,
          "hold": 15,
          "sell": 1,
          "strongSell": 3
        },
        {
          "period": "-1m",
          "strongBuy": 5,
          "buy": 24,
          "hold": 15,
          "sell": 1,
          "strongSell": 3
        },
        {
          "period": "-2m",
          "strongBuy": 5,
          "buy": 23,
          "hold": 15,
          "sell": 1,
          "strongSell": 3
        },
        {
          "period": "-3m",
          "strongBuy": 5,
          "buy": 22,
          "hold": 15,
          "sell": 1,
          "strongSell": 1
        }
      ]
    }
  }
}
```

### `upgradesDowngrades`

Get recent analyst upgrades and downgrades.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  upgradesDowngrades(symbol: "AAPL") {
    symbol
    upgradesDowngrades {
      firm
      toGrade
      fromGrade
      action
      date
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "upgradesDowngrades": {
      "symbol": "AAPL",
      "upgradesDowngrades": [
        {
          "firm": "Rosenblatt",
          "toGrade": "Neutral",
          "fromGrade": "Neutral",
          "action": "main",
          "date": "2025-11-04T13:18:21+00:00"
        },
        {
          "firm": "Wells Fargo",
          "toGrade": "Overweight",
          "fromGrade": "Overweight",
          "action": "main",
          "date": "2025-10-31T21:07:13+00:00"
        },
        ...
      ]
    }
  }
}
```

### `priceTargets`

Get analyst price targets for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  priceTargets(symbol: "AAPL") {
    symbol
    priceTargets {
      current
      mean
      median
      low
      high
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "priceTargets": {
      "symbol": "AAPL",
      "priceTargets": {
        "current": 277.55,
        "mean": 281.74805,
        "median": 280,
        "low": 215,
        "high": 345
      }
    }
  }
}
```

### `earningsEstimate`

Get earnings estimates for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  earningsEstimate(symbol: "AAPL") {
    symbol
    earningsEstimate {
      estimates
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "earningsEstimate": {
      "symbol": "AAPL",
      "earningsEstimate": {
        "estimates": {
          "+1y": {
            "avg": 9.10215,
            "growth": 0.1029,
            "high": 10.1,
            "low": 8.3,
            "numberOfAnalysts": 38,
            "yearAgoEps": 8.25269
          },
          "+1q": {
            "avg": 1.84361,
            "growth": 0.1173,
            "high": 2.15,
            "low": 1.7,
            "numberOfAnalysts": 28,
            "yearAgoEps": 1.65
          },
          "0y": {
            "avg": 8.25269,
            "growth": 0.106300004,
            "high": 9,
            "low": 7.76,
            "numberOfAnalysts": 40,
            "yearAgoEps": 7.46
          },
          "0q": {
            "avg": 2.66315,
            "growth": 0.1096,
            "high": 2.8,
            "low": 2.51,
            "numberOfAnalysts": 30,
            "yearAgoEps": 2.4
          }
        }
      }
    }
  }
}
```

### `revenueEstimate`

Get revenue estimates for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  revenueEstimate(symbol: "AAPL") {
    symbol
    revenueEstimate {
      estimates
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "revenueEstimate": {
      "symbol": "AAPL",
      "revenueEstimate": {
        "estimates": {
          "0y": {
            "avg": 452933853880,
            "growth": 0.0884,
            "high": 469000000000,
            "low": 437285000000,
            "numberOfAnalysts": 39,
            "yearAgoRevenue": 416161000000
          },
          "+1q": {
            "avg": 104521197940,
            "growth": 0.096099995,
            "high": 109162000000,
            "low": 98910000000,
            "numberOfAnalysts": 28,
            "yearAgoRevenue": 95359000000
          },
          "0q": {
            "avg": 138015309140,
            "growth": 0.1103,
            "high": 140666000000,
            "low": 136679500000,
            "numberOfAnalysts": 29,
            "yearAgoRevenue": 124300000000
          },
          "+1y": {
            "avg": 481176069850,
            "growth": 0.0624,
            "high": 526330884300,
            "low": 444291000000,
            "numberOfAnalysts": 40,
            "yearAgoRevenue": 452933853880
          }
        }
      }
    }
  }
}
```

### `earningsHistory`

Get earnings history for a stock.

**Arguments:**

| Argument | Type           | Required | Description                     | Example          |
|----------|----------------|:--------:|---------------------------------|------------------|
| `symbol` | `String!`      |    ✓     | Stock ticker symbol             | `"AAPL"`         |

**Example Query:**
```graphql
query {
  earningsHistory(symbol: "AAPL") {
    symbol
    earningsHistory {
      date
      epsActual
      epsEstimate
      surprise
      surprisePercent
    }
  }
}
```

**Response:**
```json
{
  "data": {
    "earningsHistory": {
      "symbol": "AAPL",
      "earningsHistory": [
        {
          "date": "2024-12-31T00:00:00+00:00",
          "epsActual": 2.4,
          "epsEstimate": 2.34102,
          "surprise": 0.06,
          "surprisePercent": 0.0252
        },
        {
          "date": "2025-03-31T00:00:00+00:00",
          "epsActual": 1.65,
          "epsEstimate": 1.62253,
          "surprise": 0.03,
          "surprisePercent": 0.016900001
        },
        {
          "date": "2025-06-30T00:00:00+00:00",
          "epsActual": 1.57,
          "epsEstimate": 1.42572,
          "surprise": 0.14,
          "surprisePercent": 0.1012
        },
        {
          "date": "2025-09-30T00:00:00+00:00",
          "epsActual": 1.85,
          "epsEstimate": 1.76993,
          "surprise": 0.08,
          "surprisePercent": 0.0452
        }
      ]
    }
  }
}
```