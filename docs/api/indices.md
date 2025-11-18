# Indices

## GET /v1/indices

### Overview

**Purpose:** Get major world market indices performance  
**Response Format:** Array of market indices with performance metrics and returns

### Request Parameters

| Parameter | Type   | Required | Description                                          | Example                           |
|-----------|--------|:--------:|------------------------------------------------------|-----------------------------------|
| `index`   | string |          | Comma-separated list of indices to include           | `snp,djia,nasdaq`                 |
| `region`  | string |          | Filter indices by region                             | `US`                              |

#### Available Region Options

`US`, `NA`, `SA`, `EU`, `AS`, `AF`, `ME`, `OCE`, `global`

#### Available Index Options

- **US:** `snp`, `djia`, `nasdaq`, `nyse-composite`, `nyse-amex`, `rut`, `vix`
- **North America:** `tsx-composite`
- **South America:** `ibovespa`, `ipc-mexico`, `ipsa`, `merval`, `ivbx`, `ibrx-50`
- **Europe:** `ftse-100`, `dax`, `cac-40`, `euro-stoxx-50`, `euronext-100`, `bel-20`, `moex`, `aex`, `ibex-35`, `ftse-mib`, `smi`, `psi`, `atx`, `omxs30`, `omxc25`, `wig20`, `budapest-se`, `moex-russia`, `rtsi`, `cboe-uk-100`
- **Asia:** `hang-seng`, `sti`, `sensex`, `idx-composite`, `ftse-bursa`, `kospi`, `twse`, `nikkei-225`, `shanghai`, `szse-component`, `set`, `nifty-50`, `nifty-200`, `psei-composite`, `china-a50`, `dj-shanghai`, `india-vix`
- **Africa:** `egx-30`, `jse-40`, `ftse-jse`, `afr-40`, `raf-40`, `sa-40`, `alt-15`
- **Middle East:** `ta-125`, `ta-35`, `tadawul-all-share`, `tamayuz`, `bist-100`
- **Oceania:** `asx-200`, `all-ordinaries`, `nzx-50`
- **Global/Currency:** `usd`, `msci-europe`, `gbp`, `euro`, `yen`, `australian`, `msci-world`

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** Array of [`MarketIndex`](#marketindex-schema) objects.
  - **Example (200):**
    ```json
    [
      {
        "name": "S&P 500",
        "value": 4300.0,
        "change": "+10.00",
        "percentChange": "+0.23%",
        "fiveDaysReturn": "-19.35%",
        "oneMonthReturn": "-28.48%",
        "threeMonthReturn": "-14.02%",
        "sixMonthReturn": "36.39%",
        "ytdReturn": "+10.00%",
        "yearReturn": "+20.00%",
        "threeYearReturn": "+30.00%",
        "fiveYearReturn": "+40.00%",
        "tenYearReturn": "2,005.31%",
        "maxReturn": "22,857.89%"
      }
    ]
    ```

## Schema References

### MarketIndex Schema

| Field              | Type   | Description                | Required |
|:-------------------|:-------|:---------------------------|:--------:|
| `name`             | string | Name of the index          |    ✓     |
| `value`            | number | Current value of the index |    ✓     |
| `change`           | string | Change in the index        |    ✓     |
| `percentChange`    | string | Percentage change          |    ✓     |
| `fiveDaysReturn`   | string | 5-day return               |          |
| `oneMonthReturn`   | string | 1-month return             |          |
| `threeMonthReturn` | string | 3-month return             |          |
| `sixMonthReturn`   | string | 6-month return             |          |
| `ytdReturn`        | string | Year-to-date return        |          |
| `yearReturn`       | string | 1-year return              |          |
| `threeYearReturn`  | string | 3-year return              |          |
| `fiveYearReturn`   | string | 5-year return              |          |
| `tenYearReturn`    | string | 10-year return             |          |
| `maxReturn`        | string | Maximum all-time return    |          |

