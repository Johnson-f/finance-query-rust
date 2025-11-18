use crate::client::error::YahooError;
use crate::client::FetchClient;
use crate::models::News;
use serde_json;
use std::sync::Arc;
use tracing::info;

const API_BASE_URL: &str = "https://finance-query.onrender.com/v1";

pub async fn scrape_news_for_quote(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<Vec<News>, YahooError> {
    info!("Fetching news for symbol: {} from API", symbol);
    
    let url = format!("{}/news?symbol={}", API_BASE_URL, symbol);
    let response_text = fetch_client.fetch(&url).await?;
    
    let news: Vec<News> = serde_json::from_str(&response_text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse news JSON for {}: {}", symbol, e)))?;
    
    if news.is_empty() {
        return Err(YahooError::NotFound(format!("No news found for symbol: {}", symbol)));
    }
    
    info!("Successfully fetched {} news items for {}", news.len(), symbol);
    Ok(news)
}

pub async fn scrape_general_news(
    fetch_client: &Arc<FetchClient>,
) -> Result<Vec<News>, YahooError> {
    info!("Fetching general news from API");
    
    let url = format!("{}/news", API_BASE_URL);
    let response_text = fetch_client.fetch(&url).await?;
    
    let news: Vec<News> = serde_json::from_str(&response_text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse general news JSON: {}", e)))?;
    
    info!("Successfully fetched {} general news items", news.len());
    Ok(news)
}