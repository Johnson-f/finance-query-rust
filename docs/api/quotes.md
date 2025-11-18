# Quotes

## GET /v1/quotes

### Overview

**Purpose:** Retrieve comprehensive quote data for multiple stocks  
**Response Format:** Detailed stock information with all available fields (snake_case keys)

### Request Parameters

| Parameter | Type   | Required | Description                     | Example          |
|-----------|--------|:--------:|---------------------------------|------------------|
| `symbols` | string |    ✓     | Comma-separated list of tickers | `AAPL,MSFT,GOOG` |

**Responses:**

- **200 OK**  
  - **Content-Type:** `application/json`  
  - **Schema:** Array of [`Quote`](#quote-schema) objects  
  - **Example (200):**
    ```json
    [
      {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "price": "145.00",
        "pre_market_price": "145.50",
        "after_hours_price": "145.50",
        "change": "+1.00",
        "percent_change": "+0.69%",
        "open": "144.00",
        "high": "146.00",
        "low": "143.00",
        "year_high": "150.00",
        "year_low": "100.00",
        "volume": 1000000,
        "avg_volume": 2000000,
        "market_cap": "2.5T",
        "beta": "1.23",
        "pe": "30.00",
        "eps": "4.50",
        "dividend": "0.82",
        "dividend_yield": "1.3%",
        "ex_dividend": "Feb 5, 2024",
        "net_assets": "10.5B",
        "nav": "100.00",
        "expense_ratio": "0.05%",
        "category": "Large Growth",
        "last_capital_gain": "10.00",
        "morningstar_rating": "★★",
        "morningstar_risk_rating": "Low",
        "holdings_turnover": "5.00%",
        "earnings_date": "Apr 23, 2024",
        "last_dividend": "0.82",
        "inception_date": "Jan 1, 2020",
        "sector": "Technology",
        "industry": "Consumer Electronics",
        "about": "Apple Inc. designs, manufactures, and markets...",
        "employees": "150,000",
        "five_days_return": "-19.35%",
        "one_month_return": "-28.48%",
        "three_month_return": "-14.02%",
        "six_month_return": "36.39%",
        "ytd_return": "+10.00%",
        "year_return": "+20.00%",
        "three_year_return": "+30.00%",
        "five_year_return": "+40.00%",
        "ten_year_return": "2,005.31%",
        "max_return": "22,857.89%",
        "logo": "https://img.logo.dev/apple.com?token=..."
      }
    ]
    ```

## GET /v1/detailed-quotes

### Overview

**Purpose:** Retrieve comprehensive quote data for multiple stocks with camelCase keys (Legacy support)  
**Response Format:** Detailed stock information with all available fields

### Request Parameters

| Parameter | Type   | Required | Description                     | Example          |
|-----------|--------|:--------:|---------------------------------|------------------|
| `symbols` | string |    ✓     | Comma-separated list of tickers | `AAPL,MSFT,GOOG` |

**Responses:**

- **200 OK**  
  - **Content-Type:** `application/json`  
  - **Schema:** Array of [`DetailedQuote`](#detailedquote-schema) objects  
  - **Example (200):**
    ```json
    [
      {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "price": "145.00",
        "preMarketPrice": "145.50",
        "afterHoursPrice": "145.50",
        "change": "+1.00",
        "percentChange": "+0.69%",
        "open": "144.00",
        "high": "146.00",
        "low": "143.00",
        "yearHigh": "150.00",
        "yearLow": "100.00",
        "volume": 1000000,
        "avgVolume": 2000000,
        "marketCap": "2.5T",
        "beta": "1.23",
        "pe": "30.00",
        "eps": "4.50",
        "dividend": "0.82",
        "yield": "1.3%",
        "exDividend": "Feb 5, 2024",
        "netAssets": "10.5B",
        "nav": "100.00",
        "expenseRatio": "0.05%",
        "category": "Large Growth",
        "lastCapitalGain": "10.00",
        "morningstarRating": "★★",
        "morningstarRiskRating": "Low",
        "holdingsTurnover": "5.00%",
        "earningsDate": "Apr 23, 2024",
        "lastDividend": "0.82",
        "inceptionDate": "Jan 1, 2020",
        "sector": "Technology",
        "industry": "Consumer Electronics",
        "about": "Apple Inc. designs, manufactures, and markets...",
        "employees": "150,000",
        "fiveDaysReturn": "-19.35%",
        "oneMonthReturn": "-28.48%",
        "threeMonthReturn": "-14.02%",
        "sixMonthReturn": "36.39%",
        "ytdReturn": "+10.00%",
        "yearReturn": "+20.00%",
        "threeYearReturn": "+30.00%",
        "fiveYearReturn": "+40.00%",
        "tenYearReturn": "2,005.31%",
        "maxReturn": "22,857.89%",
        "logo": "https://img.logo.dev/apple.com?token=..."
      }
    ]
    ```

## GET /v1/simple-quotes

### Overview

**Purpose:** Retrieve simplified quote data for multiple stocks  
**Response Format:** Basic stock information including symbols, names, prices, and changes

### Request Parameters

| Parameter | Type   | Required | Description                     | Example          |
|-----------|--------|:--------:|---------------------------------|------------------|
| `symbols` | string |    ✓     | Comma-separated list of tickers | `AAPL,MSFT,GOOG` |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** Array of [`SimpleQuote`](#simplequote-schema) objects.
  - **Example (200):**
    ```json
    [
      {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "price": "145.00",
        "change": "+1.00",
        "percent_change": "+0.69%",
        "logo": "https://img.logo.dev/apple.com?token=…"
      }
    ]
    ```

## GET /v1/similar

### Overview

**Purpose:** Find stocks similar to a specific ticker  
**Response Format:** List of comparable stocks with simplified quote data

### Request Parameters

| Parameter | Type    | Required | Description                   | Example  |
|-----------|---------|:--------:|-------------------------------|----------|
| `symbol`  | string  |    ✓     | Base stock for comparison     | `AAPL`   |
| `limit`   | integer |          | Maximum results (default: 10) | `15`     |

**Note:** Limit parameter accepts values between 1 and 20.

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** Array of [`SimpleQuote`](#simplequote-schema) objects.

## Schema References

### Quote Schema

| Field                 | Type    | Description                                | Required |
|-----------------------|---------|--------------------------------------------|:--------:|
| symbol                | string  | Stock symbol                               |    ✓     |
| name                  | string  | Company name                               |    ✓     |
| price                 | string  | Last traded price                          |    ✓     |
| pre_market_price      | string  | Pre-market price                           |          |
| after_hours_price     | string  | After-hours price                          |          |
| change                | string  | Change in price                            |    ✓     |
| percent_change        | string  | Percentage change                          |    ✓     |
| open                  | string  | Opening price                              |          |
| high                  | string  | Highest price                              |          |
| low                   | string  | Lowest price                               |          |
| year_high             | string  | 52-week high price                         |          |
| year_low              | string  | 52-week low price                          |          |
| volume                | integer | Volume traded                              |          |
| avg_volume            | integer | Average volume                             |          |
| market_cap            | string  | Market capitalization                      |          |
| beta                  | string  | Beta                                       |          |
| pe                    | string  | P/E ratio                                  |          |
| eps                   | string  | Earnings per share                         |          |
| dividend              | string  | Dividend value                             |          |
| dividend_yield        | string  | Dividend yield %                           |          |
| ex_dividend           | string  | Ex-dividend date                           |          |
| net_assets            | string  | Net assets                                 |          |
| nav                   | string  | Net asset value                            |          |
| expense_ratio         | string  | Expense ratio                              |          |
| category              | string  | Fund category                              |          |
| last_capital_gain     | string  | Last capital gain                          |          |
| morningstar_rating    | string  | Morningstar rating                         |          |
| morningstar_risk_rating| string | Morningstar risk rating                    |          |
| holdings_turnover     | string  | Holdings turnover                          |          |
| earnings_date         | string  | Earnings date                              |          |
| last_dividend         | string  | Last dividend                              |          |
| inception_date        | string  | Inception date                             |          |
| sector                | string  | Sector                                     |          |
| industry              | string  | Industry                                   |          |
| about                 | string  | Description                                |          |
| employees             | string  | Employees count                            |          |
| five_days_return      | string  | 5-day return                               |          |
| one_month_return      | string  | 1-month return                             |          |
| three_month_return    | string  | 3-month return                             |          |
| six_month_return      | string  | 6-month return                             |          |
| ytd_return            | string  | YTD return                                 |          |
| year_return           | string  | 1-year return                              |          |
| three_year_return     | string  | 3-year return                              |          |
| five_year_return      | string  | 5-year return                              |          |
| ten_year_return       | string  | 10-year return                             |          |
| max_return            | string  | Max return                                 |          |
| logo                  | string  | Logo URL                                   |          |

### SimpleQuote Schema

| Field            | Type   | Description       | Required |
|------------------|--------|-------------------|:--------:|
| symbol           | string | Stock symbol      |    ✓     |
| name             | string | Company name      |    ✓     |
| price            | string | Last traded price |    ✓     |
| pre_market_price | string | Pre-market price  |          |
| after_hours_price| string | After-hours price |          |
| change           | string | Change in price   |    ✓     |
| percent_change   | string | Percentage change |    ✓     |
| logo             | string | Logo URL          |          |

### DetailedQuote Schema

(Same fields as Quote Schema but with camelCase keys, e.g., `preMarketPrice`, `yearHigh`, `dividendYield` mapped to `yield`, etc. matching the legacy API format.)

