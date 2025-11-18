# Analysis API

## GET /v1/analysis/{symbol}/recommendations

### Overview

**Purpose:** Retrieve analyst recommendations for a stock  
**Response Format:** Object containing recommendations data

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`RecommendationsResponse`](#recommendationsresponse-schema)
  - **Example (200):**
    ```json
    {
      "symbol": "AAPL",
      "recommendations": [
        {
          "period": "0m",
          "strongBuy": 10,
          "buy": 20,
          "hold": 5,
          "sell": 0,
          "strongSell": 0
        }
      ]
    }
    ```

## GET /v1/analysis/{symbol}/upgrades-downgrades

### Overview

**Purpose:** Retrieve recent analyst upgrades and downgrades  
**Response Format:** Object containing upgrades/downgrades list

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`UpgradesDowngradesResponse`](#upgradesdowngradesresponse-schema)

## GET /v1/analysis/{symbol}/price-targets

### Overview

**Purpose:** Retrieve analyst price targets  
**Response Format:** Object containing price target statistics

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`PriceTargetsResponse`](#pricetargetsresponse-schema)

## GET /v1/analysis/{symbol}/earnings-estimate

### Overview

**Purpose:** Retrieve earnings estimates  
**Response Format:** Object containing earnings estimates

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`EarningsEstimateResponse`](#earningsestimateresponse-schema)

## GET /v1/analysis/{symbol}/revenue-estimate

### Overview

**Purpose:** Retrieve revenue estimates  
**Response Format:** Object containing revenue estimates

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`RevenueEstimateResponse`](#revenueestimateresponse-schema)

## GET /v1/analysis/{symbol}/earnings-history

### Overview

**Purpose:** Retrieve historical earnings data  
**Response Format:** Object containing earnings history

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`EarningsHistoryResponse`](#earningshistoryresponse-schema)

## Schema References

### RecommendationsResponse Schema

| Field           | Type                                        | Description             | Required |
|-----------------|---------------------------------------------|-------------------------|:--------:|
| symbol          | string                                      | Stock symbol            |    ✓     |
| recommendations | [RecommendationData[]](#recommendationdata) | List of recommendations |    ✓     |

### RecommendationData Schema

| Field      | Type   | Description                          | Required |
|------------|--------|--------------------------------------|:--------:|
| period     | string | Time period (e.g., "0m", "-1m")      |    ✓     |
| strongBuy  | int    | Number of strong buy recommendations |          |
| buy        | int    | Number of buy recommendations        |          |
| hold       | int    | Number of hold recommendations       |          |
| sell       | int    | Number of sell recommendations       |          |
| strongSell | int    | Number of strong sell recommendations|          |

### UpgradesDowngradesResponse Schema

| Field              | Type                                    | Description           | Required |
|--------------------|-----------------------------------------|-----------------------|:--------:|
| symbol             | string                                  | Stock symbol          |    ✓     |
| upgradesDowngrades | [UpgradeDowngrade[]](#upgradedowngrade) | List of action items  |    ✓     |

### UpgradeDowngrade Schema

| Field     | Type     | Description                   | Required |
|-----------|----------|-------------------------------|:--------:|
| firm      | string   | Analyst firm                  |    ✓     |
| toGrade   | string   | New grade                     |          |
| fromGrade | string   | Previous grade                |          |
| action    | string   | Action (up, down, main, init) |          |
| date      | datetime | Date of action                |          |

### PriceTargetsResponse Schema

| Field        | Type                        | Description  | Required |
|--------------|-----------------------------|--------------|:--------:|
| symbol       | string                      | Stock symbol |    ✓     |
| priceTargets | [PriceTarget](#pricetarget) | Price targets|    ✓     |

### PriceTarget Schema

| Field   | Type  | Description   | Required |
|---------|-------|---------------|:--------:|
| current | float | Current price |          |
| mean    | float | Mean target   |          |
| median  | float | Median target |          |
| low     | float | Low target    |          |
| high    | float | High target   |          |

### EarningsEstimateResponse Schema

| Field            | Type                                  | Description       | Required |
|------------------|---------------------------------------|-------------------|:--------:|
| symbol           | string                                | Stock symbol      |    ✓     |
| earningsEstimate | [EarningsEstimate](#earningsestimate) | Estimate data     |    ✓     |

### EarningsEstimate Schema

| Field     | Type   | Description                  | Required |
|-----------|--------|------------------------------|:--------:|
| estimates | object | Map of period -> Estimate    |    ✓     |

### RevenueEstimateResponse Schema

| Field           | Type                                | Description       | Required |
|-----------------|-------------------------------------|-------------------|:--------:|
| symbol          | string                              | Stock symbol      |    ✓     |
| revenueEstimate | [RevenueEstimate](#revenueestimate) | Estimate data     |    ✓     |

### RevenueEstimate Schema

| Field     | Type   | Description                  | Required |
|-----------|--------|------------------------------|:--------:|
| estimates | object | Map of period -> Estimate    |    ✓     |

### EarningsHistoryResponse Schema

| Field           | Type                                          | Description | Required |
|-----------------|-----------------------------------------------|-------------|:--------:|
| symbol          | string                                        | Stock symbol|    ✓     |
| earningsHistory | [EarningsHistoryItem[]](#earningshistoryitem) | History list|    ✓     |

### EarningsHistoryItem Schema

| Field           | Type     | Description                  | Required |
|-----------------|----------|------------------------------|:--------:|
| date            | datetime | Report date                  |    ✓     |
| epsActual       | float    | Actual EPS                   |          |
| epsEstimate     | float    | Estimated EPS                |          |
| surprise        | float    | EPS Surprise                 |          |
| surprisePercent | float    | Surprise %                   |          |

