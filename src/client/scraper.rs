use crate::client::error::YahooError;
use crate::client::FetchClient;
use scraper::{Html, Selector};
use std::sync::Arc;

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
    let html = fetch_client.fetch(&url).await?;

    let document = Html::parse_document(&html);

    // Find all links containing "earnings_call"
    let link_selector = Selector::parse("a[href*='earnings_call']")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse link selector: {}", e)))?;

    let event_id_regex = regex::Regex::new(r"earnings_call-(\d+)")
        .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;
    let quarter_year_regex = regex::Regex::new(r"-([Qq]\d)-(\d{4})-earnings_call")
        .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;

    let mut calls = Vec::new();
    let mut seen_event_ids = std::collections::HashSet::new();

    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            if let Some(captures) = event_id_regex.captures(href) {
                if let Some(event_id_match) = captures.get(1) {
                    let event_id = event_id_match.as_str();

                    if seen_event_ids.contains(event_id) {
                        continue;
                    }
                    seen_event_ids.insert(event_id.to_string());

                    let quarter = quarter_year_regex
                        .captures(href)
                        .and_then(|c| c.get(1))
                        .map(|m| m.as_str().to_uppercase());
                    let year = quarter_year_regex
                        .captures(href)
                        .and_then(|c| c.get(2))
                        .and_then(|m| m.as_str().parse::<i32>().ok());

                    let title = if let (Some(q), Some(y)) = (quarter, year) {
                        format!("{} {}", q, y)
                    } else {
                        "Earnings Call".to_string()
                    };

                    calls.push(serde_json::json!({
                        "eventId": event_id,
                        "quarter": quarter,
                        "year": year,
                        "title": title,
                        "url": format!("https://finance.yahoo.com{}", href),
                    }));
                }
            }
        }
    }

    Ok(calls)
}

