# Financials

## GET /v1/financials/{symbol}

### Overview

**Purpose:** Retrieve financial statements for a given stock symbol.  
**Response Format:** A financial statement object containing the data for the requested type and frequency, grouped by metric.

### Path Parameters

| Parameter | Type   | Required | Description                     | Example |
|-----------|--------|:--------:|---------------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol         | `AAPL`  |

### Query Parameters

| Parameter   | Type          | Required | Description                               | Example    |
|-------------|---------------|:--------:|-------------------------------------------|------------|
| `statement` | StatementType |    ✓     | The type of statement (`income`, `balance`, `cashflow`) | `income`   |
| `frequency` | Frequency     |          | The frequency of the report (`annual`, `quarterly`). Defaults to `annual`. | `annual`   |

### Responses

- **200 OK**  
  - **Content-Type:** `application/json`  
  - **Schema:** [`FinancialStatement`](#financialstatement-schema)
  - **Example (200):**
    ```json
    {
      "symbol": "AAPL",
      "statement_type": "income",
      "frequency": "annual",
      "statement": {
        "TotalRevenue": {
          "2023-09-30": 383285000000,
          "2022-09-30": 394328000000
        },
        "NetIncome": {
          "2023-09-30": 96995000000,
          "2022-09-30": 99803000000
        }
      }
    }
    ```

- **404 Not Found**  
  ```json
  { "detail": "No data found for SYMBOL" }
  ```

- **422 Unprocessable Entity**  
  ```json
  {
    "detail": "Invalid statement type"
  }
  ```

## Schema References

### FinancialStatement Schema

| Field          | Type                             | Description                                                                    | Required |
|----------------|----------------------------------|--------------------------------------------------------------------------------|:--------:|
| symbol         | string                           | Stock symbol (e.g., "AAPL")                                                    |    ✓     |
| statement_type | string                           | Type of financial statement                                                    |    ✓     |
| frequency      | string                           | Frequency of the financial statement                                           |    ✓     |
| statement      | object                           | Map of Metric Name -> Map of Date -> Value                                     |    ✓     |

### StatementType (Enum)
- `income` (or `income_statement`)
- `balance` (or `balance_sheet`)
- `cashflow` (or `cash_flow`)

### Frequency (Enum)
- `annual` (or `yearly`)
- `quarterly` (or `quarter`)

