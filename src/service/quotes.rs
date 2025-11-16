use crate::client::{scraper, YahooFinanceClient};
use crate::client::error::YahooError;
use crate::models::{Quote, SimpleQuote};
use serde_json::Value;
use std::sync::Arc;

pub async fn get_quotes(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<crate::client::FetchClient>,
    symbols: &[&str],
) -> Result<Vec<Quote>, YahooError> {
    // Try API first
    match yahoo_client.get_simple_quotes(symbols).await {
        Ok(data) => {
            // Parse the response
            parse_quotes_from_api(data)
        }
        Err(_) => {
            // Fallback to scraping
            let mut quotes = Vec::new();
            for symbol in symbols {
                match scraper::scrape_quote(fetch_client, symbol).await {
                    Ok(quote_data) => {
                        // Convert scraped data to Quote
                        if let Ok(quote) = parse_quote_from_scraped(quote_data) {
                            quotes.push(quote);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to scrape quote for {}: {}", symbol, e);
                    }
                }
            }
            Ok(quotes)
        }
    }
}

pub async fn get_simple_quotes(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<crate::client::FetchClient>,
    symbols: &[&str],
) -> Result<Vec<SimpleQuote>, YahooError> {
    // Try API first
    match yahoo_client.get_simple_quotes(symbols).await {
        Ok(data) => {
            parse_simple_quotes_from_api(data)
        }
        Err(_) => {
            // Fallback to scraping
            let mut quotes = Vec::new();
            for symbol in symbols {
                match scraper::scrape_simple_quote(fetch_client, symbol).await {
                    Ok(quote_data) => {
                        if let Ok(quote) = parse_simple_quote_from_scraped(quote_data) {
                            quotes.push(quote);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to scrape simple quote for {}: {}", symbol, e);
                    }
                }
            }
            Ok(quotes)
        }
    }
}

fn parse_quotes_from_api(data: Value) -> Result<Vec<Quote>, YahooError> {
    // This is a simplified parser - you'll need to implement full parsing based on Yahoo's response structure
    let mut quotes = Vec::new();
    
    if let Some(quote_response) = data.get("quoteResponse") {
        if let Some(results) = quote_response.get("result").and_then(|r| r.as_array()) {
            for result in results {
                if let Ok(quote) = parse_quote_from_api_result(result) {
                    quotes.push(quote);
                }
            }
        }
    }
    
    Ok(quotes)
}

fn parse_quote_from_api_result(result: &Value) -> Result<Quote, YahooError> {
    Ok(Quote {
        symbol: result.get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        name: result.get("longName")
            .or_else(|| result.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price: result.get("regularMarketPrice")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .unwrap_or("0.0")
            .to_string(),
        pre_market_price: result.get("preMarketPrice")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .map(|s| s.to_string()),
        after_hours_price: result.get("postMarketPrice")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .map(|s| s.to_string()),
        change: result.get("regularMarketChange")
            .and_then(|c| c.get("fmt").or_else(|| c.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .unwrap_or("0.0")
            .to_string(),
        percent_change: result.get("regularMarketChangePercent")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| format!("{:.2}%", f).as_str())))
            )
            .unwrap_or("0.0%")
            .to_string(),
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
            for result in results {
                if let Ok(quote) = parse_simple_quote_from_api_result(result) {
                    quotes.push(quote);
                }
            }
        }
    }
    
    Ok(quotes)
}

fn parse_simple_quote_from_api_result(result: &Value) -> Result<SimpleQuote, YahooError> {
    Ok(SimpleQuote {
        symbol: result.get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        name: result.get("longName")
            .or_else(|| result.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price: result.get("regularMarketPrice")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .unwrap_or("0.0")
            .to_string(),
        pre_market_price: result.get("preMarketPrice")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .map(|s| s.to_string()),
        after_hours_price: result.get("postMarketPrice")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .map(|s| s.to_string()),
        change: result.get("regularMarketChange")
            .and_then(|c| c.get("fmt").or_else(|| c.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| f.to_string().as_str())))
            )
            .unwrap_or("0.0")
            .to_string(),
        percent_change: result.get("regularMarketChangePercent")
            .and_then(|p| p.get("fmt").or_else(|| p.get("raw"))
                .and_then(|v| v.as_str().or_else(|| v.as_f64().map(|f| format!("{:.2}%", f).as_str())))
            )
            .unwrap_or("0.0%")
            .to_string(),
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

