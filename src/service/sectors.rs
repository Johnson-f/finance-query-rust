use crate::client::{error::YahooError, FetchClient, YahooFinanceClient};
use crate::models::sectors::{MarketSector, MarketSectorDetails, Sector};
use serde_json;
use std::sync::Arc;
use tracing::info;

const API_BASE_URL: &str = "https://finance-query.onrender.com/v1";

/// Get sector data for all sectors
pub async fn get_sectors(
    fetch_client: &Arc<FetchClient>,
) -> Result<Vec<MarketSector>, YahooError> {
    info!("Fetching sector data for all sectors from API");
    
    let url = format!("{}/sectors", API_BASE_URL);
    let response_text = fetch_client.fetch(&url).await?;
    
    let sectors: Vec<MarketSector> = serde_json::from_str(&response_text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse sectors JSON: {}", e)))?;
    
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
    let response_text = fetch_client.fetch(&url).await?;
    
    let sector: MarketSector = serde_json::from_str(&response_text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse sector JSON for {}: {}", symbol, e)))?;
    
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
    
    let response_text = fetch_client.fetch(&url).await?;
    
    let details: MarketSectorDetails = serde_json::from_str(&response_text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse sector details JSON for {}: {}", sector_name, e)))?;
    
    Ok(details)
}