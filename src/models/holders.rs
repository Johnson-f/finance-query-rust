use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HolderType {
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "institutional")]
    Institutional,
    #[serde(rename = "mutualfund")]
    MutualFund,
    #[serde(rename = "insider_transactions")]
    InsiderTransactions,
    #[serde(rename = "insider_purchases")]
    InsiderPurchases,
    #[serde(rename = "insider_roster")]
    InsiderRoster,
}

impl HolderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            HolderType::Major => "major",
            HolderType::Institutional => "institutional",
            HolderType::MutualFund => "mutualfund",
            HolderType::InsiderTransactions => "insider_transactions",
            HolderType::InsiderPurchases => "insider_purchases",
            HolderType::InsiderRoster => "insider_roster",
        }
    }

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "major" => Some(HolderType::Major),
            "institutional" => Some(HolderType::Institutional),
            "mutualfund" => Some(HolderType::MutualFund),
            "insider_transactions" => Some(HolderType::InsiderTransactions),
            "insider_purchases" => Some(HolderType::InsiderPurchases),
            "insider_roster" => Some(HolderType::InsiderRoster),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorHoldersBreakdown {
    #[serde(rename = "breakdownData")]
    pub breakdown_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalHolder {
    pub holder: String,
    pub shares: i64,
    #[serde(rename = "dateReported")]
    pub date_reported: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "percentOut")]
    pub percent_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundHolder {
    pub holder: String,
    pub shares: i64,
    #[serde(rename = "dateReported")]
    pub date_reported: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "percentOut")]
    pub percent_out: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderTransaction {
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,
    pub insider: String,
    pub position: String,
    pub transaction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderPurchase {
    pub period: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "purchasesShares")]
    pub purchases_shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "purchasesTransactions")]
    pub purchases_transactions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "salesShares")]
    pub sales_shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "salesTransactions")]
    pub sales_transactions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "netShares")]
    pub net_shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "netTransactions")]
    pub net_transactions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "totalInsiderShares")]
    pub total_insider_shares: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "netPercentInsiderShares")]
    pub net_percent_insider_shares: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "buyPercentInsiderShares")]
    pub buy_percent_insider_shares: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sellPercentInsiderShares")]
    pub sell_percent_insider_shares: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderRosterMember {
    pub name: String,
    pub position: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mostRecentTransaction")]
    pub most_recent_transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "latestTransactionDate")]
    pub latest_transaction_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sharesOwnedDirectly")]
    pub shares_owned_directly: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sharesOwnedIndirectly")]
    pub shares_owned_indirectly: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionDirectDate")]
    pub position_direct_date: Option<DateTime<Utc>>,
}

// Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorHoldersResponse {
    pub symbol: String,
    pub breakdown: MajorHoldersBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionalHoldersResponse {
    pub symbol: String,
    pub holders: Vec<InstitutionalHolder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundHoldersResponse {
    pub symbol: String,
    pub holders: Vec<MutualFundHolder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderTransactionsResponse {
    pub symbol: String,
    pub transactions: Vec<InsiderTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderPurchasesResponse {
    pub symbol: String,
    pub summary: InsiderPurchase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsiderRosterResponse {
    pub symbol: String,
    pub roster: Vec<InsiderRosterMember>,
}

// Internal data structure used by service layer
#[derive(Debug, Clone)]
pub struct HoldersData {
    pub symbol: String,
    #[allow(dead_code)]
    pub holder_type: HolderType,
    pub major_breakdown: Option<MajorHoldersBreakdown>,
    pub institutional_holders: Option<Vec<InstitutionalHolder>>,
    pub mutualfund_holders: Option<Vec<MutualFundHolder>>,
    pub insider_transactions: Option<Vec<InsiderTransaction>>,
    pub insider_purchases: Option<InsiderPurchase>,
    pub insider_roster: Option<Vec<InsiderRosterMember>>,
}