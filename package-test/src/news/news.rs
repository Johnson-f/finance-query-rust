//! News data fetching functions using finance-query-core
//!
//! This module provides functions to fetch news data from Yahoo Finance:
//! - News articles for a specific stock symbol
//! - News search by query
//! - Trending news and market news

use finance_query_core::{FetchClient, YahooAuthManager, YahooError, YahooFinanceClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// ============================================================================
// News Types
// ============================================================================

/// A news article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub uuid: Option<String>,
    pub title: String,
    pub publisher: Option<String>,
    pub link: String,
    pub publish_time: Option<i64>,
    pub publish_time_formatted: Option<String>,
    pub thumbnail_url: Option<String>,
    pub related_tickers: Vec<String>,
}

impl NewsArticle {
    /// Get the publish date as a formatted string
    pub fn formatted_date(&self) -> String {
        if let Some(formatted) = &self.publish_time_formatted {
            return formatted.clone();
        }
        if let Some(ts) = self.publish_time {
            chrono::DateTime::from_timestamp(ts, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        } else {
            "Unknown".to_string()
        }
    }

    /// Get relative time (e.g., "2 hours ago")
    pub fn relative_time(&self) -> String {
        if let Some(ts) = self.publish_time {
            let now = chrono::Utc::now().timestamp();
            let diff = now - ts;

            if diff < 60 {
                format!("{}s ago", diff)
            } else if diff < 3600 {
                format!("{}m ago", diff / 60)
            } else if diff < 86400 {
                format!("{}h ago", diff / 3600)
            } else {
                format!("{}d ago", diff / 86400)
            }
        } else {
            "Unknown".to_string()
        }
    }
}

/// News response containing multiple articles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsResponse {
    pub symbol: Option<String>,
    pub query: Option<String>,
    pub articles: Vec<NewsArticle>,
    pub count: usize,
}

impl NewsResponse {
    /// Get articles from a specific publisher
    pub fn from_publisher(&self, publisher: &str) -> Vec<&NewsArticle> {
        self.articles
            .iter()
            .filter(|a| {
                a.publisher
                    .as_ref()
                    .map(|p| p.to_lowercase().contains(&publisher.to_lowercase()))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get articles mentioning a specific ticker
    pub fn mentioning_ticker(&self, ticker: &str) -> Vec<&NewsArticle> {
        self.articles
            .iter()
            .filter(|a| a.related_tickers.iter().any(|t| t.eq_ignore_ascii_case(ticker)))
            .collect()
    }

    /// Check if there are any articles
    pub fn is_empty(&self) -> bool {
        self.articles.is_empty()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Creates a configured YahooFinanceClient
async fn create_client() -> Result<(Arc<YahooAuthManager>, YahooFinanceClient), YahooError> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    // Prime authentication
    auth_manager.refresh().await?;

    Ok((auth_manager, client))
}

/// Parse news articles from Yahoo Finance search response
fn parse_news_from_search(json: &Value) -> Vec<NewsArticle> {
    let mut articles = Vec::new();

    if let Some(news_array) = json.get("news").and_then(|n| n.as_array()) {
        for item in news_array {
            if let Some(title) = item.get("title").and_then(|t| t.as_str()) {
                let article = NewsArticle {
                    uuid: item.get("uuid").and_then(|u| u.as_str()).map(String::from),
                    title: title.to_string(),
                    publisher: item.get("publisher").and_then(|p| p.as_str()).map(String::from),
                    link: item
                        .get("link")
                        .and_then(|l| l.as_str())
                        .unwrap_or("")
                        .to_string(),
                    publish_time: item.get("providerPublishTime").and_then(|t| t.as_i64()),
                    publish_time_formatted: None,
                    thumbnail_url: item
                        .get("thumbnail")
                        .and_then(|t| t.get("resolutions"))
                        .and_then(|r| r.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|first| first.get("url"))
                        .and_then(|u| u.as_str())
                        .map(String::from),
                    related_tickers: item
                        .get("relatedTickers")
                        .and_then(|t| t.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default(),
                };
                articles.push(article);
            }
        }
    }

    articles
}

/// Parse news from quote summary response (reserved for future use)
#[allow(dead_code)]
fn parse_news_from_quote_summary(_json: &Value, _symbol: &str) -> Vec<NewsArticle> {
    // The quote summary doesn't typically include news
    // We rely on the search endpoint instead
    Vec::new()
}


// ============================================================================
// Public API Functions
// ============================================================================

/// Get raw news data JSON for a symbol using search
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `count` - Number of news articles to fetch
///
/// # Returns
/// Raw JSON value containing news data
pub async fn get_news_raw(symbol: &str, count: usize) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.search(symbol, count).await
}

/// Get news articles for a stock symbol
///
/// Fetches recent news articles related to a specific stock
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `count` - Maximum number of articles to return
///
/// # Returns
/// NewsResponse with articles related to the symbol
///
/// # Example
/// ```rust,ignore
/// let news = get_news_for_symbol("AAPL", 10).await?;
/// for article in &news.articles {
///     println!("{} - {} ({})", article.title, article.publisher.as_deref().unwrap_or("Unknown"), article.relative_time());
/// }
/// ```
pub async fn get_news_for_symbol(symbol: &str, count: usize) -> Result<NewsResponse, YahooError> {
    let json = get_news_raw(symbol, count).await?;
    let articles = parse_news_from_search(&json);

    Ok(NewsResponse {
        symbol: Some(symbol.to_string()),
        query: None,
        count: articles.len(),
        articles,
    })
}

/// Search for news by query
///
/// Searches for news articles matching a query string
///
/// # Arguments
/// * `query` - Search query (e.g., "tech earnings", "oil prices")
/// * `count` - Maximum number of articles to return
///
/// # Returns
/// NewsResponse with articles matching the query
///
/// # Example
/// ```rust,ignore
/// let news = search_news("artificial intelligence stocks", 10).await?;
/// println!("Found {} articles", news.count);
/// ```
pub async fn search_news(query: &str, count: usize) -> Result<NewsResponse, YahooError> {
    let (_, client) = create_client().await?;
    let json = client.search(query, count).await?;
    let articles = parse_news_from_search(&json);

    Ok(NewsResponse {
        symbol: None,
        query: Some(query.to_string()),
        count: articles.len(),
        articles,
    })
}

/// Get news for multiple symbols
///
/// Fetches news for multiple stock symbols and combines results
///
/// # Arguments
/// * `symbols` - Slice of stock symbols
/// * `count_per_symbol` - Number of articles to fetch per symbol
///
/// # Returns
/// NewsResponse with combined articles from all symbols
///
/// # Example
/// ```rust,ignore
/// let news = get_news_for_symbols(&["AAPL", "GOOGL", "MSFT"], 5).await?;
/// println!("Total articles: {}", news.count);
/// ```
pub async fn get_news_for_symbols(
    symbols: &[&str],
    count_per_symbol: usize,
) -> Result<NewsResponse, YahooError> {
    let mut all_articles = Vec::new();
    let mut seen_uuids = std::collections::HashSet::new();

    for symbol in symbols {
        match get_news_for_symbol(symbol, count_per_symbol).await {
            Ok(response) => {
                for article in response.articles {
                    // Deduplicate by UUID if available, otherwise by title
                    let key = article
                        .uuid
                        .clone()
                        .unwrap_or_else(|| article.title.clone());
                    if seen_uuids.insert(key) {
                        all_articles.push(article);
                    }
                }
            }
            Err(_) => continue, // Skip symbols that fail
        }
    }

    // Sort by publish time (most recent first)
    all_articles.sort_by(|a, b| {
        b.publish_time
            .unwrap_or(0)
            .cmp(&a.publish_time.unwrap_or(0))
    });

    Ok(NewsResponse {
        symbol: None,
        query: Some(symbols.join(", ")),
        count: all_articles.len(),
        articles: all_articles,
    })
}

/// Get market news (general financial news)
///
/// Fetches general market and financial news
///
/// # Arguments
/// * `count` - Maximum number of articles to return
///
/// # Returns
/// NewsResponse with general market news
///
/// # Example
/// ```rust,ignore
/// let news = get_market_news(20).await?;
/// for article in news.articles.iter().take(5) {
///     println!("{}", article.title);
/// }
/// ```
pub async fn get_market_news(count: usize) -> Result<NewsResponse, YahooError> {
    // Use broad market terms to get general news
    search_news("stock market", count).await
}

/// Get sector-specific news
///
/// Fetches news for a specific market sector
///
/// # Arguments
/// * `sector` - Sector name (e.g., "technology", "healthcare", "energy")
/// * `count` - Maximum number of articles to return
///
/// # Returns
/// NewsResponse with sector-specific news
///
/// # Example
/// ```rust,ignore
/// let news = get_sector_news("technology", 10).await?;
/// println!("Tech news: {} articles", news.count);
/// ```
pub async fn get_sector_news(sector: &str, count: usize) -> Result<NewsResponse, YahooError> {
    let query = format!("{} stocks", sector);
    search_news(&query, count).await
}

/// Get trending/popular news
///
/// Fetches currently trending financial news
///
/// # Arguments
/// * `count` - Maximum number of articles to return
///
/// # Returns
/// NewsResponse with trending news
pub async fn get_trending_news(count: usize) -> Result<NewsResponse, YahooError> {
    // Search for trending market topics
    search_news("trending stocks today", count).await
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require network access and valid Yahoo Finance authentication
    // Run with: cargo test -p package-test -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_get_news_for_symbol() {
        let result = get_news_for_symbol("AAPL", 5).await;
        assert!(result.is_ok());
        let news = result.unwrap();
        assert_eq!(news.symbol, Some("AAPL".to_string()));
    }

    #[tokio::test]
    #[ignore]
    async fn test_search_news() {
        let result = search_news("technology stocks", 5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_news_for_symbols() {
        let result = get_news_for_symbols(&["AAPL", "GOOGL"], 3).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_market_news() {
        let result = get_market_news(5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_sector_news() {
        let result = get_sector_news("technology", 5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_trending_news() {
        let result = get_trending_news(5).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_news_article_relative_time() {
        let now = chrono::Utc::now().timestamp();

        // Test seconds ago
        let article = NewsArticle {
            uuid: None,
            title: "Test".to_string(),
            publisher: None,
            link: "".to_string(),
            publish_time: Some(now - 30),
            publish_time_formatted: None,
            thumbnail_url: None,
            related_tickers: vec![],
        };
        assert!(article.relative_time().contains("s ago"));

        // Test hours ago
        let article = NewsArticle {
            uuid: None,
            title: "Test".to_string(),
            publisher: None,
            link: "".to_string(),
            publish_time: Some(now - 7200),
            publish_time_formatted: None,
            thumbnail_url: None,
            related_tickers: vec![],
        };
        assert!(article.relative_time().contains("h ago"));
    }

    #[test]
    fn test_news_response_filtering() {
        let articles = vec![
            NewsArticle {
                uuid: Some("1".to_string()),
                title: "Apple News".to_string(),
                publisher: Some("Reuters".to_string()),
                link: "".to_string(),
                publish_time: None,
                publish_time_formatted: None,
                thumbnail_url: None,
                related_tickers: vec!["AAPL".to_string()],
            },
            NewsArticle {
                uuid: Some("2".to_string()),
                title: "Google News".to_string(),
                publisher: Some("Bloomberg".to_string()),
                link: "".to_string(),
                publish_time: None,
                publish_time_formatted: None,
                thumbnail_url: None,
                related_tickers: vec!["GOOGL".to_string()],
            },
        ];

        let response = NewsResponse {
            symbol: None,
            query: None,
            articles,
            count: 2,
        };

        assert_eq!(response.from_publisher("Reuters").len(), 1);
        assert_eq!(response.mentioning_ticker("AAPL").len(), 1);
        assert!(!response.is_empty());
    }
}
