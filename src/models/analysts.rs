use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisType {
    #[serde(rename = "recommendations")]
    Recommendations,
    #[serde(rename = "upgrades_downgrades")]
    UpgradesDowngrades,
    #[serde(rename = "price_targets")]
    PriceTargets,
    #[serde(rename = "earnings_estimate")]
    EarningsEstimate,
    #[serde(rename = "revenue_estimate")]
    RevenueEstimate,
    #[serde(rename = "earnings_history")]
    EarningsHistory,
}

impl AnalysisType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalysisType::Recommendations => "recommendations",
            AnalysisType::UpgradesDowngrades => "upgrades_downgrades",
            AnalysisType::PriceTargets => "price_targets",
            AnalysisType::EarningsEstimate => "earnings_estimate",
            AnalysisType::RevenueEstimate => "revenue_estimate",
            AnalysisType::EarningsHistory => "earnings_history",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationData {
    pub period: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "strongBuy")]
    pub strong_buy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "strongSell")]
    pub strong_sell: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDowngrade {
    pub firm: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "toGrade")]
    pub to_grade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromGrade")]
    pub from_grade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub median: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsEstimate {
    pub estimates: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEstimate {
    pub estimates: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsHistoryItem {
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "epsActual")]
    pub eps_actual: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "epsEstimate")]
    pub eps_estimate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surprise: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "surprisePercent")]
    pub surprise_percent: Option<f64>,
}

// Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationsResponse {
    pub symbol: String,
    pub recommendations: Vec<RecommendationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradesDowngradesResponse {
    pub symbol: String,
    #[serde(rename = "upgradesDowngrades")]
    pub upgrades_downgrades: Vec<UpgradeDowngrade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTargetsResponse {
    pub symbol: String,
    #[serde(rename = "priceTargets")]
    pub price_targets: PriceTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsEstimateResponse {
    pub symbol: String,
    #[serde(rename = "earningsEstimate")]
    pub earnings_estimate: EarningsEstimate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEstimateResponse {
    pub symbol: String,
    #[serde(rename = "revenueEstimate")]
    pub revenue_estimate: RevenueEstimate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsHistoryResponse {
    pub symbol: String,
    #[serde(rename = "earningsHistory")]
    pub earnings_history: Vec<EarningsHistoryItem>,
}