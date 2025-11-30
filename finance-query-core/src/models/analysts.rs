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

/// EPS trend data showing how estimates have changed over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpsTrend {
    pub period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "7daysAgo")]
    pub days_7_ago: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "30daysAgo")]
    pub days_30_ago: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "60daysAgo")]
    pub days_60_ago: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "90daysAgo")]
    pub days_90_ago: Option<f64>,
}

/// EPS revisions showing analyst estimate changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpsRevisions {
    pub period: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "upLast7days")]
    pub up_last_7_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "upLast30days")]
    pub up_last_30_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "downLast7days")]
    pub down_last_7_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "downLast30days")]
    pub down_last_30_days: Option<i32>,
}

/// Growth estimates comparing stock to industry/sector/index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthEstimate {
    pub period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpsTrendResponse {
    pub symbol: String,
    #[serde(rename = "epsTrend")]
    pub eps_trend: Vec<EpsTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpsRevisionsResponse {
    pub symbol: String,
    #[serde(rename = "epsRevisions")]
    pub eps_revisions: Vec<EpsRevisions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthEstimatesResponse {
    pub symbol: String,
    #[serde(rename = "growthEstimates")]
    pub growth_estimates: Vec<GrowthEstimate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn optional_i32() -> impl Strategy<Value = Option<i32>> {
        proptest::option::of(0i32..1000i32)
    }

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn recommendation_data_roundtrip(
            period in "[0-9]{4}-[0-9]{2}",
            strong_buy in optional_i32(),
            buy in optional_i32(),
            hold in optional_i32(),
            sell in optional_i32(),
            strong_sell in optional_i32(),
        ) {
            let rec = RecommendationData {
                period: period.clone(),
                strong_buy,
                buy,
                hold,
                sell,
                strong_sell,
            };

            let json = serde_json::to_string(&rec).unwrap();
            let parsed: RecommendationData = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(rec.period, parsed.period);
            prop_assert_eq!(rec.strong_buy, parsed.strong_buy);
            prop_assert_eq!(rec.buy, parsed.buy);
            prop_assert_eq!(rec.hold, parsed.hold);
            prop_assert_eq!(rec.sell, parsed.sell);
            prop_assert_eq!(rec.strong_sell, parsed.strong_sell);
        }

        #[test]
        fn analysis_type_roundtrip(at in prop_oneof![
            Just(AnalysisType::Recommendations),
            Just(AnalysisType::UpgradesDowngrades),
            Just(AnalysisType::PriceTargets),
            Just(AnalysisType::EarningsEstimate),
            Just(AnalysisType::RevenueEstimate),
            Just(AnalysisType::EarningsHistory),
        ]) {
            let json = serde_json::to_string(&at).unwrap();
            let parsed: AnalysisType = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(at.as_str(), parsed.as_str());
        }
    }
}
