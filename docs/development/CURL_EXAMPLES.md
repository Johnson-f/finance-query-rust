# CURL Examples for Finance Query Rust API

The server runs on `http://localhost:8080` (or `http://0.0.0.0:8080`).

## Health Check Endpoints

### Ping
```bash
curl http://localhost:8080/ping
```

### Health Check
```bash
curl http://localhost:8080/health
```

## Quotes Endpoints

### Get Quotes (Full Details)
Get detailed quote information for one or more symbols (comma-separated):
```bash
# Single symbol
curl "http://localhost:8080/v1/quotes?symbols=AAPL"

# Multiple symbols
curl "http://localhost:8080/v1/quotes?symbols=AAPL,MSFT,GOOGL"
```

### Get Simple Quotes
Get simplified quote information:
```bash
# Single symbol
curl "http://localhost:8080/v1/quotes/simple?symbols=AAPL"

# Multiple symbols
curl "http://localhost:8080/v1/quotes/simple?symbols=AAPL,MSFT,GOOGL"
```

## Historical Data

### Get Historical Prices
Get historical price data for a symbol with time range and interval.

**Time Ranges:** `1d`, `5d`, `1mo`, `3mo`, `6mo`, `1y`, `2y`, `5y`, `10y`, `ytd`, `max`

**Intervals:** `1m`, `3m`, `5m`, `10m`, `15m`, `20m`, `30m`, `65m`, `95m`, `1h`, `1d`, `1wk`, `1mo`

```bash
# Daily data for 1 year
curl "http://localhost:8080/v1/historical/AAPL?range=1y&interval=1d"

# Hourly data for 5 days
curl "http://localhost:8080/v1/historical/AAPL?range=5d&interval=1h"

# Weekly data for 5 years
curl "http://localhost:8080/v1/historical/MSFT?range=5y&interval=1wk"
```

## Search

### Search for Symbols
Search for stocks, ETFs, or other financial instruments:
```bash
# Basic search (default: 6 results)
curl "http://localhost:8080/v1/search?q=apple"

# Search with custom number of results
curl "http://localhost:8080/v1/search?q=tesla&hits=10"
```

## News

### Get News for a Symbol
Get news articles related to a specific symbol:
```bash
curl "http://localhost:8080/v1/news/AAPL"
```

### Get General News
Get general financial news:
```bash
curl "http://localhost:8080/v1/news"
```

## Financials

### Get Financial Statements
Get financial statement data (income statement, balance sheet, or cash flow).

**Statement Types:** `income` (or `income_statement`), `balance` (or `balance_sheet`), `cashflow` (or `cash_flow`)

**Frequencies:** `annual` (or `yearly`), `quarterly` (or `quarter`)

```bash
# Annual income statement
curl "http://localhost:8080/v1/financials/AAPL?statement=income&frequency=annual"

# Quarterly balance sheet
curl "http://localhost:8080/v1/financials/AAPL?statement=balance&frequency=quarterly"

# Annual cash flow
curl "http://localhost:8080/v1/financials/MSFT?statement=cashflow&frequency=annual"
```

## Earnings

### Get Earnings Calls List
Get list of earnings calls for a symbol:
```bash
curl "http://localhost:8080/v1/earnings/AAPL/calls"
```

### Get Earnings Transcript
Get transcript for a specific earnings call:
```bash
# Note: You'll need to get the event_id from the calls endpoint first
curl "http://localhost:8080/v1/earnings/AAPL/transcript/{event_id}"
```

## Market Movers

### Get Most Active Stocks
Get the most actively traded stocks:
```bash
# Default count (50)
curl "http://localhost:8080/v1/actives"

# Custom count (25, 50, or 100)
curl "http://localhost:8080/v1/actives?count=25"
curl "http://localhost:8080/v1/actives?count=100"
```

### Get Top Gainers
Get stocks with the highest price increases:
```bash
# Default count (50)
curl "http://localhost:8080/v1/gainers"

# Custom count (25, 50, or 100)
curl "http://localhost:8080/v1/gainers?count=25"
curl "http://localhost:8080/v1/gainers?count=100"
```

### Get Top Losers
Get stocks with the highest price decreases:
```bash
# Default count (50)
curl "http://localhost:8080/v1/losers"

# Custom count (25, 50, or 100)
curl "http://localhost:8080/v1/losers?count=25"
curl "http://localhost:8080/v1/losers?count=100"
```

## Pretty Print JSON Output

Add `| jq` to format the JSON output (requires `jq` to be installed):

```bash
curl "http://localhost:8080/v1/quotes?symbols=AAPL" | jq
```

Or use Python for pretty printing:
```bash
curl "http://localhost:8080/v1/quotes?symbols=AAPL" | python3 -m json.tool
```

## Verbose Output (Debugging)

Use `-v` flag to see request/response headers:
```bash
curl -v "http://localhost:8080/v1/quotes?symbols=AAPL"
```

## Save Response to File

```bash
curl "http://localhost:8080/v1/quotes?symbols=AAPL" -o response.json
```

## Common Examples

### Quick Test - Check if server is running
```bash
curl http://localhost:8080/ping
```

### Get Apple stock quote
```bash
curl "http://localhost:8080/v1/quotes?symbols=AAPL"
```

### Get Apple's 1-year daily historical data
```bash
curl "http://localhost:8080/v1/historical/AAPL?range=1y&interval=1d"
```

### Search for "apple"
```bash
curl "http://localhost:8080/v1/search?q=apple"
```