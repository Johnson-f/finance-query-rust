# GraphQL API Examples

## Overview

The GraphQL API is available at:
- **POST `/graphql`** - GraphQL query endpoint
- **GET `/graphql-playground`** - Interactive GraphQL Playground UI

## Basic cURL Examples

### 1. Simple Quotes Query

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { quotes(symbols: [\"AAPL\", \"MSFT\", \"GOOG\"]) { symbol name price change percentChange } }"
  }'
```

### 2. Simple Quotes (Simplified Response)

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { simpleQuotes(symbols: [\"TSLA\", \"NVDA\"]) { symbol name price change percentChange } }"
  }'
```

### 3. Historical Data Query

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { historical(symbol: \"AAPL\", range: \"1y\", interval: \"1d\") { data { open high low close volume } } }"
  }'
```

### 4. Historical Data with Indicators

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { historical(symbol: \"TSLA\", range: \"1y\", interval: \"1d\", indicators: [\"sma\", \"ema\"], period: \"10,20,50\") { data { close sma ema } } }"
  }'
```

### 5. Search Query

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { search(query: \"Apple\", hits: 10) { results { symbol name exchange } } }"
  }'
```

### 6. News Query (General)

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { news { title link source time } }"
  }'
```

### 7. News by Symbol

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { newsBySymbol(symbol: \"AAPL\") { title link source time } }"
  }'
```

### 8. Financials Query

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { financials(symbol: \"AAPL\", statement: \"income\", frequency: \"annual\") { symbol statementType frequency } }"
  }'
```

### 9. Earnings Calls List

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { earningsCalls(symbol: \"AAPL\") { symbol total earningsCalls { eventId quarter year title } } }"
  }'
```

### 10. Earnings Transcript

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { earningsTranscript(symbol: \"AAPL\", quarter: \"Q1\", year: 2024) { symbol quarter year title speakers { name role } } }"
  }'
```

### 11. Market Movers

```bash
# Most Active
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { actives { symbol name price change percentChange } }"
  }'

# Top Gainers
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { gainers { symbol name price change percentChange } }"
  }'

# Top Losers
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { losers { symbol name price change percentChange } }"
  }'
```

### 12. Indices

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { indices { name value change percentChange } }"
  }'
```

### 13. Holders Queries

```bash
# Major Holders
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { majorHolders(symbol: \"AAPL\") { symbol breakdown { breakdownData } } }"
  }'

# Institutional Holders
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { institutionalHolders(symbol: \"AAPL\") { symbol holders { holder shares dateReported percentOut } } }"
  }'
```

### 14. Analyst Data

```bash
# Recommendations
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { recommendations(symbol: \"AAPL\") { symbol recommendations { period strongBuy buy hold sell } } }"
  }'

# Price Targets
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { priceTargets(symbol: \"AAPL\") { symbol priceTargets { current mean median low high } } }"
  }'
```

### 15. Sectors

```bash
# All Sectors
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { sectors { sector dayReturn ytdReturn yearReturn } }"
  }'

# Sector for Symbol
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { sectorForSymbol(symbol: \"AAPL\") { sector dayReturn ytdReturn } }"
  }'

# Sector Details
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { sectorDetails(sector: \"Technology\") { sector marketCap marketWeight industries companies } }"
  }'
```

### 16. Similar Stocks

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { similar(symbol: \"AAPL\", limit: 10) { symbol name price change percentChange } }"
  }'
```

### 17. Health Check

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { ping }"
  }'

curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { health { status timestamp services { status } } }"
  }'
```

## Using Variables

For more complex queries, you can use GraphQL variables:

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query GetHistorical($symbol: String!, $range: String!, $interval: String!) { historical(symbol: $symbol, range: $range, interval: $interval) { data { open high low close volume } } }",
    "variables": {
      "symbol": "AAPL",
      "range": "1y",
      "interval": "1d"
    }
  }'
```

## Multiple Queries in One Request

You can combine multiple queries:

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { quotes(symbols: [\"AAPL\"]) { symbol price } newsBySymbol(symbol: \"AAPL\") { title link } }"
  }'
```

## Pretty Print JSON Response

Add `jq` for formatted output:

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { quotes(symbols: [\"AAPL\"]) { symbol name price } }"
  }' | jq .
```

## Using GraphQL Playground

## GraphQL Subscriptions

GraphQL subscriptions provide real-time updates over WebSocket connections. All subscriptions update every 5 seconds.

### 1. Subscribe to Profile Updates

Subscribe to profile data (quote, similar stocks, sector, news) for a symbol.

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { profileUpdates(symbol: \"AAPL\") { quote { symbol price } similar { symbol name } sectorPerformance { sector dayReturn } news { title link } } }" }' \
  http://localhost:8080/graphql
```

### 2. Subscribe to Quote Updates

Subscribe to real-time quote updates for multiple symbols.

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { quoteUpdates(symbols: [\"AAPL\", \"MSFT\"]) { symbol name price change percentChange } }" }' \
  http://localhost:8080/graphql
```

### 3. Subscribe to Market Indices

Subscribe to US market indices updates (DJIA, NASDAQ, S&P 500).

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { indicesUpdates { symbol name price change percentChange } }" }' \
  http://localhost:8080/graphql
```

### 4. Subscribe to News Updates

Subscribe to general market news updates.

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { newsUpdates { title link source time } }" }' \
  http://localhost:8080/graphql
```

### 5. Subscribe to Sector Updates

Subscribe to sector performance updates.

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { sectorsUpdates { sector dayReturn ytdReturn } }" }' \
  http://localhost:8080/graphql
```

### 6. Subscribe to Market Movers

Subscribe to market movers (actives, gainers, losers).

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { moversUpdates { actives { symbol name price percentChange } gainers { symbol name price percentChange } losers { symbol name price percentChange } } }" }' \
  http://localhost:8080/graphql
```

### 7. Subscribe to Market Hours

Subscribe to market status/hours updates.

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { marketHoursUpdates { status reason timestamp } }" }' \
  http://localhost:8080/graphql
```

### 8. Subscribe to Moving Averages

Subscribe to real-time moving average calculations for a symbol.

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{ "query": "subscription { movingAverageUpdates(symbol: \"AAPL\", indicatorType: \"sma\", period: 20) { symbol indicatorType period value timestamp } }" }' \
  http://localhost:8080/graphql
```

**Note:** For moving averages, `indicatorType` must be either `"sma"` (Simple Moving Average) or `"ema"` (Exponential Moving Average).

## GraphQL Playground

Visit `http://localhost:8080/graphql-playground` in your browser for an interactive GraphQL IDE where you can:
- Explore the schema
- Write and test queries
- See auto-completion
- View query history

## Common Query Patterns

### Get Only Specific Fields

```bash
# Get minimal quote data
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { quotes(symbols: [\"AAPL\"]) { symbol price } }"
  }'
```

### Nested Data

```bash
# Historical data with nested indicator values
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { historical(symbol: \"TSLA\", range: \"1mo\", interval: \"1d\", indicators: [\"sma\"], period: \"20\") { data { close sma } } }"
  }'
```

## Error Handling

GraphQL returns errors in a structured format:

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { historical(symbol: \"INVALID\", range: \"1y\", interval: \"1d\") { data { close } } }"
  }'
```

Errors will be returned in the `errors` field of the response:

```json
{
  "errors": [
    {
      "message": "Error message here",
      "locations": [{"line": 1, "column": 9}],
      "path": ["historical"]
    }
  ],
  "data": null
}
```

## Production URL

If deploying to production, replace `localhost:8080` with your production URL:

```bash
curl -X POST https://api.tradstry.com/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { quotes(symbols: [\"AAPL\"]) { symbol price } }"
  }'
```

