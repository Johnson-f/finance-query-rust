<!-- 969fe822-8bfd-467c-b9a2-7d27241ab116 8fe2e40c-e7a0-4176-a34b-562b1442c316 -->
# Implement Max Range with 1d Interval Chunking

## Problem

Yahoo Finance API restricts `max` range to only work with `1mo` interval. When users request `max` range with `1d` interval, we need to chunk the request into smaller time periods and merge the results.

## Solution Overview

1. Add period1/period2 parameter support to the Yahoo Finance client
2. Detect max+1d combination in the service layer and route to chunking logic
3. Implement optimized chunking that first detects the stock's actual start date
4. Fetch data in 10-year chunks and merge results

## Implementation Steps

### 1. Add period1/period2 support to YahooFinanceClient

**File:** `src/client/yahoo_client.rs`

Add a new method `get_chart_with_periods` after the existing `get_chart` method (around line 187):

- Accepts `symbol`, `interval`, `period1` (i64), and `period2` (i64) parameters
- Uses the same URL format but with `period1` and `period2` query parameters instead of `range`
- Returns the same `Result<Value, YahooError>` type

### 2. Modify historical service to handle max+1d case

**File:** `src/service/historical.rs`

Update `get_historical` function:

- Add conditional check: if `time_range == TimeRange::Max && interval == Interval::Daily`, route to new chunking function
- Otherwise, use existing direct API call path

### 3. Implement optimized chunking function

**File:** `src/service/historical.rs`

Create `get_historical_max_daily` function:

- First fetch `max` range with `1mo` interval to detect the stock's actual earliest trading date
- Parse the response to find the minimum timestamp
- If no data found, fallback to 1970-01-01 (Unix epoch start)
- Break the time period from earliest date to now into 10-year chunks
- For each chunk, call `get_chart_with_periods` with `1d` interval
- Merge all chunk responses into a single `HistoricalResponse`
- Add logging for chunk progress and final summary

### 4. Add necessary imports

**File:** `src/service/historical.rs`

- Add `tracing::{debug, info}` for logging
- `chrono` is already available via existing dependencies

## Technical Details

- **Chunk size:** 10 years (10 * 365 * 24 * 60 * 60 seconds)
- **Start date detection:** Use `max` + `1mo` query to find earliest available data point
- **Data merging:** Use HashMap's `extend` method which automatically handles duplicate timestamps
- **Error handling:** Propagate errors from individual chunk requests using `?` operator
- **Performance:** Sequential chunk fetching to avoid rate limiting (can be optimized later with parallel requests if needed)

## Testing Considerations

- Test with stocks that have long histories (e.g., stocks from 1970s)
- Test with newer stocks to ensure it doesn't make unnecessary requests
- Verify that merged data maintains chronological order
- Ensure no duplicate timestamps in final response

### To-dos

- [ ] Add get_chart_with_periods method to YahooFinanceClient in src/client/yahoo_client.rs
- [ ] Add conditional check in get_historical to detect max+1d combination and route to chunking
- [ ] Implement get_historical_max_daily function with start date detection and 10-year chunking
- [ ] Add tracing imports and logging statements for chunk progress and final summary