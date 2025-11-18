# Earnings Transcripts

## GET /v1/earnings/{symbol}/calls

### Overview

**Purpose:** Retrieve list of available earnings calls for a stock  
**Response Format:** Object containing list of call listings

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`EarningsCallsList`](#earningscallslist-schema)

## GET /v1/earnings/{symbol}/transcript

### Overview

**Purpose:** Retrieve a specific earnings transcript  
**Response Format:** Object containing transcript content

### Path Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `symbol`  | string |    ✓     | The stock ticker symbol | `AAPL`  |

### Query Parameters

| Parameter | Type   | Required | Description             | Example |
|-----------|--------|:--------:|-------------------------|---------|
| `quarter` | string |          | Quarter (Q1, Q2, Q3, Q4)| `Q3`    |
| `year`    | integer|          | Year                    | `2023`  |

**Responses:**

- **200 OK**
  - **Content-Type:** `application/json`
  - **Schema:** [`EarningsTranscript`](#earningstranscript-schema)

## Schema References

### EarningsCallsList Schema

| Field         | Type                                      | Description               | Required |
|---------------|-------------------------------------------|---------------------------|:--------:|
| symbol        | string                                    | Stock symbol              |    ✓     |
| earningsCalls | [EarningsCallListing[]](#earningscalllisting) | List of available calls   |    ✓     |
| total         | integer                                   | Total count               |    ✓     |

### EarningsCallListing Schema

| Field   | Type   | Description               | Required |
|---------|--------|---------------------------|:--------:|
| eventId | string | Unique event ID           |    ✓     |
| quarter | string | Quarter (e.g., "Q3")      |          |
| year    | integer| Year (e.g., 2023)         |          |
| title   | string | Call title                |    ✓     |
| url     | string | URL to call info          |    ✓     |

### EarningsTranscript Schema

| Field      | Type                                      | Description               | Required |
|------------|-------------------------------------------|---------------------------|:--------:|
| symbol     | string                                    | Stock symbol              |    ✓     |
| quarter    | string                                    | Quarter                   |    ✓     |
| year       | integer                                   | Year                      |    ✓     |
| date       | datetime                                  | Date of call              |    ✓     |
| title      | string                                    | Transcript title          |    ✓     |
| speakers   | [TranscriptSpeaker[]](#transcriptspeaker) | List of speakers          |    ✓     |
| paragraphs | [TranscriptParagraph[]](#transcriptparagraph) | Transcript text segments  |    ✓     |
| metadata   | object                                    | Additional metadata       |    ✓     |

### TranscriptSpeaker Schema

| Field   | Type   | Description               | Required |
|---------|--------|---------------------------|:--------:|
| name    | string | Speaker name              |    ✓     |
| role    | string | Role (e.g., CEO)          |          |
| company | string | Company                   |          |

### TranscriptParagraph Schema

| Field   | Type   | Description               | Required |
|---------|--------|---------------------------|:--------:|
| speaker | string | Speaker name              |    ✓     |
| text    | string | Spoken text               |    ✓     |