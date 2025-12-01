# News Model

The News model provides financial news articles related to stocks, markets, and companies.

## Overview

The `News` struct represents a single news article with its metadata, including the title, source, link, thumbnail image, and publication time. This model is typically returned as part of collections when fetching news for specific symbols or market segments.

## Data Structure

### News

```rust
pub struct News {
    pub title: String,
    pub link: String,
    pub source: String,
    pub img: String,
    pub time: String,
}
```

**Fields:**
- `title` - The headline or title of the news article
- `link` - Full URL to the news article
- `source` - The publisher or news source (e.g., "Reuters", "Bloomberg", "Yahoo Finance")
- `img` - URL to the article's thumbnail image
- `time` - Human-readable timestamp indicating when the article was published (e.g., "2h ago", "1d ago")

## JSON Format

### Example Single Article

```json
{
  "title": "Apple Announces Record Q4 Earnings, Beats Expectations",
  "link": "https://finance.yahoo.com/news/apple-announces-record-q4-earnings-123456789.html",
  "source": "Yahoo Finance",
  "img": "https://s.yimg.com/uu/api/res/1.2/apple_earnings_thumbnail.jpg",
  "time": "2h ago"
}
```

### Example News Collection

```json
[
  {
    "title": "Tesla Stock Surges on Strong Delivery Numbers",
    "link": "https://finance.yahoo.com/news/tesla-stock-surges-delivery-123456.html",
    "source": "Reuters",
    "img": "https://s.yimg.com/uu/api/res/1.2/tesla_thumbnail.jpg",
    "time": "1h ago"
  },
  {
    "title": "Fed Signals Potential Rate Cut in Coming Months",
    "link": "https://finance.yahoo.com/news/fed-signals-rate-cut-789012.html",
    "source": "Bloomberg",
    "img": "https://s.yimg.com/uu/api/res/1.2/fed_thumbnail.jpg",
    "time": "3h ago"
  },
  {
    "title": "Microsoft Azure Revenue Grows 30% Year-Over-Year",
    "link": "https://finance.yahoo.com/news/microsoft-azure-revenue-345678.html",
    "source": "CNBC",
    "img": "https://s.yimg.com/uu/api/res/1.2/msft_thumbnail.jpg",
    "time": "5h ago"
  }
]
```

### Field Descriptions

- `title` - Always present, contains the article headline
- `link` - Always present, direct URL to the full article
- `source` - Always present, identifies the news publisher
- `img` - Always present, URL to thumbnail (may be placeholder if unavailable)
- `time` - Always present, relative time format (e.g., "2h ago", "1d ago", "3w ago")

## Usage Examples

### Fetching News for a Symbol

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Fetch news for Apple
    let news_list = client.get_news("AAPL").await?;
    
    println!("Latest news for AAPL:");
    for article in news_list {
        println!("\n{}", article.title);
        println!("Source: {} | {}", article.source, article.time);
        println!("Link: {}", article.link);
    }
    
    Ok(())
}
```

### Displaying News with Formatting

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let news_list = client.get_news("TSLA").await?;
    
    println!("═══════════════════════════════════════════════════════");
    println!("  TESLA (TSLA) - Latest News");
    println!("═══════════════════════════════════════════════════════\n");
    
    for (i, article) in news_list.iter().enumerate() {
        println!("{}. {}", i + 1, article.title);
        println!("   📰 {} • ⏰ {}", article.source, article.time);
        println!("   🔗 {}", article.link);
        println!();
    }
    
    Ok(())
}
```

### Filtering News by Source

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let news_list = client.get_news("NVDA").await?;
    
    // Filter for specific sources
    let premium_sources = vec!["Bloomberg", "Reuters", "Wall Street Journal"];
    
    let filtered_news: Vec<_> = news_list
        .into_iter()
        .filter(|article| {
            premium_sources.iter().any(|source| article.source.contains(source))
        })
        .collect();
    
    println!("Premium news sources for NVDA:");
    for article in filtered_news {
        println!("{} - {}", article.source, article.title);
    }
    
    Ok(())
}
```

### Filtering Recent News

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let news_list = client.get_news("MSFT").await?;
    
    // Filter for very recent news (within hours)
    let recent_news: Vec<_> = news_list
        .into_iter()
        .filter(|article| article.time.contains("h ago") || article.time.contains("m ago"))
        .collect();
    
    println!("Breaking news for MSFT:");
    for article in recent_news {
        println!("[{}] {}", article.time, article.title);
    }
    
    Ok(())
}
```

### Creating a News Feed

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "GOOGL", "MSFT", "AMZN"];
    
    println!("Tech Stock News Feed\n");
    
    for symbol in symbols {
        let news_list = client.get_news(symbol).await?;
        
        if let Some(latest) = news_list.first() {
            println!("[{}] {}", symbol, latest.title);
            println!("    {} • {}", latest.source, latest.time);
            println!();
        }
    }
    
    Ok(())
}
```

### Exporting News to JSON

```rust
use finance_query_core::YahooClient;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let news_list = client.get_news("AAPL").await?;
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&news_list)?;
    
    // Save to file
    std::fs::write("aapl_news.json", json)?;
    println!("News saved to aapl_news.json");
    
    Ok(())
}
```

### Building a News Aggregator

```rust
use finance_query_core::{YahooClient, News};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "TSLA", "NVDA", "AMD", "INTC"];
    
    let mut news_by_symbol: HashMap<String, Vec<News>> = HashMap::new();
    
    for symbol in symbols {
        let news_list = client.get_news(symbol).await?;
        news_by_symbol.insert(symbol.to_string(), news_list);
    }
    
    // Display aggregated news
    for (symbol, news_list) in news_by_symbol {
        println!("\n{} ({} articles)", symbol, news_list.len());
        println!("{}", "─".repeat(50));
        
        for article in news_list.iter().take(3) {
            println!("  • {}", article.title);
        }
    }
    
    Ok(())
}
```

### Detecting Breaking News

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let news_list = client.get_news("SPY").await?;
    
    // Check for very recent market news
    let breaking_news: Vec<_> = news_list
        .into_iter()
        .filter(|article| {
            article.time.contains("m ago") || 
            (article.time.contains("h ago") && 
             article.time.chars().next().unwrap().to_digit(10).unwrap_or(99) < 2)
        })
        .collect();
    
    if !breaking_news.is_empty() {
        println!("🚨 BREAKING NEWS 🚨\n");
        for article in breaking_news {
            println!("{}", article.title);
            println!("Published: {} by {}", article.time, article.source);
            println!("{}\n", article.link);
        }
    } else {
        println!("No breaking news at this time.");
    }
    
    Ok(())
}
```

### Creating an HTML News Page

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let news_list = client.get_news("AAPL").await?;
    
    let mut html = String::from(r#"
<!DOCTYPE html>
<html>
<head>
    <title>AAPL News</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
        .article { border: 1px solid #ddd; margin: 20px 0; padding: 15px; border-radius: 5px; }
        .article img { max-width: 200px; float: right; margin-left: 15px; }
        .article h2 { margin-top: 0; }
        .meta { color: #666; font-size: 0.9em; }
    </style>
</head>
<body>
    <h1>Apple Inc. (AAPL) - Latest News</h1>
"#);
    
    for article in news_list {
        html.push_str(&format!(r#"
    <div class="article">
        <img src="{}" alt="Article thumbnail">
        <h2><a href="{}">{}</a></h2>
        <p class="meta">{} • {}</p>
    </div>
"#, article.img, article.link, article.title, article.source, article.time));
    }
    
    html.push_str("</body>\n</html>");
    
    std::fs::write("aapl_news.html", html)?;
    println!("HTML news page created: aapl_news.html");
    
    Ok(())
}
```

## Time Format

The `time` field uses a human-readable relative format:

- `"1m ago"` - 1 minute ago
- `"30m ago"` - 30 minutes ago
- `"2h ago"` - 2 hours ago
- `"1d ago"` - 1 day ago
- `"3d ago"` - 3 days ago
- `"1w ago"` - 1 week ago
- `"2w ago"` - 2 weeks ago

Note: The exact format may vary slightly depending on the data source.

## Common Use Cases

1. **Portfolio News Monitoring** - Track news for all stocks in your portfolio
2. **Breaking News Alerts** - Filter for very recent articles to catch breaking news
3. **Sentiment Analysis** - Analyze news headlines for sentiment indicators
4. **News Aggregation** - Build custom news feeds combining multiple sources
5. **Research** - Collect historical news data for backtesting strategies
6. **Notifications** - Set up alerts when news is published for specific symbols

## Notes

- News articles are typically returned in reverse chronological order (newest first)
- The number of articles returned varies by symbol and availability
- Image URLs point to Yahoo Finance's CDN and are generally reliable
- Links direct to the original article source or Yahoo Finance's article page
- News data is updated continuously throughout trading hours
- Some articles may be behind paywalls on the source website
- The `time` field is relative to when the data was fetched

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_news("AAPL").await {
        Ok(news_list) => {
            if news_list.is_empty() {
                println!("No news available for this symbol");
            } else {
                println!("Found {} articles", news_list.len());
                for article in news_list {
                    println!("{}", article.title);
                }
            }
        }
        Err(YahooError::NotFound) => {
            eprintln!("Symbol not found or no news available");
        }
        Err(YahooError::Network(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(e) => {
            eprintln!("Error fetching news: {}", e);
        }
    }
}
```

## See Also

- [Quote Model](./quote.md) - For stock price data
- [Calendar Model](./calendar.md) - For earnings and event dates
- [Analysts Model](./analysts.md) - For analyst ratings and recommendations
