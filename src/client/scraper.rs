use crate::client::error::YahooError;
use crate::client::FetchClient;
use scraper::{Html, Selector};
use std::sync::Arc;
use tracing::{debug, warn};

pub async fn scrape_quote(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<serde_json::Value, YahooError> {
    let url = format!("https://finance.yahoo.com/quote/{}/", symbol);
    let html = fetch_client.fetch(&url).await?;

    let document = Html::parse_document(&html);

    // Extract company name
    let name_selector = Selector::parse("h1").map_err(|e| {
        YahooError::ParseError(format!("Failed to parse name selector: {}", e))
    })?;
    let name = document
        .select(&name_selector)
        .next()
        .and_then(|el| el.text().nth(1))
        .map(|s| s.split('(').next().unwrap_or("").trim().to_string())
        .unwrap_or_else(|| symbol.to_string());

    // Extract price data
    let price_selector = Selector::parse(r#"span[data-field="regularMarketPrice"]"#)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse price selector: {}", e)))?;
    let price = document
        .select(&price_selector)
        .next()
        .and_then(|el| el.text().next())
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    // Extract change
    let change_selector = Selector::parse(r#"span[data-field="regularMarketChange"]"#)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse change selector: {}", e)))?;
    let change = document
        .select(&change_selector)
        .next()
        .and_then(|el| el.text().next())
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    // Extract percent change
    let percent_change_selector = Selector::parse(r#"span[data-field="regularMarketChangePercent"]"#)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse percent change selector: {}", e)))?;
    let percent_change = document
        .select(&percent_change_selector)
        .next()
        .and_then(|el| el.text().next())
        .and_then(|s| s.trim().trim_end_matches('%').parse::<f64>().ok())
        .unwrap_or(0.0);

    Ok(serde_json::json!({
        "symbol": symbol.to_uppercase(),
        "name": name,
        "price": price,
        "change": change,
        "percent_change": percent_change,
    }))
}

pub async fn scrape_simple_quote(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<serde_json::Value, YahooError> {
    // For simple quotes, we can use the same scraping logic but return less data
    scrape_quote(fetch_client, symbol).await
}

pub async fn scrape_earnings_calls_list(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<Vec<serde_json::Value>, YahooError> {
    let url = format!("https://finance.yahoo.com/quote/{}/earnings-calls/", symbol);
    debug!("Fetching earnings calls page: {}", url);
    let html = fetch_client.fetch(&url).await?;
    debug!("Fetched HTML page, length: {} bytes", html.len());

    let document = Html::parse_document(&html);

    // Get all links first (matching Python's approach: tree.xpath("//a/@href"))
    let all_link_selector = Selector::parse("a[href]")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse all link selector: {}", e)))?;
    
    // Collect all href attributes from all links
    let all_links: Vec<String> = document
        .select(&all_link_selector)
        .filter_map(|link| link.value().attr("href").map(|s| s.to_string()))
        .collect();
    
    debug!("Found {} total links on the page", all_links.len());

    // Filter links containing "earnings_call" (matching Python: earnings_links = [link for link in all_links if "earnings_call" in link])
    let earnings_links: Vec<String> = all_links
        .into_iter()
        .filter(|link| link.contains("earnings_call"))
        .collect();
    
    debug!("Found {} links containing 'earnings_call'", earnings_links.len());

    let event_id_regex = regex::Regex::new(r"earnings_call-(\d+)")
        .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;
    let quarter_year_regex = regex::Regex::new(r"-([Qq]\d)-(\d{4})-earnings_call")
        .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;

    let mut calls = Vec::new();
    let mut seen_event_ids = std::collections::HashSet::new();

    for href in earnings_links {
        if let Some(captures) = event_id_regex.captures(&href) {
            if let Some(event_id_match) = captures.get(1) {
                let event_id = event_id_match.as_str();

                // Skip duplicates
                if seen_event_ids.contains(event_id) {
                    continue;
                }
                seen_event_ids.insert(event_id.to_string());

                // Extract quarter and year (matching Python implementation)
                let quarter = quarter_year_regex
                    .captures(&href)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_uppercase());
                let year = quarter_year_regex
                    .captures(&href)
                    .and_then(|c| c.get(2))
                    .and_then(|m| m.as_str().parse::<i32>().ok());

                let quarter_clone = quarter.clone();
                let year_clone = year;
                let title = if let (Some(ref q), Some(y)) = (quarter, year) {
                    format!("{} {}", q, y)
                } else {
                    "Earnings Call".to_string()
                };

                // Build URL - handle both absolute and relative URLs
                let url = if href.starts_with("http") {
                    href.clone()
                } else {
                    format!("https://finance.yahoo.com{}", href)
                };

                calls.push(serde_json::json!({
                    "eventId": event_id,
                    "quarter": quarter_clone,
                    "year": year_clone,
                    "title": title,
                    "url": url,
                }));
            }
        }
    }

    debug!("Parsed {} earnings calls from page", calls.len());
    if calls.is_empty() {
        warn!("No earnings_call links found on page. Page may require JavaScript rendering or structure has changed.");
        // Log sample links for debugging
        let sample_links: Vec<String> = document
            .select(&all_link_selector)
            .take(20)
            .filter_map(|link| link.value().attr("href").map(|s| s.to_string()))
            .filter(|link| link.contains("earnings") || link.contains("transcript") || link.contains("call"))
            .collect();
        if !sample_links.is_empty() {
            debug!("Sample earnings-related links found: {:?}", sample_links);
        }
    }

    Ok(calls)
}