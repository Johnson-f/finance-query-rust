# News Stream

## Overview

**Endpoint:** `/v1/ws/news`

Streams the latest general financial news articles.

## Protocol

1. **Connect:** Establish a WebSocket connection to the endpoint.
2. **Receive:** The server will automatically start streaming updates every 5 seconds.

## Server Response

A JSON array of news objects.

```json
[
  {
    "title": "Fed signals interest rate pause",
    "link": "https://example.com/news/fed-rates",
    "source": "Bloomberg",
    "img": "https://example.com/images/fed.jpg",
    "time": "10 mins ago"
  }
]
```

## Schema

See [News Schema](../api/news.md#news-schema) for detailed field definitions.

