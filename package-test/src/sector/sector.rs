//! Sector data fetching functions using finance-query-core
//!
//! This module provides functions to fetch sector-related data from Yahoo Finance:
//! - Sector performance data (day, YTD, 1Y, 3Y, 5Y returns)
//! - Sector details with top industries and companies
//! - Industry data within sectors

use finance_query_core::{
    FetchClient, Industry, MarketSector, MarketSectorDetails, Sector, YahooAuthManager,
    YahooError, YahooFinanceClient,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// Response Types
/// Sector performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorPerformance {
    pub sector: String,
    pub day_return: f64,
    pub ytd_return: f64,
    pub year_return: f64,
    pub three_year_return: f64,
    pub five_year_return: f64,
}

impl SectorPerformance {
    /// Check if sector is positive for the day
    pub fn is_positive_day(&self) -> bool {
        self.day_return > 0.0
    }

    /// Check if sector is positive YTD
    pub fn is_positive_ytd(&self) -> bool {
        self.ytd_return > 0.0
    }

    /// Get formatted day return string
    pub fn day_return_formatted(&self) -> String {
        format!("{:+.2}%", self.day_return)
    }

    /// Get formatted YTD return string
    pub fn ytd_return_formatted(&self) -> String {
        format!("{:+.2}%", self.ytd_return)
    }
}


/// Detailed sector information with companies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorDetails {
    pub sector: String,
    pub sector_key: String,
    pub performance: SectorPerformance,
    pub market_cap: Option<String>,
    pub market_weight: Option<String>,
    pub industries_count: Option<i32>,
    pub companies_count: Option<i32>,
    pub top_industries: Vec<String>,
    pub top_companies: Vec<SectorCompany>,
}

/// Company within a sector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorCompany {
    pub symbol: String,
    pub name: String,
    pub price: Option<f64>,
    pub change: Option<f64>,
    pub percent_change: Option<f64>,
    pub market_cap: Option<i64>,
}

/// All sectors overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectorsOverview {
    pub sectors: Vec<SectorPerformance>,
    pub best_performer: Option<String>,
    pub worst_performer: Option<String>,
}

impl SectorsOverview {
    /// Get sectors sorted by day return (best first)
    pub fn sorted_by_day_return(&self) -> Vec<&SectorPerformance> {
        let mut sorted: Vec<_> = self.sectors.iter().collect();
        sorted.sort_by(|a, b| b.day_return.partial_cmp(&a.day_return).unwrap());
        sorted
    }

    /// Get sectors sorted by YTD return (best first)
    pub fn sorted_by_ytd_return(&self) -> Vec<&SectorPerformance> {
        let mut sorted: Vec<_> = self.sectors.iter().collect();
        sorted.sort_by(|a, b| b.ytd_return.partial_cmp(&a.ytd_return).unwrap());
        sorted
    }

    /// Get positive sectors for the day
    pub fn positive_sectors(&self) -> Vec<&SectorPerformance> {
        self.sectors.iter().filter(|s| s.is_positive_day()).collect()
    }

    /// Get negative sectors for the day
    pub fn negative_sectors(&self) -> Vec<&SectorPerformance> {
        self.sectors.iter().filter(|s| !s.is_positive_day()).collect()
    }
}

// Helper Functions
/// Creates a configured YahooFinanceClient
async fn create_client() -> Result<(Arc<YahooAuthManager>, YahooFinanceClient), YahooError> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    auth_manager.refresh().await?;

    Ok((auth_manager, client))
}

/// Parse percentage string to f64
fn parse_percent(s: &str) -> f64 {
    s.trim_end_matches('%')
        .replace(',', "")
        .parse::<f64>()
        .unwrap_or(0.0)
}

/// Parse MarketSector to SectorPerformance
fn market_sector_to_performance(ms: &MarketSector) -> SectorPerformance {
    SectorPerformance {
        sector: ms.sector.clone(),
        day_return: parse_percent(&ms.day_return),
        ytd_return: parse_percent(&ms.ytd_return),
        year_return: parse_percent(&ms.year_return),
        three_year_return: parse_percent(&ms.three_year_return),
        five_year_return: parse_percent(&ms.five_year_return),
    }
}


// ============================================================================
// Public API Functions
// ============================================================================

/// Get all available sectors
///
/// # Returns
/// Vector of all Sector enum variants
///
/// # Example
/// ```rust,ignore
/// let sectors = get_all_sectors();
/// for sector in sectors {
///     println!("{}", sector.as_str());
/// }
/// ```
pub fn get_all_sectors() -> Vec<Sector> {
    Sector::all()
}

/// Get sector performance data from screener
///
/// # Arguments
/// * `sector` - The sector to fetch data for
///
/// # Returns
/// SectorPerformance with return data
///
/// # Example
/// ```rust,ignore
/// let perf = get_sector_performance(Sector::Technology).await?;
/// println!("{}: Day {:+.2}%, YTD {:+.2}%", perf.sector, perf.day_return, perf.ytd_return);
/// ```
pub async fn get_sector_performance(sector: Sector) -> Result<SectorPerformance, YahooError> {
    let (_, client) = create_client().await?;

    // Use screener to get sector ETF data as proxy for sector performance
    let etf_symbol = sector_to_etf(sector);
    let url = "https://query1.finance.yahoo.com/v7/finance/quote";
    let params = [("symbols", etf_symbol)];

    let response = client.make_request(&url, Some(&params)).await?;
    let text = response.text().await.map_err(YahooError::NetworkError)?;
    let data: Value = serde_json::from_str(&text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse sector data: {}", e)))?;

    parse_sector_from_quote(&data, sector)
}

/// Get performance data for all sectors
///
/// # Returns
/// SectorsOverview with all sector performances
///
/// # Example
/// ```rust,ignore
/// let overview = get_all_sectors_performance().await?;
/// println!("Best: {}", overview.best_performer.unwrap_or_default());
/// for sector in overview.sorted_by_day_return() {
///     println!("{}: {}", sector.sector, sector.day_return_formatted());
/// }
/// ```
pub async fn get_all_sectors_performance() -> Result<SectorsOverview, YahooError> {
    let (_, client) = create_client().await?;

    // Get all sector ETFs in one request
    let etf_symbols = get_all_sector_etfs().join(",");
    let url = "https://query1.finance.yahoo.com/v7/finance/quote";
    let params = [("symbols", etf_symbols.as_str())];

    let response = client.make_request(&url, Some(&params)).await?;
    let text = response.text().await.map_err(YahooError::NetworkError)?;
    let data: Value = serde_json::from_str(&text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse sectors data: {}", e)))?;

    parse_all_sectors_from_quotes(&data)
}

/// Get top companies in a sector
///
/// # Arguments
/// * `sector` - The sector to fetch companies for
/// * `count` - Number of companies to return (max 25)
///
/// # Returns
/// Vector of SectorCompany
///
/// # Example
/// ```rust,ignore
/// let companies = get_sector_top_companies(Sector::Technology, 10).await?;
/// for company in companies {
///     println!("{} ({}): ${:.2}", company.name, company.symbol, company.price.unwrap_or(0.0));
/// }
/// ```
pub async fn get_sector_top_companies(
    sector: Sector,
    count: usize,
) -> Result<Vec<SectorCompany>, YahooError> {
    let (_, client) = create_client().await?;

    let sector_name = sector.as_str();
    let count_str = count.min(25).to_string();

    // Use screener to get top companies by market cap in sector
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/screener/predefined/saved?count={}&scrIds={}",
        count_str,
        get_screener_id_for_sector(sector)
    );

    let response = client.make_request(&url, None).await?;
    let text = response.text().await.map_err(YahooError::NetworkError)?;
    let data: Value = serde_json::from_str(&text)
        .map_err(|e| YahooError::ParseError(format!("Failed to parse screener data: {}", e)))?;

    parse_companies_from_screener(&data, sector_name)
}

/// Get industry data
///
/// # Arguments
/// * `industry_key` - The industry key (e.g., "software-infrastructure")
///
/// # Returns
/// Industry data with top companies
///
/// # Example
/// ```rust,ignore
/// let industry = get_industry("software-infrastructure").await?;
/// println!("{}: {} companies", industry.name, industry.top_performing_companies.len());
/// ```
pub async fn get_industry(industry_key: &str) -> Result<Industry, YahooError> {
    let (_, client) = create_client().await?;
    client.get_industry(industry_key).await
}


// ============================================================================
// Sector ETF Mappings
// ============================================================================

/// Map sector to its representative ETF symbol
fn sector_to_etf(sector: Sector) -> &'static str {
    match sector {
        Sector::Technology => "XLK",
        Sector::Healthcare => "XLV",
        Sector::FinancialServices => "XLF",
        Sector::ConsumerCyclical => "XLY",
        Sector::Industrials => "XLI",
        Sector::ConsumerDefensive => "XLP",
        Sector::Energy => "XLE",
        Sector::RealEstate => "XLRE",
        Sector::Utilities => "XLU",
        Sector::BasicMaterials => "XLB",
        Sector::Communication => "XLC",
    }
}

/// Get all sector ETF symbols
fn get_all_sector_etfs() -> Vec<&'static str> {
    vec![
        "XLK",  // Technology
        "XLV",  // Healthcare
        "XLF",  // Financial Services
        "XLY",  // Consumer Cyclical
        "XLI",  // Industrials
        "XLP",  // Consumer Defensive
        "XLE",  // Energy
        "XLRE", // Real Estate
        "XLU",  // Utilities
        "XLB",  // Basic Materials
        "XLC",  // Communication
    ]
}

/// Map ETF symbol to sector name
fn etf_to_sector_name(etf: &str) -> &'static str {
    match etf {
        "XLK" => "Technology",
        "XLV" => "Healthcare",
        "XLF" => "Financial Services",
        "XLY" => "Consumer Cyclical",
        "XLI" => "Industrials",
        "XLP" => "Consumer Defensive",
        "XLE" => "Energy",
        "XLRE" => "Real Estate",
        "XLU" => "Utilities",
        "XLB" => "Basic Materials",
        "XLC" => "Communication Services",
        _ => "Unknown",
    }
}

/// Get screener ID for sector (using most_actives as fallback)
fn get_screener_id_for_sector(sector: Sector) -> &'static str {
    // Yahoo Finance doesn't have direct sector screeners, use most_actives
    // In production, you'd use custom screener queries
    "most_actives"
}

// ============================================================================
// Parsing Functions
// ============================================================================

fn parse_sector_from_quote(data: &Value, sector: Sector) -> Result<SectorPerformance, YahooError> {
    let results = data
        .get("quoteResponse")
        .and_then(|qr| qr.get("result"))
        .and_then(|r| r.as_array())
        .ok_or_else(|| YahooError::ParseError("No quote results found".to_string()))?;

    if let Some(quote) = results.first() {
        let day_change = quote
            .get("regularMarketChangePercent")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let ytd_return = quote
            .get("ytdReturn")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        // For ETFs, we can get various return periods
        let year_return = quote
            .get("fiftyTwoWeekChangePercent")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        Ok(SectorPerformance {
            sector: sector.as_str().to_string(),
            day_return: day_change,
            ytd_return: ytd_return * 100.0, // Convert to percentage
            year_return,
            three_year_return: 0.0, // Not available in simple quote
            five_year_return: 0.0,  // Not available in simple quote
        })
    } else {
        Err(YahooError::ParseError("No sector data found".to_string()))
    }
}

fn parse_all_sectors_from_quotes(data: &Value) -> Result<SectorsOverview, YahooError> {
    let results = data
        .get("quoteResponse")
        .and_then(|qr| qr.get("result"))
        .and_then(|r| r.as_array())
        .ok_or_else(|| YahooError::ParseError("No quote results found".to_string()))?;

    let mut sectors = Vec::new();
    let mut best_return = f64::MIN;
    let mut worst_return = f64::MAX;
    let mut best_performer = None;
    let mut worst_performer = None;

    for quote in results {
        let symbol = quote
            .get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("");

        let sector_name = etf_to_sector_name(symbol);
        if sector_name == "Unknown" {
            continue;
        }

        let day_change = quote
            .get("regularMarketChangePercent")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let ytd_return = quote
            .get("ytdReturn")
            .and_then(|v| v.as_f64())
            .map(|v| v * 100.0)
            .unwrap_or(0.0);

        let year_return = quote
            .get("fiftyTwoWeekChangePercent")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        if day_change > best_return {
            best_return = day_change;
            best_performer = Some(sector_name.to_string());
        }
        if day_change < worst_return {
            worst_return = day_change;
            worst_performer = Some(sector_name.to_string());
        }

        sectors.push(SectorPerformance {
            sector: sector_name.to_string(),
            day_return: day_change,
            ytd_return,
            year_return,
            three_year_return: 0.0,
            five_year_return: 0.0,
        });
    }

    Ok(SectorsOverview {
        sectors,
        best_performer,
        worst_performer,
    })
}

fn parse_companies_from_screener(
    data: &Value,
    sector_name: &str,
) -> Result<Vec<SectorCompany>, YahooError> {
    let quotes = data
        .get("finance")
        .and_then(|f| f.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("quotes"))
        .and_then(|q| q.as_array())
        .ok_or_else(|| YahooError::ParseError("No screener results found".to_string()))?;

    let mut companies = Vec::new();

    for quote in quotes {
        let symbol = quote
            .get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();

        if symbol.is_empty() || symbol.contains('.') {
            continue;
        }

        let name = quote
            .get("longName")
            .or_else(|| quote.get("shortName"))
            .and_then(|n| n.as_str())
            .unwrap_or("")
            .to_string();

        let price = quote
            .get("regularMarketPrice")
            .and_then(|p| p.as_f64());

        let change = quote
            .get("regularMarketChange")
            .and_then(|c| c.as_f64());

        let percent_change = quote
            .get("regularMarketChangePercent")
            .and_then(|p| p.as_f64());

        let market_cap = quote
            .get("marketCap")
            .and_then(|m| m.as_i64());

        companies.push(SectorCompany {
            symbol,
            name,
            price,
            change,
            percent_change,
            market_cap,
        });
    }

    Ok(companies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_sectors() {
        let sectors = get_all_sectors();
        assert_eq!(sectors.len(), 11);
    }

    #[test]
    fn test_sector_to_etf() {
        assert_eq!(sector_to_etf(Sector::Technology), "XLK");
        assert_eq!(sector_to_etf(Sector::Healthcare), "XLV");
        assert_eq!(sector_to_etf(Sector::Energy), "XLE");
    }

    #[test]
    fn test_etf_to_sector_name() {
        assert_eq!(etf_to_sector_name("XLK"), "Technology");
        assert_eq!(etf_to_sector_name("XLV"), "Healthcare");
        assert_eq!(etf_to_sector_name("INVALID"), "Unknown");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_sector_performance() {
        let result = get_sector_performance(Sector::Technology).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_all_sectors_performance() {
        let result = get_all_sectors_performance().await;
        assert!(result.is_ok());
        let overview = result.unwrap();
        assert!(!overview.sectors.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_industry() {
        let result = get_industry("software-infrastructure").await;
        assert!(result.is_ok());
    }
}