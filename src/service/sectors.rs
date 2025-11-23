use crate::client::{error::YahooError, FetchClient, YahooFinanceClient};
use crate::models::sectors::{MarketSector, MarketSectorDetails, Sector};
use serde_json;
use std::sync::Arc;
use tracing::{info, debug, warn};

const API_BASE_URL: &str = "https://finance-query.onrender.com/v1";

/// Get sector data for all sectors
pub async fn get_sectors(
    fetch_client: &Arc<FetchClient>,
) -> Result<Vec<MarketSector>, YahooError> {
    info!("Fetching sector data for all sectors from API");
    
    let url = format!("{}/sectors", API_BASE_URL);
    let response_text = fetch_client.fetch_json(&url).await?;
    
    // Validate response is not empty or whitespace
    let trimmed = response_text.trim();
    if trimmed.is_empty() {
        warn!("Empty response received from {}", url);
        return Err(YahooError::ParseError(format!(
            "Empty response received from {} (response length: {} bytes)",
            url, response_text.len()
        )));
    }
    
    debug!("Response from {} (length: {} bytes, preview: {})", 
           url, 
           response_text.len(),
           trimmed.chars().take(200).collect::<String>());
    
    let sectors: Vec<MarketSector> = serde_json::from_str(trimmed)
        .map_err(|e| {
            warn!("Failed to parse JSON from {}: {}. Response preview: {}", 
                  url, e, trimmed.chars().take(200).collect::<String>());
            YahooError::ParseError(format!(
                "Failed to parse sectors JSON from {} (response length: {} bytes): {}", 
                url, response_text.len(), e
            ))
        })?;
    
    info!("Successfully fetched {} sectors", sectors.len());
    Ok(sectors)
}

/// Get sector data for a specific symbol
pub async fn get_sector_for_symbol(
    _yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<MarketSector, YahooError> {
    info!("Fetching sector for symbol: {} from API", symbol);
    
    let url = format!("{}/sectors/symbol/{}", API_BASE_URL, symbol);
    let response_text = fetch_client.fetch_json(&url).await?;
    
    // Validate response is not empty or whitespace
    let trimmed = response_text.trim();
    if trimmed.is_empty() {
        warn!("Empty response received from {} for symbol {}", url, symbol);
        return Err(YahooError::ParseError(format!(
            "Empty response received from {} for symbol {} (response length: {} bytes)",
            url, symbol, response_text.len()
        )));
    }
    
    debug!("Response from {} (length: {} bytes, preview: {})", 
           url, 
           response_text.len(),
           trimmed.chars().take(200).collect::<String>());
    
    let sector: MarketSector = serde_json::from_str(trimmed)
        .map_err(|e| {
            warn!("Failed to parse JSON from {} for symbol {}: {}. Response preview: {}", 
                  url, symbol, e, trimmed.chars().take(200).collect::<String>());
            YahooError::ParseError(format!(
                "Failed to parse sector JSON from {} for symbol {} (response length: {} bytes): {}", 
                url, symbol, response_text.len(), e
            ))
        })?;
    
    Ok(sector)
}

/// Get detailed sector data for a specific sector
pub async fn get_sector_details(
    fetch_client: &Arc<FetchClient>,
    sector: Sector,
) -> Result<MarketSectorDetails, YahooError> {
    info!("Fetching detailed sector data for: {} from API", sector.as_str());
    
    // URL-encode the sector name for the path (e.g., "Financial Services" -> "Financial%20Services")
    let sector_name = sector.as_str();
    // Simple URL encoding: replace spaces with %20
    let encoded_sector = sector_name.replace(' ', "%20");
    let url = format!("{}/sectors/details/{}", API_BASE_URL, encoded_sector);
    
    let response_text = fetch_client.fetch_json(&url).await?;
    
    // Validate response is not empty or whitespace
    let trimmed = response_text.trim();
    if trimmed.is_empty() {
        warn!("Empty response received from {} for sector {}", url, sector_name);
        return Err(YahooError::ParseError(format!(
            "Empty response received from {} for sector {} (response length: {} bytes)",
            url, sector_name, response_text.len()
        )));
    }
    
    debug!("Response from {} (length: {} bytes, preview: {})", 
           url, 
           response_text.len(),
           trimmed.chars().take(200).collect::<String>());
    
    let details: MarketSectorDetails = serde_json::from_str(trimmed)
        .map_err(|e| {
            warn!("Failed to parse JSON from {} for sector {}: {}. Response preview: {}", 
                  url, sector_name, e, trimmed.chars().take(200).collect::<String>());
            YahooError::ParseError(format!(
                "Failed to parse sector details JSON from {} for sector {} (response length: {} bytes): {}", 
                url, sector_name, response_text.len(), e
            ))
        })?;
    
    Ok(details)
}