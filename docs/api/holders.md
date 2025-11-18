# Holders

## GET /v1/holders/{symbol}/major

### Overview

**Purpose:** Retrieve major holders breakdown for a stock  
**Response Format:** Object containing breakdown metrics

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`MajorHoldersResponse`](#majorholdersresponse-schema)
  - **Example (200):**
    ```json
    {
      "symbol": "AAPL",
      "breakdown": {
        "breakdownData": {
          "institutionsPercentHeld": 0.595,
          "insidersPercentHeld": 0.0007,
          "institutionsFloatPercentHeld": 0.596,
          "institutionsCount": 5743
        }
      }
    }
    ```

## GET /v1/holders/{symbol}/institutional

### Overview

**Purpose:** Retrieve institutional holders for a stock  
**Response Format:** Object containing list of institutional holders

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`InstitutionalHoldersResponse`](#institutionalholdersresponse-schema)
  - **Example (200):**
    ```json
    {
      "symbol": "AAPL",
      "holders": [
        {
          "holder": "Vanguard Group Inc",
          "shares": 1311658000,
          "dateReported": "2024-03-31T00:00:00Z",
          "percentOut": 8.44,
          "value": 224234567000
        }
      ]
    }
    ```

## GET /v1/holders/{symbol}/mutualfund

### Overview

**Purpose:** Retrieve mutual fund holders for a stock  
**Response Format:** Object containing list of mutual fund holders

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`MutualFundHoldersResponse`](#mutualfundholdersresponse-schema)

## GET /v1/holders/{symbol}/insider-transactions

### Overview

**Purpose:** Retrieve insider transactions for a stock  
**Response Format:** Object containing list of transactions

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`InsiderTransactionsResponse`](#insidertransactionsresponse-schema)

## GET /v1/holders/{symbol}/insider-purchases

### Overview

**Purpose:** Retrieve insider purchase activity summary  
**Response Format:** Object containing summary statistics

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`InsiderPurchasesResponse`](#insiderpurchasesresponse-schema)

## GET /v1/holders/{symbol}/insider-roster

### Overview

**Purpose:** Retrieve current insider roster  
**Response Format:** Object containing list of roster members

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`InsiderRosterResponse`](#insiderrosterresponse-schema)

## Schema References

### MajorHoldersResponse Schema

| Field     | Type                                            | Description             | Required |
|-----------|-------------------------------------------------|-------------------------|:--------:|
| symbol    | string                                          | Stock symbol            |    ✓     |
| breakdown | [MajorHoldersBreakdown](#majorholdersbreakdown) | Major holders breakdown |    ✓     |

### MajorHoldersBreakdown Schema

| Field         | Type   | Description                             | Required |
|---------------|--------|-----------------------------------------|:--------:|
| breakdownData | object | Key-value pairs of major holder metrics |    ✓     |

### InstitutionalHoldersResponse Schema

| Field   | Type                                          | Description                   | Required |
|---------|-----------------------------------------------|-------------------------------|:--------:|
| symbol  | string                                        | Stock symbol                  |    ✓     |
| holders | [InstitutionalHolder[]](#institutionalholder) | List of institutional holders |    ✓     |

### InstitutionalHolder Schema

| Field        | Type     | Description                      | Required |
|--------------|----------|----------------------------------|:--------:|
| holder       | string   | Institution name                 |    ✓     |
| shares       | integer  | Number of shares held            |    ✓     |
| dateReported | datetime | Date of last report              |    ✓     |
| percentOut   | number   | Percentage of outstanding shares |          |
| value        | integer  | Value of holdings                |          |

### MutualFundHoldersResponse Schema

| Field   | Type                                    | Description                 | Required |
|---------|-----------------------------------------|-----------------------------|:--------:|
| symbol  | string                                  | Stock symbol                |    ✓     |
| holders | [MutualFundHolder[]](#mutualfundholder) | List of mutual fund holders |    ✓     |

### MutualFundHolder Schema

(Same fields as InstitutionalHolder)

### InsiderTransactionsResponse Schema

| Field        | Type                                    | Description                  | Required |
|--------------|-----------------------------------------|------------------------------|:--------:|
| symbol       | string                                  | Stock symbol                 |    ✓     |
| transactions | [InsiderTransaction[]](#insidertransaction) | List of insider transactions |    ✓     |

### InsiderTransaction Schema

| Field       | Type     | Description                      | Required |
|-------------|----------|----------------------------------|:--------:|
| startDate   | datetime | Transaction start date           |    ✓     |
| insider     | string   | Insider name                     |    ✓     |
| position    | string   | Insider position/relation        |    ✓     |
| transaction | string   | Transaction description          |    ✓     |
| shares      | integer  | Number of shares                 |          |
| value       | integer  | Transaction value                |          |
| ownership   | string   | Ownership type (direct/indirect) |          |

### InsiderPurchasesResponse Schema

| Field   | Type                                | Description             | Required |
|---------|-------------------------------------|-------------------------|:--------:|
| symbol  | string                              | Stock symbol            |    ✓     |
| summary | [InsiderPurchase](#insiderpurchase) | Insider purchase summary|    ✓     |

### InsiderPurchase Schema

| Field                     | Type    | Description                     | Required |
|---------------------------|---------|---------------------------------|:--------:|
| period                    | string  | Time period                     |    ✓     |
| purchasesShares           | integer | Shares purchased                |          |
| purchasesTransactions     | integer | Number of purchase transactions |          |
| salesShares               | integer | Shares sold                     |          |
| salesTransactions         | integer | Number of sale transactions     |          |
| netShares                 | integer | Net shares purchased/sold       |          |
| netTransactions           | integer | Net transactions                |          |
| totalInsiderShares        | integer | Total insider shares held       |          |
| netPercentInsiderShares   | number  | Net % of insider shares         |          |
| buyPercentInsiderShares   | number  | % buy shares                    |          |
| sellPercentInsiderShares  | number  | % sell shares                   |          |

### InsiderRosterResponse Schema

| Field  | Type                                      | Description    | Required |
|--------|-------------------------------------------|----------------|:--------:|
| symbol | string                                    | Stock symbol   |    ✓     |
| roster | [InsiderRosterMember[]](#insiderrostermember) | Insider roster |    ✓     |

### InsiderRosterMember Schema

| Field                 | Type     | Description             | Required |
|-----------------------|----------|-------------------------|:--------:|
| name                  | string   | Insider name            |    ✓     |
| position              | string   | Position/relation       |    ✓     |
| mostRecentTransaction | string   | Most recent transaction |          |
| latestTransactionDate | datetime | Latest transaction date |          |
| sharesOwnedDirectly   | integer  | Shares owned directly   |          |
| sharesOwnedIndirectly | integer  | Shares owned indirectly |          |
| positionDirectDate    | datetime | Position direct date    |          |

