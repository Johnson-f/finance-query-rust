//! Holder data fetching functions using finance-query-core
//!
//! This module provides functions to fetch ownership and holder data from Yahoo Finance:
//! - Major holders breakdown (institutional vs insider ownership percentages)
//! - Institutional holders (top institutional investors)
//! - Mutual fund holders (top mutual fund investors)
//! - Insider transactions (recent insider buys/sells)
//! - Insider purchases summary (aggregated insider activity)
//! - Insider roster (list of company insiders)

use finance_query_core::{FetchClient, YahooAuthManager, YahooError, YahooFinanceClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// ============================================================================
// Holder Type Enum
// ============================================================================

/// Type of holder data to fetch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolderType {
    /// Major holders breakdown (ownership percentages)
    Major,
    /// Top institutional holders
    Institutional,
    /// Top mutual fund holders
    MutualFund,
    /// Recent insider transactions
    InsiderTransactions,
    /// Insider purchases summary
    InsiderPurchases,
    /// Company insider roster
    InsiderRoster,
}

impl HolderType {
    pub fn as_module(&self) -> &'static str {
        match self {
            HolderType::Major => "majorHoldersBreakdown",
            HolderType::Institutional => "institutionOwnership",
            HolderType::MutualFund => "fundOwnership",
            HolderType::InsiderTransactions => "insiderTransactions",
            HolderType::InsiderPurchases => "netSharePurchaseActivity",
            HolderType::InsiderRoster => "insiderHolders",
        }
    }
}

// ============================================================================
// Response Types - Major Holders
// ============================================================================

/// Major holders breakdown showing ownership percentages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorHoldersBreakdown {
    pub insiders_percent_held: Option<f64>,
    pub institutions_percent_held: Option<f64>,
    pub institutions_float_percent_held: Option<f64>,
    pub institutions_count: Option<i64>,
}

impl MajorHoldersBreakdown {
    /// Get the percentage held by retail investors (100% - insiders - institutions)
    pub fn retail_percent(&self) -> Option<f64> {
        match (self.insiders_percent_held, self.institutions_percent_held) {
            (Some(insiders), Some(institutions)) => Some(100.0 - (insiders * 100.0) - (institutions * 100.0)),
            _ => None,
        }
    }
}

// ============================================================================
// Response Types - Institutional Holders
// ============================================================================

/// An institutional holder (hedge fund, pension fund, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalHolder {
    pub organization: String,
    pub shares: i64,
    pub date_reported: Option<String>,
    pub percent_held: Option<f64>,
    pub value: Option<i64>,
}

// ============================================================================
// Response Types - Mutual Fund Holders
// ============================================================================

/// A mutual fund holder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundHolder {
    pub organization: String,
    pub shares: i64,
    pub date_reported: Option<String>,
    pub percent_held: Option<f64>,
    pub value: Option<i64>,
}

// ============================================================================
// Response Types - Insider Transactions
// ============================================================================

/// An insider transaction (buy/sell by company insider)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderTransaction {
    pub filer_name: String,
    pub filer_relation: Option<String>,
    pub transaction_text: Option<String>,
    pub start_date: Option<String>,
    pub shares: Option<i64>,
    pub value: Option<i64>,
    pub ownership: Option<String>,
}

impl InsiderTransaction {
    /// Check if this is a buy transaction
    pub fn is_buy(&self) -> bool {
        self.transaction_text
            .as_ref()
            .map(|t| t.to_lowercase().contains("buy") || t.to_lowercase().contains("purchase"))
            .unwrap_or(false)
    }

    /// Check if this is a sell transaction
    pub fn is_sell(&self) -> bool {
        self.transaction_text
            .as_ref()
            .map(|t| t.to_lowercase().contains("sell") || t.to_lowercase().contains("sale"))
            .unwrap_or(false)
    }
}

// ============================================================================
// Response Types - Insider Purchases Summary
// ============================================================================

/// Summary of insider purchase activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderPurchasesSummary {
    pub purchases: Option<i64>,
    pub purchases_shares: Option<i64>,
    pub sales: Option<i64>,
    pub sales_shares: Option<i64>,
    pub net_shares_purchased: Option<i64>,
    pub net_shares_purchased_directly: Option<i64>,
    pub total_insider_shares: Option<i64>,
    pub buy_info_shares: Option<f64>,
    pub sell_info_shares: Option<f64>,
    pub period: Option<String>,
}

impl InsiderPurchasesSummary {
    /// Check if insiders are net buyers
    pub fn is_net_buying(&self) -> bool {
        self.net_shares_purchased.map(|n| n > 0).unwrap_or(false)
    }

    /// Get net transaction count (buys - sells)
    pub fn net_transactions(&self) -> Option<i64> {
        match (self.purchases, self.sales) {
            (Some(p), Some(s)) => Some(p - s),
            _ => None,
        }
    }
}

// ============================================================================
// Response Types - Insider Roster
// ============================================================================

/// A company insider (officer, director, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderRosterMember {
    pub name: String,
    pub relation: Option<String>,
    pub url: Option<String>,
    pub transaction_description: Option<String>,
    pub latest_transaction_date: Option<String>,
    pub position_direct: Option<i64>,
    pub position_indirect: Option<i64>,
}

impl InsiderRosterMember {
    /// Get total shares held (direct + indirect)
    pub fn total_shares(&self) -> i64 {
        self.position_direct.unwrap_or(0) + self.position_indirect.unwrap_or(0)
    }
}

// ============================================================================
// Aggregated Response Types
// ============================================================================

/// Complete holder data for a symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolderData {
    pub symbol: String,
    pub major_holders: Option<MajorHoldersBreakdown>,
    pub institutional_holders: Option<Vec<InstitutionalHolder>>,
    pub mutual_fund_holders: Option<Vec<MutualFundHolder>>,
    pub insider_transactions: Option<Vec<InsiderTransaction>>,
    pub insider_purchases: Option<InsiderPurchasesSummary>,
    pub insider_roster: Option<Vec<InsiderRosterMember>>,
}

impl HolderData {
    /// Get total institutional ownership value
    pub fn total_institutional_value(&self) -> Option<i64> {
        self.institutional_holders.as_ref().map(|holders| {
            holders.iter().filter_map(|h| h.value).sum()
        })
    }

    /// Get total mutual fund ownership value
    pub fn total_mutual_fund_value(&self) -> Option<i64> {
        self.mutual_fund_holders.as_ref().map(|holders| {
            holders.iter().filter_map(|h| h.value).sum()
        })
    }

    /// Count recent insider buys
    pub fn insider_buy_count(&self) -> usize {
        self.insider_transactions
            .as_ref()
            .map(|txns| txns.iter().filter(|t| t.is_buy()).count())
            .unwrap_or(0)
    }

    /// Count recent insider sells
    pub fn insider_sell_count(&self) -> usize {
        self.insider_transactions
            .as_ref()
            .map(|txns| txns.iter().filter(|t| t.is_sell()).count())
            .unwrap_or(0)
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Creates a configured YahooFinanceClient
async fn create_client() -> Result<(Arc<YahooAuthManager>, YahooFinanceClient), YahooError> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    // Prime authentication
    auth_manager.refresh().await?;

    Ok((auth_manager, client))
}


/// Parse major holders breakdown from JSON
fn parse_major_holders(json: &Value) -> Option<MajorHoldersBreakdown> {
    let breakdown = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("majorHoldersBreakdown"))?;

    Some(MajorHoldersBreakdown {
        insiders_percent_held: breakdown.get("insidersPercentHeld").and_then(|v| v.as_f64()),
        institutions_percent_held: breakdown.get("institutionsPercentHeld").and_then(|v| v.as_f64()),
        institutions_float_percent_held: breakdown.get("institutionsFloatPercentHeld").and_then(|v| v.as_f64()),
        institutions_count: breakdown.get("institutionsCount").and_then(|v| v.as_i64()),
    })
}

/// Parse institutional holders from JSON
fn parse_institutional_holders(json: &Value) -> Option<Vec<InstitutionalHolder>> {
    let ownership = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("institutionOwnership"))
        .and_then(|o| o.get("ownershipList"))
        .and_then(|l| l.as_array())?;

    let holders: Vec<InstitutionalHolder> = ownership
        .iter()
        .filter_map(|h| {
            let organization = h.get("organization").and_then(|o| o.as_str())?.to_string();
            let shares = h.get("position").and_then(|p| p.as_i64()).unwrap_or(0);

            Some(InstitutionalHolder {
                organization,
                shares,
                date_reported: h.get("reportDate").and_then(|d| d.as_str()).map(|s| {
                    // Convert Unix timestamp to date string if needed
                    if let Ok(ts) = s.parse::<i64>() {
                        chrono::DateTime::from_timestamp(ts, 0)
                            .map(|dt| dt.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| s.to_string())
                    } else {
                        s.to_string()
                    }
                }),
                percent_held: h.get("pctHeld").and_then(|p| p.as_f64()),
                value: h.get("value").and_then(|v| v.as_i64()),
            })
        })
        .collect();

    if holders.is_empty() {
        None
    } else {
        Some(holders)
    }
}

/// Parse mutual fund holders from JSON
fn parse_mutual_fund_holders(json: &Value) -> Option<Vec<MutualFundHolder>> {
    let ownership = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("fundOwnership"))
        .and_then(|o| o.get("ownershipList"))
        .and_then(|l| l.as_array())?;

    let holders: Vec<MutualFundHolder> = ownership
        .iter()
        .filter_map(|h| {
            let organization = h.get("organization").and_then(|o| o.as_str())?.to_string();
            let shares = h.get("position").and_then(|p| p.as_i64()).unwrap_or(0);

            Some(MutualFundHolder {
                organization,
                shares,
                date_reported: h.get("reportDate").and_then(|d| d.as_str()).map(|s| {
                    if let Ok(ts) = s.parse::<i64>() {
                        chrono::DateTime::from_timestamp(ts, 0)
                            .map(|dt| dt.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| s.to_string())
                    } else {
                        s.to_string()
                    }
                }),
                percent_held: h.get("pctHeld").and_then(|p| p.as_f64()),
                value: h.get("value").and_then(|v| v.as_i64()),
            })
        })
        .collect();

    if holders.is_empty() {
        None
    } else {
        Some(holders)
    }
}

/// Parse insider transactions from JSON
fn parse_insider_transactions(json: &Value) -> Option<Vec<InsiderTransaction>> {
    let transactions = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("insiderTransactions"))
        .and_then(|t| t.get("transactions"))
        .and_then(|t| t.as_array())?;

    let txns: Vec<InsiderTransaction> = transactions
        .iter()
        .filter_map(|t| {
            let filer_name = t.get("filerName").and_then(|n| n.as_str())?.to_string();

            Some(InsiderTransaction {
                filer_name,
                filer_relation: t.get("filerRelation").and_then(|r| r.as_str()).map(String::from),
                transaction_text: t.get("transactionText").and_then(|t| t.as_str()).map(String::from),
                start_date: t.get("startDate").and_then(|d| {
                    d.as_i64().map(|ts| {
                        chrono::DateTime::from_timestamp(ts, 0)
                            .map(|dt| dt.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| ts.to_string())
                    })
                }),
                shares: t.get("shares").and_then(|s| s.as_i64()),
                value: t.get("value").and_then(|v| v.as_i64()),
                ownership: t.get("ownership").and_then(|o| o.as_str()).map(String::from),
            })
        })
        .collect();

    if txns.is_empty() {
        None
    } else {
        Some(txns)
    }
}

/// Parse insider purchases summary from JSON
fn parse_insider_purchases(json: &Value) -> Option<InsiderPurchasesSummary> {
    let activity = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("netSharePurchaseActivity"))?;

    Some(InsiderPurchasesSummary {
        purchases: activity.get("buyInfoCount").and_then(|v| v.as_i64()),
        purchases_shares: activity.get("buyInfoShares").and_then(|v| v.as_i64()),
        sales: activity.get("sellInfoCount").and_then(|v| v.as_i64()),
        sales_shares: activity.get("sellInfoShares").and_then(|v| v.as_i64()),
        net_shares_purchased: activity.get("netInfoCount").and_then(|v| v.as_i64()),
        net_shares_purchased_directly: activity.get("netPercentInsiderShares").and_then(|v| v.as_i64()),
        total_insider_shares: activity.get("totalInsiderShares").and_then(|v| v.as_i64()),
        buy_info_shares: activity.get("buyPercentInsiderShares").and_then(|v| v.as_f64()),
        sell_info_shares: activity.get("sellPercentInsiderShares").and_then(|v| v.as_f64()),
        period: activity.get("period").and_then(|p| p.as_str()).map(String::from),
    })
}

/// Parse insider roster from JSON
fn parse_insider_roster(json: &Value) -> Option<Vec<InsiderRosterMember>> {
    let holders = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("insiderHolders"))
        .and_then(|h| h.get("holders"))
        .and_then(|h| h.as_array())?;

    let roster: Vec<InsiderRosterMember> = holders
        .iter()
        .filter_map(|h| {
            let name = h.get("name").and_then(|n| n.as_str())?.to_string();

            Some(InsiderRosterMember {
                name,
                relation: h.get("relation").and_then(|r| r.as_str()).map(String::from),
                url: h.get("url").and_then(|u| u.as_str()).map(String::from),
                transaction_description: h.get("transactionDescription").and_then(|t| t.as_str()).map(String::from),
                latest_transaction_date: h.get("latestTransDate").and_then(|d| {
                    d.as_i64().map(|ts| {
                        chrono::DateTime::from_timestamp(ts, 0)
                            .map(|dt| dt.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| ts.to_string())
                    })
                }),
                position_direct: h.get("positionDirect").and_then(|p| p.as_i64()),
                position_indirect: h.get("positionIndirect").and_then(|p| p.as_i64()),
            })
        })
        .collect();

    if roster.is_empty() {
        None
    } else {
        Some(roster)
    }
}


// ============================================================================
// Public API Functions
// ============================================================================

/// Get raw holder data JSON for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `modules` - Slice of module names to fetch
///
/// # Returns
/// Raw JSON value containing holder data
pub async fn get_holders_raw(symbol: &str, modules: &[&str]) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_quote_summary(symbol, modules).await
}

/// Get major holders breakdown for a symbol
///
/// Shows ownership percentages: insiders, institutions, float held by institutions
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// MajorHoldersBreakdown with ownership percentages
///
/// # Example
/// ```rust,ignore
/// let breakdown = get_major_holders("AAPL").await?;
/// if let Some(inst_pct) = breakdown.institutions_percent_held {
///     println!("Institutional ownership: {:.1}%", inst_pct * 100.0);
/// }
/// ```
pub async fn get_major_holders(symbol: &str) -> Result<MajorHoldersBreakdown, YahooError> {
    let json = get_holders_raw(symbol, &["majorHoldersBreakdown"]).await?;
    parse_major_holders(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse major holders breakdown".to_string())
    })
}

/// Get institutional holders for a symbol
///
/// Returns top institutional investors (hedge funds, pension funds, etc.)
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of InstitutionalHolder with holder details
///
/// # Example
/// ```rust,ignore
/// let holders = get_institutional_holders("AAPL").await?;
/// for holder in holders.iter().take(5) {
///     println!("{}: {} shares", holder.organization, holder.shares);
/// }
/// ```
pub async fn get_institutional_holders(symbol: &str) -> Result<Vec<InstitutionalHolder>, YahooError> {
    let json = get_holders_raw(symbol, &["institutionOwnership"]).await?;
    parse_institutional_holders(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse institutional holders".to_string())
    })
}

/// Get mutual fund holders for a symbol
///
/// Returns top mutual fund investors
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of MutualFundHolder with holder details
///
/// # Example
/// ```rust,ignore
/// let holders = get_mutual_fund_holders("AAPL").await?;
/// for holder in holders.iter().take(5) {
///     println!("{}: {} shares", holder.organization, holder.shares);
/// }
/// ```
pub async fn get_mutual_fund_holders(symbol: &str) -> Result<Vec<MutualFundHolder>, YahooError> {
    let json = get_holders_raw(symbol, &["fundOwnership"]).await?;
    parse_mutual_fund_holders(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse mutual fund holders".to_string())
    })
}

/// Get insider transactions for a symbol
///
/// Returns recent insider buy/sell transactions
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of InsiderTransaction with transaction details
///
/// # Example
/// ```rust,ignore
/// let transactions = get_insider_transactions("AAPL").await?;
/// for txn in transactions.iter().take(5) {
///     println!("{}: {} - {} shares", txn.filer_name, txn.transaction_text.as_deref().unwrap_or("N/A"), txn.shares.unwrap_or(0));
/// }
/// ```
pub async fn get_insider_transactions(symbol: &str) -> Result<Vec<InsiderTransaction>, YahooError> {
    let json = get_holders_raw(symbol, &["insiderTransactions"]).await?;
    parse_insider_transactions(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse insider transactions".to_string())
    })
}

/// Get insider purchases summary for a symbol
///
/// Returns aggregated insider purchase/sale activity
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// InsiderPurchasesSummary with aggregated activity
///
/// # Example
/// ```rust,ignore
/// let summary = get_insider_purchases("AAPL").await?;
/// println!("Net insider activity: {} shares", summary.net_shares_purchased.unwrap_or(0));
/// ```
pub async fn get_insider_purchases(symbol: &str) -> Result<InsiderPurchasesSummary, YahooError> {
    let json = get_holders_raw(symbol, &["netSharePurchaseActivity"]).await?;
    parse_insider_purchases(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse insider purchases".to_string())
    })
}

/// Get insider roster for a symbol
///
/// Returns list of company insiders (officers, directors)
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of InsiderRosterMember with insider details
///
/// # Example
/// ```rust,ignore
/// let roster = get_insider_roster("AAPL").await?;
/// for insider in roster.iter().take(5) {
///     println!("{} ({}): {} shares", insider.name, insider.relation.as_deref().unwrap_or("N/A"), insider.total_shares());
/// }
/// ```
pub async fn get_insider_roster(symbol: &str) -> Result<Vec<InsiderRosterMember>, YahooError> {
    let json = get_holders_raw(symbol, &["insiderHolders"]).await?;
    parse_insider_roster(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse insider roster".to_string())
    })
}

/// Get all holder data for a symbol
///
/// Fetches all holder-related data in a single request
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// HolderData with all holder information
///
/// # Example
/// ```rust,ignore
/// let data = get_all_holders("AAPL").await?;
/// if let Some(breakdown) = &data.major_holders {
///     println!("Institutional: {:.1}%", breakdown.institutions_percent_held.unwrap_or(0.0) * 100.0);
/// }
/// println!("Insider buys: {}, sells: {}", data.insider_buy_count(), data.insider_sell_count());
/// ```
pub async fn get_all_holders(symbol: &str) -> Result<HolderData, YahooError> {
    let modules = &[
        "majorHoldersBreakdown",
        "institutionOwnership",
        "fundOwnership",
        "insiderTransactions",
        "netSharePurchaseActivity",
        "insiderHolders",
    ];

    let json = get_holders_raw(symbol, modules).await?;

    Ok(HolderData {
        symbol: symbol.to_string(),
        major_holders: parse_major_holders(&json),
        institutional_holders: parse_institutional_holders(&json),
        mutual_fund_holders: parse_mutual_fund_holders(&json),
        insider_transactions: parse_insider_transactions(&json),
        insider_purchases: parse_insider_purchases(&json),
        insider_roster: parse_insider_roster(&json),
    })
}

/// Get specific holder types for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol
/// * `holder_types` - Slice of HolderType to fetch
///
/// # Returns
/// HolderData with only the requested holder types populated
///
/// # Example
/// ```rust,ignore
/// let data = get_custom_holders(
///     "AAPL",
///     &[HolderType::Major, HolderType::Institutional]
/// ).await?;
/// ```
pub async fn get_custom_holders(
    symbol: &str,
    holder_types: &[HolderType],
) -> Result<HolderData, YahooError> {
    let modules: Vec<&str> = holder_types.iter().map(|t| t.as_module()).collect();
    let json = get_holders_raw(symbol, &modules).await?;

    let mut data = HolderData {
        symbol: symbol.to_string(),
        major_holders: None,
        institutional_holders: None,
        mutual_fund_holders: None,
        insider_transactions: None,
        insider_purchases: None,
        insider_roster: None,
    };

    for holder_type in holder_types {
        match holder_type {
            HolderType::Major => data.major_holders = parse_major_holders(&json),
            HolderType::Institutional => data.institutional_holders = parse_institutional_holders(&json),
            HolderType::MutualFund => data.mutual_fund_holders = parse_mutual_fund_holders(&json),
            HolderType::InsiderTransactions => data.insider_transactions = parse_insider_transactions(&json),
            HolderType::InsiderPurchases => data.insider_purchases = parse_insider_purchases(&json),
            HolderType::InsiderRoster => data.insider_roster = parse_insider_roster(&json),
        }
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require network access and valid Yahoo Finance authentication
    // Run with: cargo test -p package-test -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_get_major_holders() {
        let result = get_major_holders("AAPL").await;
        assert!(result.is_ok());
        let breakdown = result.unwrap();
        assert!(breakdown.institutions_percent_held.is_some());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_institutional_holders() {
        let result = get_institutional_holders("AAPL").await;
        assert!(result.is_ok());
        let holders = result.unwrap();
        assert!(!holders.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_mutual_fund_holders() {
        let result = get_mutual_fund_holders("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_insider_transactions() {
        let result = get_insider_transactions("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_insider_purchases() {
        let result = get_insider_purchases("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_insider_roster() {
        let result = get_insider_roster("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_all_holders() {
        let result = get_all_holders("AAPL").await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_custom_holders() {
        let result = get_custom_holders(
            "AAPL",
            &[HolderType::Major, HolderType::Institutional],
        )
        .await;
        assert!(result.is_ok());
    }
}