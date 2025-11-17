use crate::client::error::YahooError;
use crate::client::FetchClient;
use crate::models::News;
use scraper::{Html, Selector};
use std::sync::Arc;

pub async fn scrape_news_for_quote(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<Vec<News>, YahooError> {
    // Convert symbol to lowercase for stockanalysis.com URLs
    let symbol_lower = symbol.to_lowercase();
    
    // Try different URLs based on exchange (matching Python implementation)
    let urls = vec![
        format!("https://stockanalysis.com/stocks/{}", symbol_lower),
        format!("https://stockanalysis.com/etf/{}", symbol_lower),
        format!("https://stockanalysis.com/quote/otc/{}", symbol_lower),
    ];

    let mut network_errors = Vec::new();
    let mut parsing_errors = Vec::new();
    let mut no_news_found = false;

    for url in urls {
        tracing::debug!("Attempting to fetch news from: {}", url);
        
        // Use a longer timeout for news requests (30 seconds) as stockanalysis.com can be slow
        match fetch_client.fetch_with_timeout(&url, std::time::Duration::from_secs(30)).await {
            Ok(html) => {
                tracing::debug!("Successfully fetched HTML from: {} (length: {} bytes)", url, html.len());
                
                match parse_stockanalysis_news_from_html(&html) {
                    Ok(news_list) => {
                        tracing::debug!("Parsed {} news items from {}", news_list.len(), url);
                    if !news_list.is_empty() {
                        return Ok(news_list);
                        } else {
                            tracing::warn!("No news items found in HTML from {}", url);
                            no_news_found = true;
                            // Continue to try other URLs
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse news from {}: {:?}", url, e);
                        parsing_errors.push((url.clone(), e));
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to fetch from {}: {:?}", url, e);
                network_errors.push((url.clone(), e));
                // Continue to try other URLs
                continue;
            }
        }
    }

    // Return appropriate error based on what happened
    if !parsing_errors.is_empty() {
        // If we had parsing errors, return the first one
        // We need to clone or convert the error since we can't move out of the vector
        let error_msg = parsing_errors[0].1.to_string();
        Err(YahooError::ParseError(format!("Failed to parse news: {}", error_msg)))
    } else if no_news_found {
        // If we successfully parsed but found no news
        Err(YahooError::NotFound(format!("No news found for symbol: {}", symbol)))
    } else if !network_errors.is_empty() {
        // All URLs failed with network errors
        // Return a descriptive error message
        let last_url = &network_errors.last().unwrap().0;
        let error_msg = network_errors.last().unwrap().1.to_string();
        // Use ParseError for timeout cases, NetworkError for actual network issues
        if error_msg.contains("timed out") {
            Err(YahooError::ParseError(
                format!("All URLs timed out. Last attempted: {}", last_url)
            ))
        } else {
            // For actual network errors, we need to preserve the original error
            // Since we can't clone, we'll create a new error with the message
            Err(YahooError::ParseError(
                format!("All URLs failed. Last attempted: {}. Error: {}", last_url, error_msg)
            ))
        }
    } else {
    Err(YahooError::NotFound(format!("Could not find news for symbol: {}", symbol)))
    }
}

pub async fn scrape_general_news(
    fetch_client: &Arc<FetchClient>,
) -> Result<Vec<News>, YahooError> {
    let url = "https://stockanalysis.com/news/";
    tracing::debug!("Fetching general news from: {}", url);
    
    let html = fetch_client.fetch_with_timeout(url, std::time::Duration::from_secs(30)).await?;
    parse_stockanalysis_news_from_html(&html)
}

fn parse_stockanalysis_news_from_html(html: &str) -> Result<Vec<News>, YahooError> {
    let document = Html::parse_document(html);
    let mut news_list = Vec::new();

    tracing::debug!("Parsing HTML document (length: {} bytes)", html.len());

    // Based on Python implementation:
    // 1. First find the container (main > div[3] > div[2] > div > div[2] for stocks)
    // 2. Then within that container, find news items: div[div/h3/a and div/p and div/div[@title]]
    
    let main_selector = Selector::parse("main")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse main selector: {}", e)))?;
    
    let h3_a_selector = Selector::parse("h3 a")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse h3 a selector: {}", e)))?;
    
    let img_selector = Selector::parse("a img")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse img selector: {}", e)))?;
    
    let div_title_selector = Selector::parse("div[title]")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse div[title] selector: {}", e)))?;

    let p_selector = Selector::parse("p")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse p selector: {}", e)))?;

    // Count elements for debugging
    let main_count = document.select(&main_selector).count();
    let h3_count = document.select(&h3_a_selector).count();
    let div_title_count = document.select(&div_title_selector).count();
    let p_count = document.select(&p_selector).count();
    
    tracing::debug!("Found {} main elements, {} h3 a elements, {} div[title] elements, {} p elements", 
        main_count, h3_count, div_title_count, p_count);

    // Strategy 1: Try exact Python pattern (h3 a + p + div[title] all in same div)
    news_list = try_parse_with_pattern(&document, &h3_a_selector, &p_selector, &div_title_selector, &img_selector, true)?;
    
    if !news_list.is_empty() {
        tracing::debug!("Found {} news items using exact pattern (h3 a + p + div[title])", news_list.len());
        return Ok(news_list);
    }

    // Strategy 2: Fallback to less strict pattern (h3 a + div[title] without requiring p)
    tracing::debug!("Exact pattern found no results, trying fallback pattern (h3 a + div[title])");
    news_list = try_parse_with_pattern(&document, &h3_a_selector, &p_selector, &div_title_selector, &img_selector, false)?;
    
    if !news_list.is_empty() {
        tracing::debug!("Found {} news items using fallback pattern (h3 a + div[title])", news_list.len());
        return Ok(news_list);
    }

    // Strategy 3: Try searching within main container more specifically
    if let Some(main) = document.select(&main_selector).next() {
        tracing::debug!("Trying to find news within main container");
        // Try to navigate to the specific container path: main > div[3] > div[2] > div > div[2]
        let div_selector = Selector::parse("div")
            .map_err(|e| YahooError::ParseError(format!("Failed to parse div selector: {}", e)))?;
        
        // Get all divs within main
        let main_divs: Vec<_> = main.select(&div_selector).collect();
        tracing::debug!("Found {} divs within main element", main_divs.len());
        
        // Try to find news in nested divs
        for div in &main_divs {
            let nested_news = try_parse_with_pattern_from_container(div, &h3_a_selector, &p_selector, &div_title_selector, &img_selector, false)?;
            if !nested_news.is_empty() {
                tracing::debug!("Found {} news items in nested divs", nested_news.len());
                return Ok(nested_news);
            }
        }
    }

    // If still no news, log HTML snippet for debugging
    if news_list.is_empty() {
        let html_preview = html.chars().take(1000).collect::<String>();
        tracing::debug!("No news found after all strategies. HTML preview (first 1000 chars):\n{}", html_preview);
        tracing::debug!("Element counts - h3 a: {}, div[title]: {}, p: {}", h3_count, div_title_count, p_count);
    }

    Ok(news_list)
}

fn try_parse_with_pattern(
    document: &Html,
    h3_a_selector: &Selector,
    p_selector: &Selector,
    div_title_selector: &Selector,
    img_selector: &Selector,
    require_p: bool,
) -> Result<Vec<News>, YahooError> {
    let mut news_list = Vec::new();
    let all_divs_selector = Selector::parse("div")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse div selector: {}", e)))?;

    for news_div in document.select(&all_divs_selector) {
        let has_h3_a = news_div.select(h3_a_selector).next().is_some();
        let has_p = news_div.select(p_selector).next().is_some();
        let has_div_title = news_div.select(div_title_selector).next().is_some();
        
        // Check pattern based on requirement
        if !has_h3_a || !has_div_title {
            continue;
        }
        if require_p && !has_p {
            continue;
        }

        if let Some(news) = extract_news_item(&news_div, h3_a_selector, img_selector, div_title_selector) {
            news_list.push(news);
        }
    }

    Ok(news_list)
}

fn try_parse_with_pattern_from_container(
    container: &scraper::ElementRef,
    h3_a_selector: &Selector,
    p_selector: &Selector,
    div_title_selector: &Selector,
    img_selector: &Selector,
    require_p: bool,
) -> Result<Vec<News>, YahooError> {
    let mut news_list = Vec::new();
    let all_divs_selector = Selector::parse("div")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse div selector: {}", e)))?;

    for news_div in container.select(&all_divs_selector) {
        let has_h3_a = news_div.select(h3_a_selector).next().is_some();
        let has_p = news_div.select(p_selector).next().is_some();
        let has_div_title = news_div.select(div_title_selector).next().is_some();
        
        if !has_h3_a || !has_div_title {
            continue;
        }
        if require_p && !has_p {
            continue;
        }

        if let Some(news) = extract_news_item(&news_div, h3_a_selector, img_selector, div_title_selector) {
            news_list.push(news);
        }
    }

    Ok(news_list)
}

fn extract_news_item(
    news_div: &scraper::ElementRef,
    h3_a_selector: &Selector,
    img_selector: &Selector,
    div_title_selector: &Selector,
) -> Option<News> {
    // Extract title and link from h3 a
    let title_elem = news_div.select(h3_a_selector).next()?;
    
            let title = title_elem.text().collect::<String>().trim().to_string();
    if title.is_empty() {
        return None;
    }

            let link = title_elem.value().attr("href")
                .map(|h| {
                    if h.starts_with("http") {
                        h.to_string()
            } else if h.starts_with("/") {
                format!("https://stockanalysis.com{}", h)
            } else {
                format!("https://stockanalysis.com/{}", h)
            }
        })?;
    
    if link.is_empty() {
        return None;
    }

    // Extract image from a img
    let img = news_div.select(img_selector)
        .next()
        .and_then(|img_elem| img_elem.value().attr("src"))
        .map(|src| {
            if src.starts_with("http") {
                src.to_string()
            } else if src.starts_with("/") {
                format!("https://stockanalysis.com{}", src)
                    } else {
                format!("https://stockanalysis.com/{}", src)
                    }
                })
                .unwrap_or_default();

    // Extract source and time from div[title] - Python splits by " - "
    let (time, source) = news_div.select(div_title_selector)
        .next()
        .and_then(|div| div.text().next())
        .map(|text| {
            let text = text.trim();
            // Python implementation: time, source = source_date.split(" - ")
            if let Some(pos) = text.find(" - ") {
                let time = text[..pos].trim().to_string();
                let source = text[pos + 3..].trim().to_string();
                (time, source)
            } else {
                (text.to_string(), "StockAnalysis".to_string())
            }
        })
        .unwrap_or_else(|| (String::new(), "StockAnalysis".to_string()));

    Some(News {
        title,
        link,
        source,
        img,
        time,
    })
}