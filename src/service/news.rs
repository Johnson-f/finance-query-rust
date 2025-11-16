use crate::client::error::YahooError;
use crate::client::FetchClient;
use crate::models::News;
use scraper::{Html, Selector};
use std::sync::Arc;

pub async fn scrape_news_for_quote(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<Vec<News>, YahooError> {
    // Try different URLs based on exchange
    let urls = vec![
        format!("https://stockanalysis.com/stocks/{}", symbol),
        format!("https://stockanalysis.com/etf/{}", symbol),
        format!("https://stockanalysis.com/quote/otc/{}", symbol),
    ];

    for url in urls {
        match fetch_client.fetch(&url).await {
            Ok(html) => {
                if let Ok(news_list) = parse_news_from_html(&html) {
                    if !news_list.is_empty() {
                        return Ok(news_list);
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Err(YahooError::NotFound(format!("Could not find news for symbol: {}", symbol)))
}

pub async fn scrape_general_news(
    fetch_client: &Arc<FetchClient>,
) -> Result<Vec<News>, YahooError> {
    let url = "https://stockanalysis.com/news/";
    let html = fetch_client.fetch(url).await?;
    parse_news_from_html(&html)
}

fn parse_news_from_html(html: &str) -> Result<Vec<News>, YahooError> {
    let document = Html::parse_document(html);
    let mut news_list = Vec::new();

    // Try to find news items - this selector may need adjustment based on actual HTML structure
    let news_selector = Selector::parse("div[class*='news'], article, .news-item")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse news selector: {}", e)))?;

    for element in document.select(&news_selector) {
        // Try to extract title, link, source, img, time
        // This is a simplified parser - you may need to adjust based on actual HTML structure
        let title_selector = Selector::parse("h3 a, .title a, a[href*='news']")
            .map_err(|e| YahooError::ParseError(format!("Failed to parse title selector: {}", e)))?;
        
        if let Some(title_elem) = element.select(&title_selector).next() {
            let title = title_elem.text().collect::<String>().trim().to_string();
            let link = title_elem.value().attr("href")
                .map(|h| {
                    if h.starts_with("http") {
                        h.to_string()
                    } else {
                        format!("https://stockanalysis.com{}", h)
                    }
                })
                .unwrap_or_default();

            if !title.is_empty() && !link.is_empty() {
                news_list.push(News {
                    title,
                    link,
                    source: "StockAnalysis".to_string(),
                    img: String::new(),
                    time: String::new(),
                });
            }
        }
    }

    Ok(news_list)
}

