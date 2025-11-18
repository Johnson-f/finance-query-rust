# Search

## GET /v1/search

### Overview

**Purpose:** Search for stocks by company name or symbol  
**Response Format:** Object containing a list of matching securities

### Request Parameters

| Parameter | Type    | Required | Description                               | Example  |
|-----------|---------|:--------:|-------------------------------------------|----------|
| `q`       | string  |    ✓     | Partial or full company name or symbol    | `Apple`  |
| `hits`    | integer |          | Number of results to return (default: 6)  | `10`     |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`SearchResponse`](#searchresponse-schema) object
  - **Example (200):**
    ```json
    {
      "results": [
        {
          "symbol": "AAPL",
          "name": "Apple Inc.",
          "exchange": "NASDAQ",
          "quote_type": "EQUITY"
        }
      ]
    }
    ```

## Schema References

### SearchResponse Schema

| Field   | Type                        | Description         | Required |
|---------|-----------------------------|---------------------|:--------:|
| results | Array of [`SearchResult`]   | List of matches     |    ✓     |

### SearchResult Schema

| Field      | Type   | Description                                     | Required |
|------------|--------|-------------------------------------------------|:--------:|
| symbol     | string | Stock symbol (e.g., "AAPL")                     |    ✓     |
| name       | string | Full company name (e.g., "Apple Inc.")          |    ✓     |
| exchange   | string | Exchange where security trades (e.g., "NASDAQ") |          |
| quote_type | string | Security type (e.g., "EQUITY", "ETF")           |          |

