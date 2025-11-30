use async_graphql::*;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use finance_query_core::models::analysts::{
    RecommendationData as RecommendationDataModel,
    UpgradeDowngrade as UpgradeDowngradeModel,
    PriceTarget as PriceTargetModel,
    EarningsEstimate as EarningsEstimateModel,
    RevenueEstimate as RevenueEstimateModel,
    EarningsHistoryItem as EarningsHistoryItemModel,
    RecommendationsResponse as RecommendationsResponseModel,
    UpgradesDowngradesResponse as UpgradesDowngradesResponseModel,
    PriceTargetsResponse as PriceTargetsResponseModel,
    EarningsEstimateResponse as EarningsEstimateResponseModel,
    RevenueEstimateResponse as RevenueEstimateResponseModel,
    EarningsHistoryResponse as EarningsHistoryResponseModel,
};

#[derive(SimpleObject, Clone)]
pub struct RecommendationData {
    pub period: String,
    #[graphql(name = "strongBuy")]
    pub strong_buy: Option<i32>,
    pub buy: Option<i32>,
    pub hold: Option<i32>,
    pub sell: Option<i32>,
    #[graphql(name = "strongSell")]
    pub strong_sell: Option<i32>,
}

impl From<RecommendationDataModel> for RecommendationData {
    fn from(data: RecommendationDataModel) -> Self {
        RecommendationData {
            period: data.period,
            strong_buy: data.strong_buy,
            buy: data.buy,
            hold: data.hold,
            sell: data.sell,
            strong_sell: data.strong_sell,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct UpgradeDowngrade {
    pub firm: String,
    #[graphql(name = "toGrade")]
    pub to_grade: Option<String>,
    #[graphql(name = "fromGrade")]
    pub from_grade: Option<String>,
    pub action: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

impl From<UpgradeDowngradeModel> for UpgradeDowngrade {
    fn from(upgrade: UpgradeDowngradeModel) -> Self {
        UpgradeDowngrade {
            firm: upgrade.firm,
            to_grade: upgrade.to_grade,
            from_grade: upgrade.from_grade,
            action: upgrade.action,
            date: upgrade.date,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct PriceTarget {
    pub current: Option<f64>,
    pub mean: Option<f64>,
    pub median: Option<f64>,
    pub low: Option<f64>,
    pub high: Option<f64>,
}

impl From<PriceTargetModel> for PriceTarget {
    fn from(target: PriceTargetModel) -> Self {
        PriceTarget {
            current: target.current,
            mean: target.mean,
            median: target.median,
            low: target.low,
            high: target.high,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct EarningsEstimate {
    pub estimates: HashMap<String, async_graphql::Json<serde_json::Value>>,
}

impl From<EarningsEstimateModel> for EarningsEstimate {
    fn from(estimate: EarningsEstimateModel) -> Self {
        EarningsEstimate {
            estimates: estimate.estimates.into_iter()
                .map(|(k, v)| (k, async_graphql::Json(v)))
                .collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct RevenueEstimate {
    pub estimates: HashMap<String, async_graphql::Json<serde_json::Value>>,
}

impl From<RevenueEstimateModel> for RevenueEstimate {
    fn from(estimate: RevenueEstimateModel) -> Self {
        RevenueEstimate {
            estimates: estimate.estimates.into_iter()
                .map(|(k, v)| (k, async_graphql::Json(v)))
                .collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct EarningsHistoryItem {
    pub date: DateTime<Utc>,
    #[graphql(name = "epsActual")]
    pub eps_actual: Option<f64>,
    #[graphql(name = "epsEstimate")]
    pub eps_estimate: Option<f64>,
    pub surprise: Option<f64>,
    #[graphql(name = "surprisePercent")]
    pub surprise_percent: Option<f64>,
}

impl From<EarningsHistoryItemModel> for EarningsHistoryItem {
    fn from(item: EarningsHistoryItemModel) -> Self {
        EarningsHistoryItem {
            date: item.date,
            eps_actual: item.eps_actual,
            eps_estimate: item.eps_estimate,
            surprise: item.surprise,
            surprise_percent: item.surprise_percent,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct RecommendationsResponse {
    pub symbol: String,
    pub recommendations: Vec<RecommendationData>,
}

impl From<RecommendationsResponseModel> for RecommendationsResponse {
    fn from(response: RecommendationsResponseModel) -> Self {
        RecommendationsResponse {
            symbol: response.symbol,
            recommendations: response.recommendations.into_iter().map(RecommendationData::from).collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct UpgradesDowngradesResponse {
    pub symbol: String,
    #[graphql(name = "upgradesDowngrades")]
    pub upgrades_downgrades: Vec<UpgradeDowngrade>,
}

impl From<UpgradesDowngradesResponseModel> for UpgradesDowngradesResponse {
    fn from(response: UpgradesDowngradesResponseModel) -> Self {
        UpgradesDowngradesResponse {
            symbol: response.symbol,
            upgrades_downgrades: response.upgrades_downgrades.into_iter().map(UpgradeDowngrade::from).collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct PriceTargetsResponse {
    pub symbol: String,
    #[graphql(name = "priceTargets")]
    pub price_targets: PriceTarget,
}

impl From<PriceTargetsResponseModel> for PriceTargetsResponse {
    fn from(response: PriceTargetsResponseModel) -> Self {
        PriceTargetsResponse {
            symbol: response.symbol,
            price_targets: PriceTarget::from(response.price_targets),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct EarningsEstimateResponse {
    pub symbol: String,
    #[graphql(name = "earningsEstimate")]
    pub earnings_estimate: EarningsEstimate,
}

impl From<EarningsEstimateResponseModel> for EarningsEstimateResponse {
    fn from(response: EarningsEstimateResponseModel) -> Self {
        EarningsEstimateResponse {
            symbol: response.symbol,
            earnings_estimate: EarningsEstimate::from(response.earnings_estimate),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct RevenueEstimateResponse {
    pub symbol: String,
    #[graphql(name = "revenueEstimate")]
    pub revenue_estimate: RevenueEstimate,
}

impl From<RevenueEstimateResponseModel> for RevenueEstimateResponse {
    fn from(response: RevenueEstimateResponseModel) -> Self {
        RevenueEstimateResponse {
            symbol: response.symbol,
            revenue_estimate: RevenueEstimate::from(response.revenue_estimate),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct EarningsHistoryResponse {
    pub symbol: String,
    #[graphql(name = "earningsHistory")]
    pub earnings_history: Vec<EarningsHistoryItem>,
}

impl From<EarningsHistoryResponseModel> for EarningsHistoryResponse {
    fn from(response: EarningsHistoryResponseModel) -> Self {
        EarningsHistoryResponse {
            symbol: response.symbol,
            earnings_history: response.earnings_history.into_iter().map(EarningsHistoryItem::from).collect(),
        }
    }
}