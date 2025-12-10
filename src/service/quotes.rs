use crate::service::logo;
use finance_query_core::client::FetchClient;
use finance_query_core::client::error::YahooError;
use finance_query_core::client::{YahooFinanceClient, scraper};
use finance_query_core::models::{Quote, SimpleQuote};
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

pub async fn get_quotes(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbols: &[&str],
) -> Result<Vec<Quote>, YahooError> {
    info!("Fetching quotes for symbols: {:?}", symbols);

    // Check if we should force scraping (for local development/testing)
    let force_scraping = std::env::var("FORCE_SCRAPING").is_ok();

    // Fetch detailed quotes using quoteSummary endpoint for each symbol
    let mut quotes = Vec::new();
    for symbol in symbols {
        // If force_scraping is enabled, skip API and go straight to scraping
        if force_scraping {
            info!("FORCE_SCRAPING enabled, using scraper for {}", symbol);
            match scraper::scrape_quote(fetch_client, symbol).await {
                Ok(quote_data) => {
                    if let Ok(mut quote) = parse_quote_from_scraped(quote_data) {
                        info!("Successfully parsed scraped quote for {}", symbol);
                        quote.logo = logo::get_logo(fetch_client, Some(symbol), None).await;
                        quotes.push(quote);
                    }
                }
                Err(e) => {
                    error!("Failed to scrape quote for {}: {}", symbol, e);
                }
            }
            continue;
        }

        match yahoo_client.get_quote(symbol).await {
            Ok(data) => {
                debug!(
                    "Received quoteSummary response for {}: {}",
                    symbol,
                    serde_json::to_string(&data)
                        .unwrap_or_else(|_| "Failed to serialize".to_string())
                );

                // Parse quoteSummary format
                match parse_quote_from_summary(data, fetch_client, symbol).await {
                    Ok(quote) => {
                        info!("Successfully parsed detailed quote for {}", symbol);
                        quotes.push(quote);
                    }
                    Err(e) => {
                        error!("Failed to parse quoteSummary for {}: {}", symbol, e);
                        // Try fallback to scraping
                        warn!("Falling back to scraping for {}", symbol);
                        if let Ok(quote_data) = scraper::scrape_quote(fetch_client, symbol).await
                            && let Ok(mut quote) = parse_quote_from_scraped(quote_data)
                        {
                            // Fetch logo for scraped quote
                            quote.logo = logo::get_logo(fetch_client, Some(symbol), None).await;
                            quotes.push(quote);
                        }
                    }
                }
            }
            Err(e) => {
                warn!(
                    "API call failed for {}: {}. Falling back to scraping",
                    symbol, e
                );
                // Fallback to scraping
                match scraper::scrape_quote(fetch_client, symbol).await {
                    Ok(quote_data) => {
                        debug!("Scraped quote data for {}: {:?}", symbol, quote_data);
                        if let Ok(mut quote) = parse_quote_from_scraped(quote_data) {
                            info!("Successfully parsed scraped quote for {}", symbol);
                            // Fetch logo for scraped quote
                            quote.logo = logo::get_logo(fetch_client, Some(symbol), None).await;
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
        }
    }

    info!("Returning {} quotes", quotes.len());
    Ok(quotes)
}

pub async fn get_simple_quotes(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbols: &[&str],
) -> Result<Vec<SimpleQuote>, YahooError> {
    info!("Fetching simple quotes for symbols: {:?}", symbols);

    // Try API first
    match yahoo_client.get_simple_quotes(symbols).await {
        Ok(data) => {
            debug!(
                "Received API response: {}",
                serde_json::to_string(&data).unwrap_or_else(|_| "Failed to serialize".to_string())
            );

            // Log the structure to understand what we're getting
            if let Some(quote_response) = data.get("quoteResponse") {
                debug!("Found quoteResponse in data");
                if let Some(results) = quote_response.get("result") {
                    if let Some(results_array) = results.as_array() {
                        info!(
                            "Found {} results in quoteResponse.result",
                            results_array.len()
                        );
                    } else {
                        warn!("quoteResponse.result is not an array: {:?}", results);
                    }
                } else {
                    warn!("No 'result' field in quoteResponse");
                }
            } else {
                warn!(
                    "No 'quoteResponse' field in API response. Top-level keys: {:?}",
                    data.as_object().map(|o| o.keys().collect::<Vec<_>>())
                );
            }

            let quotes = parse_simple_quotes_from_api(data)?;
            // Skip logo fetching for WebSocket performance
            info!(
                "Successfully parsed {} simple quotes from API",
                quotes.len()
            );
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
                            // Skip logo fetching for WebSocket performance
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
            info!(
                "Returning {} simple quotes from scraping fallback",
                quotes.len()
            );
            Ok(quotes)
        }
    }
}

// Helper function to extract formatted value from Yahoo API response
fn get_fmt(data: &Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| {
        if let Some(obj) = v.as_object() {
            obj.get("fmt").or_else(|| obj.get("raw")).and_then(|val| {
                val.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| val.as_f64().map(|f| f.to_string()))
            })
        } else if let Some(num) = v.as_f64() {
            Some(num.to_string())
        } else {
            v.as_str().map(|str_val| str_val.to_string())
        }
    })
}

// Helper function to extract raw numeric value
fn get_raw(data: &Value, key: &str) -> Option<i64> {
    data.get(key).and_then(|v| {
        if let Some(obj) = v.as_object() {
            obj.get("raw").and_then(|r| r.as_i64())
        } else {
            v.as_i64()
        }
    })
}

// Helper function to format date
#[allow(dead_code)]
fn format_date(date_val: Option<&Value>) -> Option<String> {
    date_val.and_then(|d| {
        if let Some(str_val) = d.as_str() {
            Some(str_val.to_string())
        } else {
            // Convert epoch timestamp to date string if needed
            d.as_i64().map(|num| num.to_string())
        }
    })
}

async fn parse_quote_from_summary(
    data: Value,
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<Quote, YahooError> {
    let summary_result = data
        .get("quoteSummary")
        .and_then(|qs| qs.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .ok_or_else(|| YahooError::ParseError("No quoteSummary result found".to_string()))?;

    let price_data = summary_result.get("price").unwrap_or(&Value::Null);
    let summary_detail = summary_result.get("summaryDetail").unwrap_or(&Value::Null);
    let stats = summary_result
        .get("defaultKeyStatistics")
        .unwrap_or(&Value::Null);
    let profile = summary_result.get("assetProfile").unwrap_or(&Value::Null);
    let calendar = summary_result.get("calendarEvents").unwrap_or(&Value::Null);
    let performance_overview = summary_result
        .get("quoteUnadjustedPerformanceOverview")
        .and_then(|q| q.get("performanceOverview"))
        .unwrap_or(&Value::Null);

    // Extract website URL for logo fetching
    let website_url = profile.get("website").and_then(|w| w.as_str());

    // Parse earnings dates
    let earnings_date = calendar
        .get("earnings")
        .and_then(|e| e.get("earningsDate"))
        .and_then(|ed| ed.as_array())
        .map(|dates| {
            dates
                .iter()
                .filter_map(|d| get_fmt(d, "fmt"))
                .collect::<Vec<_>>()
                .join(" - ")
        })
        .filter(|s| !s.is_empty());

    // Parse ex-dividend date
    let ex_dividend = calendar
        .get("exDividendDate")
        .and_then(|d| get_fmt(d, "fmt"));

    // Parse inception date
    let inception_date = stats.get("fundInceptionDate").and_then(|d| {
        if let Some(raw) = d.get("raw").and_then(|r| r.as_i64()) {
            // Convert epoch to date string if needed
            Some(raw.to_string())
        } else {
            get_fmt(d, "fmt")
        }
    });

    // Parse morningstar rating
    let morningstar_rating = stats
        .get("morningStarOverallRating")
        .and_then(|r| r.get("raw").and_then(|raw| raw.as_i64()))
        .map(|rating| {
            if rating > 0 {
                "★".repeat(rating as usize)
            } else {
                String::new()
            }
        })
        .filter(|s| !s.is_empty());

    // Parse morningstar risk rating
    let morningstar_risk_rating = stats
        .get("morningStarRiskRating")
        .and_then(|r| r.get("raw").and_then(|raw| raw.as_i64()))
        .map(|risk| {
            match risk {
                1 => "Low",
                2 => "Below Average",
                3 => "Average",
                4 => "Above Average",
                5 => "High",
                _ => "Unknown",
            }
            .to_string()
        });

    // Parse employees
    let employees = profile
        .get("fullTimeEmployees")
        .and_then(|e| e.as_i64())
        .map(|e| e.to_string());

    Ok(Quote {
        symbol: price_data
            .get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        name: price_data
            .get("longName")
            .or_else(|| price_data.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price: get_fmt(price_data, "regularMarketPrice").unwrap_or_else(|| "0.0".to_string()),
        pre_market_price: price_data
            .get("preMarketTime")
            .and_then(|t| t.as_i64())
            .filter(|&time| time > 0)
            .and_then(|_| get_fmt(price_data, "preMarketPrice")),
        after_hours_price: price_data
            .get("postMarketTime")
            .and_then(|t| t.as_i64())
            .filter(|&time| time > 0)
            .and_then(|_| get_fmt(price_data, "postMarketPrice")),
        change: get_fmt(price_data, "regularMarketChange").unwrap_or_else(|| "0.0".to_string()),
        percent_change: get_fmt(price_data, "regularMarketChangePercent")
            .unwrap_or_else(|| "0.0%".to_string()),
        open: get_fmt(summary_detail, "open"),
        high: get_fmt(summary_detail, "dayHigh"),
        low: get_fmt(summary_detail, "dayLow"),
        year_high: get_fmt(summary_detail, "fiftyTwoWeekHigh"),
        year_low: get_fmt(summary_detail, "fiftyTwoWeekLow"),
        volume: get_raw(summary_detail, "volume"),
        avg_volume: get_raw(summary_detail, "averageVolume"),
        market_cap: get_fmt(summary_detail, "marketCap"),
        beta: get_fmt(summary_detail, "beta"),
        pe: get_fmt(summary_detail, "trailingPE"),
        eps: get_fmt(summary_detail, "trailingEps"),
        dividend: get_fmt(summary_detail, "dividendRate"),
        dividend_yield: get_fmt(summary_detail, "dividendYield"),
        ex_dividend,
        net_assets: get_fmt(summary_detail, "totalAssets"),
        nav: get_fmt(summary_detail, "navPrice"),
        expense_ratio: stats.get("annualReportExpenseRatio").and_then(|r| {
            if let Some(raw) = r.get("raw").and_then(|raw| raw.as_f64()) {
                Some(format!("{:.2}%", raw * 100.0))
            } else {
                get_fmt(r, "raw")
                    .map(|r| format!("{:.2}%", r.parse::<f64>().unwrap_or(0.0) * 100.0))
            }
        }),
        category: stats
            .get("category")
            .and_then(|c| c.as_str())
            .map(|s| s.to_string()),
        last_capital_gain: get_fmt(stats, "lastCapGain"),
        morningstar_rating,
        morningstar_risk_rating,
        holdings_turnover: stats.get("annualHoldingsTurnover").and_then(|t| {
            if let Some(raw) = t.get("raw").and_then(|raw| raw.as_f64()) {
                Some(format!("{:.2}%", raw * 100.0))
            } else {
                get_fmt(t, "raw")
                    .map(|t| format!("{:.2}%", t.parse::<f64>().unwrap_or(0.0) * 100.0))
            }
        }),
        earnings_date,
        last_dividend: get_fmt(stats, "lastDividendValue"),
        inception_date,
        sector: profile
            .get("sector")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()),
        industry: profile
            .get("industry")
            .and_then(|i| i.as_str())
            .map(|s| s.to_string()),
        about: profile
            .get("longBusinessSummary")
            .and_then(|a| a.as_str())
            .map(|s| s.to_string()),
        employees,
        five_days_return: get_fmt(performance_overview, "fiveDaysReturn"),
        one_month_return: get_fmt(performance_overview, "oneMonthReturn"),
        three_month_return: get_fmt(performance_overview, "threeMonthReturn"),
        six_month_return: get_fmt(performance_overview, "sixMonthReturn"),
        ytd_return: get_fmt(performance_overview, "ytdReturnPct"),
        year_return: get_fmt(performance_overview, "oneYearTotalReturn"),
        three_year_return: get_fmt(performance_overview, "threeYearTotalReturn"),
        five_year_return: get_fmt(performance_overview, "fiveYearTotalReturn"),
        ten_year_return: get_fmt(performance_overview, "tenYearTotalReturn"),
        max_return: get_fmt(performance_overview, "maxReturn"),
        logo: logo::get_logo(fetch_client, Some(symbol), website_url).await,
    })
}

#[allow(dead_code)]
fn parse_quotes_from_api(data: Value) -> Result<Vec<Quote>, YahooError> {
    let mut quotes = Vec::new();

    if let Some(quote_response) = data.get("quoteResponse") {
        if let Some(results) = quote_response.get("result").and_then(|r| r.as_array()) {
            info!("Parsing {} quote results", results.len());
            for (idx, result) in results.iter().enumerate() {
                match parse_quote_from_api_result(result) {
                    Ok(quote) => {
                        debug!(
                            "Successfully parsed quote {}: symbol={}, price={}",
                            idx, quote.symbol, quote.price
                        );
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
        warn!(
            "No quotes parsed from API response. Response structure: {}",
            serde_json::to_string(&data).unwrap_or_else(|_| "Failed to serialize".to_string())
        );
    }

    Ok(quotes)
}

#[allow(dead_code)]
fn parse_quote_from_api_result(result: &Value) -> Result<Quote, YahooError> {
    let symbol = result
        .get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string();

    debug!("Parsing quote for symbol: {}", symbol);

    // Check if regularMarketPrice exists and log its structure
    if let Some(price_val) = result.get("regularMarketPrice") {
        debug!("regularMarketPrice for {}: {:?}", symbol, price_val);
        if price_val.is_object() {
            debug!(
                "regularMarketPrice is an object with keys: {:?}",
                price_val.as_object().map(|o| o.keys().collect::<Vec<_>>())
            );
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

    let price = result
        .get("regularMarketPrice")
        .and_then(|p| {
            // Try to get as object first (fmt/raw structure)
            if let Some(obj) = p.as_object() {
                obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .or_else(|| v.as_f64().map(|f| f.to_string()))
                })
            } else if let Some(num) = p.as_f64() {
                // Direct number
                Some(num.to_string())
            } else {
                // Direct string
                p.as_str().map(|str_val| str_val.to_string())
            }
        })
        .unwrap_or_else(|| {
            warn!("Could not extract price for {}, defaulting to 0.0", symbol);
            "0.0".to_string()
        });

    Ok(Quote {
        symbol,
        name: result
            .get("longName")
            .or_else(|| result.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price,
        pre_market_price: result.get("preMarketPrice").and_then(|p| {
            p.get("fmt").or_else(|| p.get("raw")).and_then(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| v.as_f64().map(|f| f.to_string()))
            })
        }),
        after_hours_price: result.get("postMarketPrice").and_then(|p| {
            if let Some(obj) = p.as_object() {
                obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .or_else(|| v.as_f64().map(|f| f.to_string()))
                })
            } else if let Some(num) = p.as_f64() {
                Some(num.to_string())
            } else {
                p.as_str().map(|str_val| str_val.to_string())
            }
        }),
        change: result
            .get("regularMarketChange")
            .and_then(|c| {
                if let Some(obj) = c.as_object() {
                    obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                        v.as_str()
                            .map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| f.to_string()))
                    })
                } else if let Some(num) = c.as_f64() {
                    Some(num.to_string())
                } else {
                    c.as_str().map(|str_val| str_val.to_string())
                }
            })
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: result
            .get("regularMarketChangePercent")
            .and_then(|p| {
                if let Some(obj) = p.as_object() {
                    obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                        v.as_str()
                            .map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| format!("{:.2}%", f)))
                    })
                } else if let Some(num) = p.as_f64() {
                    Some(format!("{:.2}%", num))
                } else {
                    p.as_str().map(|str_val| str_val.to_string())
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
                        debug!(
                            "Successfully parsed simple quote {}: symbol={}, price={}",
                            idx, quote.symbol, quote.price
                        );
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
    let symbol = result
        .get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string();

    debug!("Parsing simple quote for symbol: {}", symbol);

    let price = result
        .get("regularMarketPrice")
        .and_then(|p| {
            if let Some(obj) = p.as_object() {
                obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .or_else(|| v.as_f64().map(|f| f.to_string()))
                })
            } else if let Some(num) = p.as_f64() {
                Some(num.to_string())
            } else {
                p.as_str().map(|str_val| str_val.to_string())
            }
        })
        .unwrap_or_else(|| {
            warn!("Could not extract price for {}, defaulting to 0.0", symbol);
            "0.0".to_string()
        });

    Ok(SimpleQuote {
        symbol,
        name: result
            .get("longName")
            .or_else(|| result.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price,
        pre_market_price: result.get("preMarketPrice").and_then(|p| {
            if let Some(obj) = p.as_object() {
                obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .or_else(|| v.as_f64().map(|f| f.to_string()))
                })
            } else if let Some(num) = p.as_f64() {
                Some(num.to_string())
            } else {
                p.as_str().map(|str_val| str_val.to_string())
            }
        }),
        after_hours_price: result.get("postMarketPrice").and_then(|p| {
            if let Some(obj) = p.as_object() {
                obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .or_else(|| v.as_f64().map(|f| f.to_string()))
                })
            } else if let Some(num) = p.as_f64() {
                Some(num.to_string())
            } else {
                p.as_str().map(|str_val| str_val.to_string())
            }
        }),
        change: result
            .get("regularMarketChange")
            .and_then(|c| {
                if let Some(obj) = c.as_object() {
                    obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                        v.as_str()
                            .map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| f.to_string()))
                    })
                } else if let Some(num) = c.as_f64() {
                    Some(num.to_string())
                } else {
                    c.as_str().map(|str_val| str_val.to_string())
                }
            })
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: result
            .get("regularMarketChangePercent")
            .and_then(|p| {
                if let Some(obj) = p.as_object() {
                    obj.get("fmt").or_else(|| obj.get("raw")).and_then(|v| {
                        v.as_str()
                            .map(|s| s.to_string())
                            .or_else(|| v.as_f64().map(|f| format!("{:.2}%", f)))
                    })
                } else if let Some(num) = p.as_f64() {
                    Some(format!("{:.2}%", num))
                } else {
                    p.as_str().map(|str_val| str_val.to_string())
                }
            })
            .unwrap_or_else(|| "0.0%".to_string()),
        logo: None,
    })
}

fn parse_quote_from_scraped(data: Value) -> Result<Quote, YahooError> {
    Ok(Quote {
        symbol: data
            .get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        name: data
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price: data
            .get("price")
            .and_then(|p| p.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        pre_market_price: None,
        after_hours_price: None,
        change: data
            .get("change")
            .and_then(|c| c.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: data
            .get("percent_change")
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
        symbol: data
            .get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        name: data
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string(),
        price: data
            .get("price")
            .and_then(|p| p.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        pre_market_price: None,
        after_hours_price: None,
        change: data
            .get("change")
            .and_then(|c| c.as_f64().map(|f| f.to_string()))
            .unwrap_or_else(|| "0.0".to_string()),
        percent_change: data
            .get("percent_change")
            .and_then(|p| p.as_f64().map(|f| format!("{:.2}%", f)))
            .unwrap_or_else(|| "0.0%".to_string()),
        logo: None,
    })
}

pub async fn get_similar_quotes(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
    limit: usize,
) -> Result<Vec<SimpleQuote>, YahooError> {
    info!(
        "Fetching similar quotes for symbol: {} (limit: {})",
        symbol, limit
    );

    // Get similar symbols from Yahoo API
    let similar_data = yahoo_client.get_similar_quotes(symbol, limit).await?;

    // Extract symbols from the response
    let symbols: Vec<&str> = similar_data
        .get("finance")
        .and_then(|f| f.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .and_then(|first| first.get("recommendedSymbols"))
        .and_then(|recs| recs.as_array())
        .map(|recs| {
            recs.iter()
                .filter_map(|rec| rec.get("symbol").and_then(|s| s.as_str()))
                .take(limit)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if symbols.is_empty() {
        warn!("No similar symbols found for {}", symbol);
        return Ok(Vec::new());
    }

    info!("Found {} similar symbols: {:?}", symbols.len(), symbols);

    // Fetch simple quotes for the similar symbols
    get_simple_quotes(yahoo_client, fetch_client, &symbols).await
}
