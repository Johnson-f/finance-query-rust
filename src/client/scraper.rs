use crate::client::error::YahooError;
use crate::client::FetchClient;
use scraper::{Html, Selector};
use std::sync::Arc;
use tracing::{debug, warn};
use serde_json::Value;
use regex::Regex;

pub async fn scrape_quote(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<serde_json::Value, YahooError> {
    let url = format!("https://finance.yahoo.com/quote/{}/", symbol);
    debug!("Scraping quote from: {}", url);
    let html = fetch_client.fetch(&url).await?;
    debug!("Fetched HTML, length: {} bytes", html.len());

    let document = Html::parse_document(&html);

    // Extract company name from h1
    let name_selector = Selector::parse("h1").map_err(|e| {
        YahooError::ParseError(format!("Failed to parse name selector: {}", e))
    })?;
    let name = document
        .select(&name_selector)
        .next()
        .map(|el| {
            let full_text: String = el.text().collect();
            // Format is usually "Company Name (SYMBOL)"
            full_text.split('(').next().unwrap_or(&full_text).trim().to_string()
        })
        .unwrap_or_else(|| symbol.to_string());
    
    debug!("Extracted name: {}", name);

    // NEW: Try multiple strategies to extract price data
    
    // Strategy 1: Look for fin-streamer elements (current Yahoo Finance format)
    let price = extract_fin_streamer_value(&document, "regularMarketPrice")
        .or_else(|| extract_data_field_value(&document, "regularMarketPrice"))
        .or_else(|| extract_from_json_ld(&document, "price"))
        .unwrap_or(0.0);
    
    let change = extract_fin_streamer_value(&document, "regularMarketChange")
        .or_else(|| extract_data_field_value(&document, "regularMarketChange"))
        .unwrap_or(0.0);
    
    let percent_change = extract_fin_streamer_value(&document, "regularMarketChangePercent")
        .or_else(|| extract_data_field_value(&document, "regularMarketChangePercent"))
        .unwrap_or(0.0);

    debug!("Extracted values - price: {}, change: {}, percent_change: {}", price, change, percent_change);

    Ok(serde_json::json!({
        "symbol": symbol.to_uppercase(),
        "name": name,
        "price": price,
        "change": change,
        "percent_change": percent_change,
    }))
}

// Extract value from fin-streamer elements (current Yahoo format)
fn extract_fin_streamer_value(document: &Html, data_field: &str) -> Option<f64> {
    let selector = Selector::parse(&format!(r#"fin-streamer[data-field="{}"]"#, data_field)).ok()?;
    document
        .select(&selector)
        .next()
        .and_then(|el| {
            // Try data-value attribute first
            el.value().attr("data-value")
                .and_then(|v| v.parse::<f64>().ok())
                .or_else(|| {
                    // Fallback to text content
                    let text: String = el.text().collect();
                    parse_numeric_value(&text)
                })
        })
}

// Fallback: Extract from old data-field format
fn extract_data_field_value(document: &Html, data_field: &str) -> Option<f64> {
    let selector = Selector::parse(&format!(r#"span[data-field="{}"]"#, data_field)).ok()?;
    document
        .select(&selector)
        .next()
        .and_then(|el| {
            let text: String = el.text().collect();
            parse_numeric_value(&text)
        })
}

// Extract from JSON-LD structured data
fn extract_from_json_ld(document: &Html, field: &str) -> Option<f64> {
    let script_selector = Selector::parse(r#"script[type="application/ld+json"]"#).ok()?;
    
    for script in document.select(&script_selector) {
        let json_text: String = script.text().collect();
        if let Ok(json) = serde_json::from_str::<Value>(&json_text) {
            if field == "price" {
                if let Some(price_val) = json.get("price").or_else(|| json.get("offers").and_then(|o| o.get("price"))) {
                    if let Some(price_str) = price_val.as_str() {
                        return parse_numeric_value(price_str);
                    } else if let Some(price_num) = price_val.as_f64() {
                        return Some(price_num);
                    }
                }
            }
        }
    }
    None
}

// Parse numeric value from string, handling commas, currency symbols, percentages
fn parse_numeric_value(text: &str) -> Option<f64> {
    let cleaned = text
        .trim()
        .replace(',', "")
        .replace('$', "")
        .replace('%', "")
        .replace('+', "")
        .trim()
        .to_string();
    
    cleaned.parse::<f64>().ok()
}

pub async fn scrape_simple_quote(
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<serde_json::Value, YahooError> {
    // For simple quotes, we can use the same scraping logic but return less data
    scrape_quote(fetch_client, symbol).await
}

// Helper function to extract a JSON object from a string
fn extract_json_object(json_str: &str) -> Result<Value, YahooError> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    let mut json_end = 0;
    
    for (i, ch) in json_str.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }
        
        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => {
                if brace_count == 0 {
                    // This is the start
                }
                brace_count += 1;
            }
            '}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    json_end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    
    if json_end > 0 {
        let json_slice = &json_str[..json_end];
        serde_json::from_str(json_slice)
            .map_err(|e| YahooError::ParseError(format!("Failed to parse JSON: {}", e)))
    } else {
        Err(YahooError::ParseError("Could not find complete JSON object".to_string()))
    }
}

// Helper function to find transcript data in nested JSON structure (root.App.main[0][3][1][0])
fn find_transcript_in_nested_json(value: &Value) -> Option<Value> {
    // Try common paths in Yahoo Finance structure
    let paths = vec![
        vec!["0", "3", "1", "0"],
        vec!["0", "3", "1"],
        vec!["0", "3"],
        vec!["0"],
    ];
    
    for path in paths {
        let mut current = value;
        let mut found = true;
        
        for key in &path {
            if let Ok(index) = key.parse::<usize>() {
                if let Some(arr) = current.as_array() {
                    if let Some(item) = arr.get(index) {
                        current = item;
                    } else {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }
        }
        
        if found {
            // Check if this contains transcript data
            if current.get("transcriptContent").is_some() {
                return Some(current.clone());
            }
            // Recursively search in this object
            if let Some(transcript) = search_for_transcript(current) {
                return Some(transcript);
            }
        }
    }
    
    // Fallback: recursive search
    search_for_transcript(value)
}

fn search_for_transcript(value: &Value) -> Option<Value> {
    match value {
        Value::Object(map) => {
            if map.contains_key("transcriptContent") {
                return Some(Value::Object(map.clone()));
            }
            for v in map.values() {
                if let Some(result) = search_for_transcript(v) {
                    return Some(result);
                }
            }
            None
        }
        Value::Array(arr) => {
            for item in arr {
                if let Some(result) = search_for_transcript(item) {
                    return Some(result);
                }
            }
            None
        }
        _ => None,
    }
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

    let event_id_regex = Regex::new(r"earnings_call-(\d+)")
        .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;
    let quarter_year_regex = Regex::new(r"-([Qq]\d)-(\d{4})-earnings_call")
        .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;

    let mut calls = Vec::new();
    let mut seen_event_ids = std::collections::HashSet::new();

    for href in earnings_links {
        if let Some(captures) = event_id_regex.captures(&href)
            && let Some(event_id_match) = captures.get(1)
        {
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

pub async fn scrape_earnings_transcript_from_url(
    fetch_client: &Arc<FetchClient>,
    url: &str,
) -> Result<Value, YahooError> {
    debug!("Fetching earnings transcript from URL: {}", url);
    let html = fetch_client.fetch(url).await?;
    debug!("Fetched HTML page, length: {} bytes", html.len());

    let document = Html::parse_document(&html);

    // Try to extract embedded JSON data from script tags (common pattern for Yahoo Finance)
    let script_selector = Selector::parse("script")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse script selector: {}", e)))?;
    
    // Pre-compile regex outside the loop for Strategy 2
    let transcript_content_regex = Regex::new(r#""transcriptContent"\s*:\s*\{[^}]*\}"#)
        .map_err(|e| YahooError::ParseError(format!("Regex error: {}", e)))?;
    
    // Look for script tags containing transcript data
    for script in document.select(&script_selector) {
        let script_text: String = script.text().collect();
        
        // Look for common patterns like "transcriptContent" or "root.App.main"
        if script_text.contains("transcriptContent") || script_text.contains("root.App.main") {
            // Try multiple extraction strategies
            
            // Strategy 1: Look for root.App.main pattern
            if script_text.contains("root.App.main")
                && let Some(start) = script_text.find("root.App.main")
            {
                let after_main = &script_text[start..];
                // Find the assignment
                if let Some(assign_pos) = after_main.find('=') {
                    let json_start = &after_main[assign_pos + 1..].trim_start();
                    if let Some(brace_start) = json_start.find('{') {
                        let json_str = &json_start[brace_start..];
                        if let Ok(parsed) = extract_json_object(json_str) {
                            // Navigate through the structure: root.App.main[0][3][1][0]
                            if let Some(transcript_data) = find_transcript_in_nested_json(&parsed) {
                                debug!("Found transcript data in root.App.main");
                                return Ok(transcript_data);
                            }
                        }
                    }
                }
            }
            
            // Strategy 2: Direct transcriptContent pattern
            if script_text.contains("transcriptContent")
                && let Some(captures) = transcript_content_regex.find(&script_text)
            {
                // Try to extract a larger JSON context
                let start = captures.start().saturating_sub(100);
                let end = (captures.end() + 1000).min(script_text.len());
                let json_candidate = &script_text[start..end];
                
                // Find the opening brace before transcriptContent
                if let Some(brace_pos) = json_candidate.rfind('{') {
                    let json_str = &json_candidate[brace_pos..];
                    if let Ok(parsed) = extract_json_object(json_str)
                        && parsed.get("transcriptContent").is_some()
                    {
                        debug!("Found transcript data via transcriptContent pattern");
                        return Ok(parsed);
                    }
                }
            }
        }
    }

    // Fallback: Try to parse transcript from DOM structure
    // Look for common transcript container selectors
    let transcript_selectors = vec![
        "div[data-module='Transcript']",
        "div.transcript",
        "div#transcript",
        "section[data-testid='transcript']",
    ];

    for selector_str in transcript_selectors {
        if let Ok(selector) = Selector::parse(selector_str)
            && document.select(&selector).next().is_some()
        {
            debug!("Found transcript container with selector: {}", selector_str);
            // Extract transcript from DOM
            return extract_transcript_from_dom(&document, selector_str);
        }
    }

    // If no structured data found, try to extract from common patterns
    warn!("Could not find structured transcript data, attempting generic extraction");
    extract_transcript_from_dom_generic(&document)
}

fn extract_transcript_from_dom(
    document: &Html,
    container_selector: &str,
) -> Result<Value, YahooError> {
    // This is a placeholder - actual implementation would parse the DOM
    // For now, return a structure that matches what parse_transcript expects
    let container_sel = Selector::parse(container_selector)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse container selector: {}", e)))?;
    
    let mut paragraphs = Vec::new();
    let mut speakers = Vec::new();
    let mut speaker_mapping = std::collections::HashMap::new();

    // Try to find speaker elements and transcript paragraphs
    let speaker_selector = Selector::parse("div[class*='speaker'], span[class*='speaker'], strong")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse speaker selector: {}", e)))?;
    
    let text_selector = Selector::parse("p, div[class*='text'], div[class*='paragraph']")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse text selector: {}", e)))?;

    if let Some(container) = document.select(&container_sel).next() {
        let mut current_speaker = "Unknown".to_string();
        
        for element in container.select(&text_selector) {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                // Check if this element contains a speaker name
                if let Some(speaker_elem) = element.select(&speaker_selector).next() {
                    let speaker_name = speaker_elem.text().collect::<String>().trim().to_string();
                    if !speaker_name.is_empty() {
                        current_speaker = speaker_name.clone();
                        if !speaker_mapping.contains_key(&current_speaker) {
                            let speaker_id = format!("speaker_{}", speakers.len());
                            speaker_mapping.insert(current_speaker.clone(), speaker_id.clone());
                            speakers.push(serde_json::json!({
                                "speaker": speaker_id,
                                "speaker_data": {
                                    "name": current_speaker,
                                    "role": None::<String>,
                                    "company": None::<String>
                                }
                            }));
                        }
                    }
                }
                
                paragraphs.push(serde_json::json!({
                    "speaker": speaker_mapping.get(&current_speaker).cloned().unwrap_or_else(|| "unknown".to_string()),
                    "text": text
                }));
            }
        }
    }

    // Build response structure matching API format
    Ok(serde_json::json!({
        "transcriptContent": {
            "speaker_mapping": speakers,
            "transcript": {
                "paragraphs": paragraphs
            }
        },
        "transcriptMetadata": {
            "fiscalYear": None::<i32>,
            "fiscalPeriod": None::<String>,
            "title": None::<String>,
            "date": None::<i64>,
            "eventType": "Earnings Call",
            "isLatest": false
        }
    }))
}

fn extract_transcript_from_dom_generic(document: &Html) -> Result<Value, YahooError> {
    // Generic extraction - look for any text content that might be transcript
    let mut paragraphs = Vec::new();
    let mut speakers = Vec::new();
    
    // Try to find any content that looks like a transcript
    let content_selectors = vec![
        "div[class*='transcript']",
        "div[class*='earnings']",
        "article",
        "main",
    ];

    for selector_str in content_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for container in document.select(&selector) {
                let text = container.text().collect::<String>();
                if text.len() > 1000 {  // Likely a transcript if it's long
                    // Split by common patterns (speaker names, timestamps, etc.)
                    let lines: Vec<&str> = text.lines().collect();
                    let mut current_speaker = "Unknown".to_string();
                    
                    for line in lines {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        
                        // Check if line looks like a speaker name (short, possibly in caps or bold)
                        if trimmed.len() < 50 && (trimmed.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c == ':') || trimmed.ends_with(':')) {
                            current_speaker = trimmed.trim_end_matches(':').trim().to_string();
                            if !speakers.iter().any(|s: &Value| s.get("speaker_data").and_then(|sd| sd.get("name")).and_then(|n| n.as_str()) == Some(&current_speaker)) {
                                let speaker_id = format!("speaker_{}", speakers.len());
                                speakers.push(serde_json::json!({
                                    "speaker": speaker_id,
                                    "speaker_data": {
                                        "name": current_speaker.clone(),
                                        "role": None::<String>,
                                        "company": None::<String>
                                    }
                                }));
                            }
                        } else if trimmed.len() > 20 {
                            // Likely transcript text
                            // Find or create speaker ID for current_speaker
                            let speaker_id = if speakers.is_empty() {
                                // Create a default speaker if none exists
                                let default_id = "speaker_0".to_string();
                                speakers.push(serde_json::json!({
                                    "speaker": default_id.clone(),
                                    "speaker_data": {
                                        "name": current_speaker.clone(),
                                        "role": None::<String>,
                                        "company": None::<String>
                                    }
                                }));
                                default_id
                            } else {
                                // Find existing speaker or use the last one
                                speakers.iter()
                                    .find(|s| {
                                        s.get("speaker_data")
                                            .and_then(|sd| sd.get("name"))
                                            .and_then(|n| n.as_str())
                                            .map(|n| n == current_speaker)
                                            .unwrap_or(false)
                                    })
                                    .and_then(|s| s.get("speaker").and_then(|id| id.as_str()))
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| {
                                        format!("speaker_{}", speakers.len() - 1)
                                    })
                            };
                            
                            paragraphs.push(serde_json::json!({
                                "speaker": speaker_id,
                                "text": trimmed
                            }));
                        }
                    }
                    
                    if !paragraphs.is_empty() {
                        break;
                    }
                }
            }
        }
    }

    // Build response structure
    Ok(serde_json::json!({
        "transcriptContent": {
            "speaker_mapping": speakers,
            "transcript": {
                "paragraphs": paragraphs
            }
        },
        "transcriptMetadata": {
            "fiscalYear": None::<i32>,
            "fiscalPeriod": None::<String>,
            "title": None::<String>,
            "date": None::<i64>,
            "eventType": "Earnings Call",
            "isLatest": false
        }
    }))
}