use crate::client::{scraper, YahooFinanceClient};
use crate::client::error::YahooError;
use crate::models::{Quote, SimpleQuote};
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

pub async fn get_quotes(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<crate::client::FetchClient>,
    symbols: &[&str],
) -> Result<Vec<Quote>, YahooError> {
    info!("Fetching quotes for symbols: {:?}", symbols);
    
    // Try API first
    match yahoo_client.get_simple_quotes(symbols).await {
        Ok(data) => {
            debug!("Received API response: {}", serde_json::to_string(&data).unwrap_or_else(|_| "Failed to serialize".to_string()));
            
            // Log the structure to understand what we're getting
            if let Some(quote_response) = data.get("quoteResponse") {
                debug!("Found quoteResponse in data");
                if let Some(results) = quote_response.get("result") {
                    if let Some(results_array) = results.as_array() {
                        info!("Found {} results in quoteResponse.result", results_array.len());
                        for (idx, result) in results_array.iter().enumerate() {
                            debug!("Result {}: symbol={:?}, price keys={:?}", 
                                idx,
                                result.get("symbol"),
                                result.get("regularMarketPrice").map(|p| p.as_object().map(|o| o.keys().collect::<Vec<_>>()))
                            );
                        }
                    } else {
                        warn!("quoteResponse.result is not an array: {:?}", results);
                    }
                } else {
                    warn!("No 'result' field in quoteResponse");
                }
            } else {
                warn!("No 'quoteResponse' field in API response. Top-level keys: {:?}", 
                    data.as_object().map(|o| o.keys().collect::<Vec<_>>()));
            }
            
            // Parse the response
            let quotes = parse_quotes_from_api(data)?;
            info!("Successfully parsed {} quotes from API", quotes.len());
            Ok(quotes)
        }
        Err(e) => {
            warn!("API call failed: {}. Falling back to scraping", e);
            // Fallback to scraping
            let mut quotes = Vec::new();
            for symbol in symbols {
                match scraper::scrape_quote(fetch_client, symbol).await {
                    Ok(quote_data) => {
                        debug!("Scraped quote data for {}: {:?}", symbol, quote_data);
                        // Convert scraped data to Quote
                        if let Ok(quote) = parse_quote_from_scraped(quote_data) {
                            info!("Successfully parsed scraped quote for {}", symbol);
                            quotes.push(quote);
                        } else {
                            error!("Failed to parse scraped quote data for {}", symbol);
                        }
                    }
                    Err(e) => {
                        error!("Failed to scrape quote for {}: {}", symbol, e);
                    }
                }
            }
            info!("Returning {} quotes from scraping fallback", quotes.len());
            Ok(quotes)
        }
    }
}

pub async fn get_simple_quotes(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<crate::client::FetchClient>,
    symbols: &[&str],
) -> Result<Vec<SimpleQuote>, YahooError> {
    info!("Fetching simple quotes for symbols: {:?}", symbols);
    
    // Try API first
    match yahoo_client.get_simple_quotes(symbols).await {
        Ok(data) => {
            debug!("Received API response: {}", serde_json::to_string(&data).unwrap_or_else(|_| "Failed to serialize".to_string()));
            
            // Log the structure to understand what we're getting
            if let Some(quote_response) = data.get("quoteResponse") {
                debug!("Found quoteResponse in data");
                if let Some(results) = quote_response.get("result") {
                    if let Some(results_array) = results.as_array() {
                        info!("Found {} results in quoteResponse.result", results_array.len());
                    } else {
                        warn!("quoteResponse.result is not an array: {:?}", results);
                    }
                } else {
                    warn!("No 'result' field in quoteResponse");
                }
            } else {
                warn!("No 'quoteResponse' field in API response. Top-level keys: {:?}", 
                    data.as_object().map(|o| o.keys().collect::<Vec<_>>()));
            }
            
            let quotes = parse_simple_quotes_from_api(data)?;
            info!("Successfully parsed {} simple quotes from API", quotes.len());
            Ok(quotes)
        }
        Err(e) => {
            warn!("API call failed: {}. Falling back to scraping", e);
            // Fallback to scraping
            let mut quotes = Vec::new();
            for symbol in symbols {
                match scraper::scrape_simple_quote(fetch_client, symbol).await {
                    Ok(quote_data) => {
                        debug!("Scraped simple quote data for {}: {:?}", symbol, quote_data);
                        if let Ok(quote) = parse_simple_quote_from_scraped(quote_data) {
                            info!("Successfully parsed scraped simple quote for {}", symbol);
                            quotes.push(quote);
                        } else {
                            error!("Failed to parse scraped simple quote data for {}", symbol);
                        }
                    }
                    Err(e) => {
                        error!("Failed to scrape simple quote for {}: {}", symbol, e);
                    }
                }
            }
            info!("Returning {} simple quotes from scraping fallback", quotes.len());
            Ok(quotes)
        }
    }
}

fn parse_quotes_from_api(data: Value) -> Result<Vec<Quote>, YahooError> {
    let mut quotes = Vec::new();
    
    if let Some(quote_response) = data.get("quoteResponse") {
        if let Some(results) = quote_response.get("result").and_then(|r| r.as_array()) {
            info!("Parsing {} quote results", results.len());
            for (idx, result) in results.iter().enumerate() {
                match parse_quote_from_api_result(result) {
                    Ok(quote) => {
                        debug!("Successfully parsed quote {}: symbol={}, price={}", idx, quote.symbol, quote.price);
                        quotes.push(quote);
                    }
                    Err(e) => {
                        error!("Failed to parse quote result {}: {}", idx, e);
                    }
                }
            }
        } else {
            warn!("No 'result' array found in quoteResponse");
        }
    } else {
        warn!("No 'quoteResponse' found in data");
    }
    
    if quotes.is_empty() {
        warn!("No quotes parsed from API response. Response structure: {}", 
            serde_json::to_string(&data).unwrap_or_else(|_| "Failed to serialize".to_string()));
    }
    
    Ok(quotes)
}

fn parse_quote_from_api_result(result: &Value) -> Result<Quote, YahooError> {
    let symbol = result.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    
    debug!("Parsing quote for symbol: {}", symbol);
    
    // Check if regularMarketPrice exists and log its structure
    if let Some(price_val) = result.get("regularMarketPrice") {
        debug!("regularMarketPrice for {}: {:?}", symbol, price_val);
        if price_val.is_object() {
            debug!("regularMarketPrice is an object with keys: {:?}", 
                price_val.as_object().map(|o| o.keys().collect::<Vec<_>>()));
        } else if price_val.is_number() {
            debug!("regularMarketPrice is a number: {:?}", price_val.as_f64());
        } else if price_val.is_string() {
            debug!("regularMarketPrice is a string: {:?}", price_val.as_str());
        } else {
            warn!("regularMarketPrice has unexpected type for {}", symbol);
        }
    } else {
        warn!("No regularMarketPrice field found for {}", symbol);
    }
    
    let price = result.get("regularMarketPrice")
        .and_then(|p| {
            // Try to get as object first (fmt/raw structure)
            if let Some(obj) = p.as_object() {
                obj.get("fmt")
                    .or_else(|| obj.get("raw"))
                    .and_then(|v| v.as_str().map(|s| s.to_string())
                        .or_else(|| v.as_f64().map(|f| f.to_string())))
            } else if let Some(num) = p.as_f64() {
                // Direct number
                Some(num.to_string())
            } else if let Some(str_val) = p.as_str() {
                // Direct string
                Some(str_val.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            warn!("Could not extract price for {}, defaulting to 0.0", symbol);
            "0.0".to_string()
        });
    
    Ok(Quote {
        symbol,
        name: result.get("longName")
            .or_else(|| result.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price,
        pre_market_price: result.get("preMarketPrice")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().map(|s| s.to_string())
                    .or_else(|| v.as_f64().map(|f| f.to_string())))
            ),
        after_hours_price: result.get("postMarketPrice")
            .and_then(|p| {
                if let Some(obj) = p.as_object() {
                    obj.get("fmt")
                        .or_else(|| obj.get("raw"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| f.to_string())))
                } else if let Some(num) = p.as_f64() {
                    Some(num.to_string())
                } else if let Some(str_val) = p.as_str() {
                    Some(str_val.to_string())
                } else {
                    None
                }
            }),
        change: result.get("regularMarketChange")
            .and_then(|c| {
                if let Some(obj) = c.as_object() {
                    obj.get("fmt")
                        .or_else(|| obj.get("raw"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| f.to_string())))
                } else if let Some(num) = c.as_f64() {
                    Some(num.to_string())
                } else if let Some(str_val) = c.as_str() {
                    Some(str_val.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: result.get("regularMarketChangePercent")
            .and_then(|p| {
                if let Some(obj) = p.as_object() {
                    obj.get("fmt")
                        .or_else(|| obj.get("raw"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| format!("{:.2}%", f))))
                } else if let Some(num) = p.as_f64() {
                    Some(format!("{:.2}%", num))
                } else if let Some(str_val) = p.as_str() {
                    Some(str_val.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "0.0%".to_string()),
        open: None,
        high: None,
        low: None,
        year_high: None,
        year_low: None,
        volume: None,
        avg_volume: None,
        market_cap: None,
        beta: None,
        pe: None,
        eps: None,
        dividend: None,
        dividend_yield: None,
        ex_dividend: None,
        net_assets: None,
        nav: None,
        expense_ratio: None,
        category: None,
        last_capital_gain: None,
        morningstar_rating: None,
        morningstar_risk_rating: None,
        holdings_turnover: None,
        earnings_date: None,
        last_dividend: None,
        inception_date: None,
        sector: None,
        industry: None,
        about: None,
        employees: None,
        five_days_return: None,
        one_month_return: None,
        three_month_return: None,
        six_month_return: None,
        ytd_return: None,
        year_return: None,
        three_year_return: None,
        five_year_return: None,
        ten_year_return: None,
        max_return: None,
        logo: None,
    })
}

fn parse_simple_quotes_from_api(data: Value) -> Result<Vec<SimpleQuote>, YahooError> {
    let mut quotes = Vec::new();
    
    if let Some(quote_response) = data.get("quoteResponse") {
        if let Some(results) = quote_response.get("result").and_then(|r| r.as_array()) {
            info!("Parsing {} simple quote results", results.len());
            for (idx, result) in results.iter().enumerate() {
                match parse_simple_quote_from_api_result(result) {
                    Ok(quote) => {
                        debug!("Successfully parsed simple quote {}: symbol={}, price={}", idx, quote.symbol, quote.price);
                        quotes.push(quote);
                    }
                    Err(e) => {
                        error!("Failed to parse simple quote result {}: {}", idx, e);
                    }
                }
            }
        } else {
            warn!("No 'result' array found in quoteResponse");
        }
    } else {
        warn!("No 'quoteResponse' found in data");
    }
    
    if quotes.is_empty() {
        warn!("No simple quotes parsed from API response");
    }
    
    Ok(quotes)
}

fn parse_simple_quote_from_api_result(result: &Value) -> Result<SimpleQuote, YahooError> {
    let symbol = result.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    
    debug!("Parsing simple quote for symbol: {}", symbol);
    
    let price = result.get("regularMarketPrice")
        .and_then(|p| {
            if let Some(obj) = p.as_object() {
                obj.get("fmt")
                    .or_else(|| obj.get("raw"))
                    .and_then(|v| v.as_str().map(|s| s.to_string())
                        .or_else(|| v.as_f64().map(|f| f.to_string())))
            } else if let Some(num) = p.as_f64() {
                Some(num.to_string())
            } else if let Some(str_val) = p.as_str() {
                Some(str_val.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            warn!("Could not extract price for {}, defaulting to 0.0", symbol);
            "0.0".to_string()
        });
    
    Ok(SimpleQuote {
        symbol,
        name: result.get("longName")
            .or_else(|| result.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price,
        pre_market_price: result.get("preMarketPrice")
            .and_then(|p| {
                if let Some(obj) = p.as_object() {
                    obj.get("fmt")
                        .or_else(|| obj.get("raw"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| f.to_string())))
                } else if let Some(num) = p.as_f64() {
                    Some(num.to_string())
                } else if let Some(str_val) = p.as_str() {
                    Some(str_val.to_string())
                } else {
                    None
                }
            }),
        after_hours_price: result.get("postMarketPrice")
            .and_then(|p| {
                if let Some(obj) = p.as_object() {
                    obj.get("fmt")
                        .or_else(|| obj.get("raw"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| f.to_string())))
                } else if let Some(num) = p.as_f64() {
                    Some(num.to_string())
                } else if let Some(str_val) = p.as_str() {
                    Some(str_val.to_string())
                } else {
                    None
                }
            }),
        change: result.get("regularMarketChange")
            .and_then(|c| {
                if let Some(obj) = c.as_object() {
                    obj.get("fmt")
                        .or_else(|| obj.get("raw"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| f.to_string())))
                } else if let Some(num) = c.as_f64() {
                    Some(num.to_string())
                } else if let Some(str_val) = c.as_str() {
                    Some(str_val.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: result.get("regularMarketChangePercent")
            .and_then(|p| {
                if let Some(obj) = p.as_object() {
                    obj.get("fmt")
                        .or_else(|| obj.get("raw"))
                        .and_then(|v| v.as_str().map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| format!("{:.2}%", f))))
                } else if let Some(num) = p.as_f64() {
                    Some(format!("{:.2}%", num))
                } else if let Some(str_val) = p.as_str() {
                    Some(str_val.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "0.0%".to_string()),
        logo: None,
    })
}

fn parse_quote_from_scraped(data: Value) -> Result<Quote, YahooError> {
    Ok(Quote {
        symbol: data.get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        name: data.get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price: data.get("price")
            .and_then(|p| p.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        pre_market_price: None,
        after_hours_price: None,
        change: data.get("change")
            .and_then(|c| c.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: data.get("percent_change")
            .and_then(|p| p.as_f64().map(|f| format!("{:.2}%", f)))
            .unwrap_or_else(|| "0.0%".to_string()),
        open: None,
        high: None,
        low: None,
        year_high: None,
        year_low: None,
        volume: None,
        avg_volume: None,
        market_cap: None,
        beta: None,
        pe: None,
        eps: None,
        dividend: None,
        dividend_yield: None,
        ex_dividend: None,
        net_assets: None,
        nav: None,
        expense_ratio: None,
        category: None,
        last_capital_gain: None,
        morningstar_rating: None,
        morningstar_risk_rating: None,
        holdings_turnover: None,
        earnings_date: None,
        last_dividend: None,
        inception_date: None,
        sector: None,
        industry: None,
        about: None,
        employees: None,
        five_days_return: None,
        one_month_return: None,
        three_month_return: None,
        six_month_return: None,
        ytd_return: None,
        year_return: None,
        three_year_return: None,
        five_year_return: None,
        ten_year_return: None,
        max_return: None,
        logo: None,
    })
}

fn parse_simple_quote_from_scraped(data: Value) -> Result<SimpleQuote, YahooError> {
    Ok(SimpleQuote {
        symbol: data.get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        name: data.get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price: data.get("price")
            .and_then(|p| p.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        pre_market_price: None,
        after_hours_price: None,
        change: data.get("change")
            .and_then(|c| c.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: data.get("percent_change")
            .and_then(|p| p.as_f64().map(|f| format!("{:.2}%", f)))
            .unwrap_or_else(|| "0.0%".to_string()),
        logo: None,
    })
}