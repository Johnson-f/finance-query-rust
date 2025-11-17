use crate::client::{error::YahooError, FetchClient, YahooFinanceClient};
use crate::models::sectors::{MarketSector, MarketSectorDetails, Sector};
use scraper::{Html, Selector};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info, warn};

// URL mapping for sectors
fn get_sector_url(sector: Sector) -> String {
    format!("https://finance.yahoo.com/sectors/{}/", sector.url_path())
}

/// Get sector data for all sectors
pub async fn get_sectors(
    fetch_client: &Arc<FetchClient>,
) -> Result<Vec<MarketSector>, YahooError> {
    info!("Fetching sector data for all sectors");
    
    let sectors = Sector::all();
    let mut tasks = Vec::new();
    
    // Create fetch tasks for all sectors
    for sector in sectors {
        let url = get_sector_url(sector);
        let fetch_client_clone = Arc::clone(fetch_client);
        let sector_name = sector.as_str().to_string();
        tasks.push(tokio::spawn(async move {
            let html = fetch_client_clone.fetch(&url).await?;
            parse_sector(&html, &sector_name).await
        }));
    }
    
    // Wait for all tasks to complete
    let mut results = Vec::new();
    for task in tasks {
        match task.await {
            Ok(Ok(sector)) => results.push(sector),
            Ok(Err(e)) => {
                warn!("Failed to fetch sector: {}", e);
                // Continue with other sectors even if one fails
            }
            Err(e) => {
                error!("Task join error: {}", e);
            }
        }
    }
    
    info!("Successfully fetched {} sectors", results.len());
    Ok(results)
}

/// Get sector data for a specific symbol
pub async fn get_sector_for_symbol(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<MarketSector, YahooError> {
    info!("Fetching sector for symbol: {}", symbol);
    
    // Get sector name from quote summary
    let sector_name = get_yahoo_sector(yahoo_client, symbol).await?;
    
    if sector_name.is_none() {
        return Err(YahooError::NotFound(format!("Sector for {} not found", symbol)));
    }
    
    let sector_name = sector_name.unwrap();
    
    // Find the Sector enum from the name
    let sector = Sector::from_str(&sector_name)
        .map_err(|_| YahooError::ParseError(format!("Invalid sector name: {}", sector_name)))?;
    
    // Fetch and parse the sector page
    let url = get_sector_url(sector);
    let html = fetch_client.fetch(&url).await?;
    parse_sector(&html, &sector_name).await
}

/// Get detailed sector data for a specific sector
pub async fn get_sector_details(
    fetch_client: &Arc<FetchClient>,
    sector: Sector,
) -> Result<MarketSectorDetails, YahooError> {
    info!("Fetching detailed sector data for: {}", sector.as_str());
    
    let url = get_sector_url(sector);
    let html = fetch_client.fetch(&url).await?;
    parse_sector_details(&html, sector.as_str()).await
}

/// Get sector name for a symbol from Yahoo Finance
async fn get_yahoo_sector(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
) -> Result<Option<String>, YahooError> {
    let response = yahoo_client
        .get_quote_summary(&symbol.to_uppercase(), &["assetProfile"])
        .await?;
    
    let result = response
        .get("quoteSummary")
        .and_then(|qs| qs.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.get(0));
    
    if let Some(data) = result {
        if let Some(profile) = data.get("assetProfile") {
            if let Some(sector) = profile.get("sector").and_then(|s| s.as_str()) {
                return Ok(Some(sector.to_string()));
            }
        }
    }
    
    Ok(None)
}

/// Parse sector data from HTML
async fn parse_sector(html: &str, sector: &str) -> Result<MarketSector, YahooError> {
    let document = Html::parse_document(html);
    
    // The Python version uses XPath: /html/body/div[2]/main/section/section/section/section/section[1]/section[2]/div
    // Then finds sections within that div, and extracts div[contains(@class, "perf")]/text()
    
    // Try to navigate to main > section > section > section > section > section[1] > section[2] > div
    // We'll use a more flexible approach: find main, then navigate through sections
    let main_selector = Selector::parse("main")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse main selector: {}", e)))?;
    
    let main = document.select(&main_selector).next()
        .ok_or_else(|| YahooError::ParseError("Could not find main element".to_string()))?;
    
    // Navigate through sections: main > section > section > section > section > section[1] > section[2]
    // We'll try to find sections that contain divs with class "perf"
    let section_selector = Selector::parse("section")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse section selector: {}", e)))?;
    
    let perf_selector = Selector::parse("div[class*='perf']")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse perf selector: {}", e)))?;
    
    let mut performance_data = Vec::new();
    
    // Strategy 1: Find sections that contain performance divs
    // Look for sections that have exactly one perf div (each card has one perf value)
    for section in main.select(&section_selector) {
        let perf_divs: Vec<_> = section.select(&perf_selector).collect();
        
        // Each card section should have one perf div
        if perf_divs.len() == 1 {
            if let Some(perf_div) = perf_divs.first() {
                let text = perf_div.text().collect::<String>().trim().to_string();
                
                if !text.is_empty() {
                    // Check for positive/negative class in the div or its parent
                    let mut sign = "";
                    
                    // Check the div itself
                    let div_classes = perf_div.value().attr("class").unwrap_or("");
                    if div_classes.contains("positive") {
                        sign = "+";
                    } else if div_classes.contains("negative") {
                        sign = "-";
                    }
                    
                    // Check parent elements
                    if sign.is_empty() {
                        let mut current = perf_div.parent();
                        for _ in 0..5 { // Check up to 5 levels up
                            if let Some(parent_node) = current {
                                if let Some(parent_elem) = parent_node.value().as_element() {
                                    let classes = parent_elem.attr("class").unwrap_or("");
                                    if classes.contains("positive") {
                                        sign = "+";
                                        break;
                                    } else if classes.contains("negative") {
                                        sign = "-";
                                        break;
                                    }
                                    current = parent_node.parent();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    
                    let perf_text = if sign.is_empty() {
                        text
                    } else {
                        format!("{}{}", sign, text)
                    };
                    
                    performance_data.push(perf_text);
                }
            }
        }
    }
    
    // Strategy 2: If we didn't find enough, try a different approach
    // Look for all perf divs and group them by their parent section
    if performance_data.len() < 5 {
        performance_data.clear();
        
        // Find all sections, then look for perf divs within each
        let all_sections: Vec<_> = main.select(&section_selector).collect();
        
        for section in all_sections {
            let perf_divs: Vec<_> = section.select(&perf_selector).collect();
            if !perf_divs.is_empty() {
                // Take the first perf div from this section
                if let Some(perf_div) = perf_divs.first() {
                    let text = perf_div.text().collect::<String>().trim().to_string();
                    
                    if !text.is_empty() {
                        let mut sign = "";
                        let div_classes = perf_div.value().attr("class").unwrap_or("");
                        if div_classes.contains("positive") {
                            sign = "+";
                        } else if div_classes.contains("negative") {
                            sign = "-";
                        }
                        
                        // Check parent
                        if sign.is_empty() {
                            let mut current = perf_div.parent();
                            for _ in 0..5 {
                                if let Some(parent_node) = current {
                                    if let Some(parent_elem) = parent_node.value().as_element() {
                                        let classes = parent_elem.attr("class").unwrap_or("");
                                        if classes.contains("positive") {
                                            sign = "+";
                                            break;
                                        } else if classes.contains("negative") {
                                            sign = "-";
                                            break;
                                        }
                                        current = parent_node.parent();
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                        
                        let perf_text = if sign.is_empty() {
                            text
                        } else {
                            format!("{}{}", sign, text)
                        };
                        
                        performance_data.push(perf_text);
                    }
                }
            }
        }
    }
    
    // Strategy 3: Last resort - find all perf divs in the document
    if performance_data.len() < 5 {
        performance_data.clear();
        
        for perf_div in document.select(&perf_selector).take(10) {
            let text = perf_div.text().collect::<String>().trim().to_string();
            
            if !text.is_empty() && !text.contains("Sector") {
                let mut sign = "";
                let div_classes = perf_div.value().attr("class").unwrap_or("");
                if div_classes.contains("positive") {
                    sign = "+";
                } else if div_classes.contains("negative") {
                    sign = "-";
                }
                
                let perf_text = if sign.is_empty() {
                    text
                } else {
                    format!("{}{}", sign, text)
                };
                
                performance_data.push(perf_text);
            }
        }
    }
    
    if performance_data.len() < 5 {
        // Debug: log how many perf divs we found
        let total_perf_divs = document.select(&perf_selector).count();
        let main_sections = main.select(&section_selector).count();
        
        warn!(
            "Failed to parse sector data for {}: expected 5 performance values, got {}. Found {} perf divs total, {} sections in main. HTML length: {}",
            sector, performance_data.len(), total_perf_divs, main_sections, html.len()
        );
        
        return Err(YahooError::ParseError(format!(
            "Failed to parse sector data: expected 5 performance values, got {}. Found {} perf divs in document",
            performance_data.len(),
            total_perf_divs
        )));
    }
    
    Ok(MarketSector {
        sector: sector.to_string(),
        day_return: performance_data[0].clone(),
        ytd_return: performance_data[1].clone(),
        year_return: performance_data[2].clone(),
        three_year_return: performance_data[3].clone(),
        five_year_return: performance_data[4].clone(),
    })
}

/// Parse detailed sector data from HTML
async fn parse_sector_details(html: &str, sector_name: &str) -> Result<MarketSectorDetails, YahooError> {
    let document = Html::parse_document(html);
    
    // Parse returns (same as parse_sector)
    let mut returns = Vec::new();
    let perf_selector = Selector::parse("div.perf, .perf")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse perf selector: {}", e)))?;
    let section_selector = Selector::parse("section")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse section selector: {}", e)))?;
    
    for section in document.select(&section_selector) {
        let perf_divs: Vec<_> = section.select(&perf_selector).collect();
        if perf_divs.len() >= 5 {
            for perf_div in perf_divs.iter().take(5) {
                let text = perf_div.text().collect::<String>().trim().to_string();
                let mut sign = "";
                let div_classes = perf_div.value().attr("class").unwrap_or("");
                if div_classes.contains("positive") {
                    sign = "+";
                } else if div_classes.contains("negative") {
                    sign = "-";
                }
                
                // Check parent
                if sign.is_empty() {
                    if let Some(parent_node) = perf_div.parent() {
                        if let Some(parent_elem) = parent_node.value().as_element() {
                            let classes = parent_elem.attr("class").unwrap_or("");
                            if classes.contains("positive") {
                                sign = "+";
                            } else if classes.contains("negative") {
                                sign = "-";
                            }
                        }
                    }
                }
                let perf_text = if sign.is_empty() {
                    text
                } else {
                    format!("{}{}", sign, text)
                };
                returns.push(perf_text);
            }
            break;
        }
    }
    
    if returns.len() < 5 {
        return Err(YahooError::ParseError("Failed to parse returns data".to_string()));
    }
    
    // Parse market info (market cap, market weight, industries, companies)
    // The Python version uses: /html/body/div[2]/main/section/section/section/section/section[1]/div/section/div[2]/div[2]
    let info_selector = Selector::parse("div[class*='info'], section[class*='info']")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse info selector: {}", e)))?;
    
    let mut market_cap = String::new();
    let mut market_weight = String::new();
    let mut industries = 0;
    let mut companies = 0;
    
    // Try to find divs with data values
    let div_selector = Selector::parse("div")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse div selector: {}", e)))?;
    
    // Look for patterns in the HTML structure
    // This is a simplified approach - the actual structure may vary
    for div in document.select(&div_selector) {
        let text = div.text().collect::<String>();
        // Try to identify market cap, weight, etc. by context
        // This is a heuristic approach
    }
    
    // For now, use placeholder values - the actual parsing would need more specific selectors
    // based on the actual HTML structure
    warn!("Sector details parsing is simplified - some fields may be missing");
    
    // Parse top industries
    let mut top_industries = Vec::new();
    let table_row_selector = Selector::parse("table tbody tr")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse table row selector: {}", e)))?;
    
    for row in document.select(&table_row_selector).take(20) {
        let cells: Vec<_> = row.select(&Selector::parse("td").unwrap()).collect();
        if cells.len() >= 2 {
            let industry = cells[0].text().collect::<String>().trim().to_string();
            let weight = cells[1].text().collect::<String>().trim().to_string();
            if !industry.is_empty() && !weight.is_empty() {
                top_industries.push(format!("{}: {}", industry, weight));
            }
        }
    }
    
    // Parse top companies
    let mut top_companies = Vec::new();
    let link_selector = Selector::parse("a[href*='/quote/']")
        .map_err(|e| YahooError::ParseError(format!("Failed to parse link selector: {}", e)))?;
    
    for link in document.select(&link_selector).take(20) {
        if let Some(href) = link.value().attr("href") {
            // Extract symbol from URL like /quote/AAPL
            if let Some(symbol_part) = href.split('/').nth(2) {
                if !symbol_part.is_empty() && symbol_part.len() <= 10 {
                    top_companies.push(symbol_part.to_string());
                }
            }
        }
        // Also try to get text content
        let text = link.text().collect::<String>().trim().to_string();
        if text.len() <= 10 && text.chars().all(|c| c.is_uppercase() || c.is_ascii_digit()) {
            if !top_companies.contains(&text) {
                top_companies.push(text);
            }
        }
    }
    
    Ok(MarketSectorDetails {
        sector: sector_name.to_string(),
        day_return: returns[0].clone(),
        ytd_return: returns[1].clone(),
        year_return: returns[2].clone(),
        three_year_return: returns[3].clone(),
        five_year_return: returns[4].clone(),
        market_cap,
        market_weight,
        industries,
        companies,
        top_industries,
        top_companies,
    })
}