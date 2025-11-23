<!-- ca597b6c-00ea-4b27-924c-47e44628d5a6 1f10c90f-9bc5-408b-afd0-8ce365d74977 -->
# Implement URL-based Earnings Transcript Scraping

## Current Flow

1. User calls: `GET /v1/earnings-transcript/TSLA?quarter=Q3&year=2024`
2. Handler calls `service::get_earnings_transcript(symbol, quarter, year)`
3. Service calls `get_earnings_calls_list()` which scrapes Yahoo Finance to get list of earnings calls
4. Service filters the list for the requested quarter/year (lines 98-126)
5. Service uses API (`yahoo_client.get_earnings_transcript`) with `event_id` and `company_id` to fetch transcript
6. Service parses the API response into `EarningsTranscript`

## Desired Flow

1. User calls: `GET /v1/earnings-transcript/TSLA?quarter=Q3&year=2024`
2. Handler calls `service::get_earnings_transcript(symbol, quarter, year)`
3. Service calls `get_earnings_calls_list()` which scrapes Yahoo Finance to get list of earnings calls
4. Service filters the list for the requested quarter/year (lines 98-126) ✅ Already implemented
5. **NEW:** Service scrapes the transcript content directly from `target_call.url` (e.g., `https://finance.yahoo.com/quote/TSLA/earnings/TSLA-Q3-2024-earnings_call-215125.html`)
6. Service parses the scraped HTML into `EarningsTranscript`

## Changes Required

### 1. Add scraper function for transcript content (`src/client/scraper.rs`)

- Create `scrape_earnings_transcript_from_url(fetch_client, url)` function
- Fetch HTML from the earnings call URL using `fetch_client.fetch(url)`
- Parse HTML DOM to extract:
- Speaker information (name, role, company) from the page
- Transcript paragraphs (speaker + text pairs) from transcript sections
- Metadata (date, title, fiscal year/period) from page headers/metadata
- Return data structured to work with transcript parsing

### 2. Modify `get_earnings_transcript` function (`src/service/earnings_transcript.rs`)

- **Keep** the existing flow: call `get_earnings_calls_list()` and filter (lines 87-126)
- **Remove** lines 128-155 (API-based fetching):
- Remove `get_quote_type` call
- Remove `company_id` extraction
- Remove `yahoo_client.get_earnings_transcript` API call
- **Add** after line 126 (after filtering):
- Call `scraper::scrape_earnings_transcript_from_url(fetch_client, &target_call.url)`
- Pass scraped data to parsing function
- **Modify** `parse_transcript` or create new parsing function to handle HTML-scraped data format

### 3. Update function signature

- Remove `yahoo_client` parameter from `get_earnings_transcript` (no longer needed)
- Keep `fetch_client` as it's required for both list scraping and transcript scraping

## Implementation Details

The filtering logic (lines 98-126) is already correct and will remain:

- Normalizes quarter format (Q3, q3, 3 → Q3)
- Finds matching call by quarter and year
- Falls back to most recent call if quarter/year not specified
- Returns `target_call` with the `.url` field populated

The new scraper function will:

- Use `fetch_client.fetch(&url)` to get HTML from `target_call.url`
- Parse the HTML using `scraper::Html::parse_document`
- Extract transcript content from the DOM structure
- Return structured data that can be converted to `EarningsTranscript`

## Files to Modify

1. `src/client/scraper.rs` - Add `scrape_earnings_transcript_from_url` function
2. `src/service/earnings_transcript.rs` - Modify `get_earnings_transcript` to use URL scraping instead of API (remove API calls, add scraper call)
3. `src/routes/earnings_transcript.rs` - Update handler to not pass `yahoo_client` to service function

### To-dos

- [ ] Create scrape_earnings_transcript_from_url function in src/client/scraper.rs to extract transcript data from HTML
- [ ] Update get_earnings_transcript in src/service/earnings_transcript.rs to use URL scraping instead of API calls
- [ ] Adapt parse_transcript or create HTML-specific parsing to handle scraped data format