//! Analyst data fetching functions using finance-query-core
//!
//! This module provides functions to fetch analyst data from Yahoo Finance:
//! - Analyst recommendations (buy/hold/sell ratings)
//! - Upgrades and downgrades from analyst firms
//! - Price targets (current, mean, median, low, high)
//! - Earnings estimates and history
//! - Revenue estimates
//! - EPS trends and revisions
//! - Growth estimates

use finance_query_core::{FetchClient, YahooAuthManager, YahooError, YahooFinanceClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// ============================================================================
// Analysis Type Enum
// ============================================================================

/// Type of analyst data to fetch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisType {
    /// Analyst recommendations (buy/hold/sell)
    Recommendations,
    /// Upgrades and downgrades
    UpgradesDowngrades,
    /// Price targets
    PriceTargets,
    /// Earnings estimates
    EarningsEstimate,
    /// Revenue estimates
    RevenueEstimate,
    /// Earnings history
    EarningsHistory,
    /// EPS trend
    EpsTrend,
    /// EPS revisions
    EpsRevisions,
    /// Growth estimates
    GrowthEstimates,
}

impl AnalysisType {
    pub fn as_module(&self) -> &'static str {
        match self {
            AnalysisType::Recommendations => "recommendationTrend",
            AnalysisType::UpgradesDowngrades => "upgradeDowngradeHistory",
            AnalysisType::PriceTargets => "financialData",
            AnalysisType::EarningsEstimate => "earningsTrend",
            AnalysisType::RevenueEstimate => "earningsTrend",
            AnalysisType::EarningsHistory => "earningsHistory",
            AnalysisType::EpsTrend => "earningsTrend",
            AnalysisType::EpsRevisions => "earningsTrend",
            AnalysisType::GrowthEstimates => "earningsTrend",
        }
    }
}

// ============================================================================
// Response Types - Recommendations
// ============================================================================

/// Analyst recommendation data for a period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationData {
    pub period: String,
    pub strong_buy: i32,
    pub buy: i32,
    pub hold: i32,
    pub sell: i32,
    pub strong_sell: i32,
}

impl RecommendationData {
    /// Get total number of analysts
    pub fn total_analysts(&self) -> i32 {
        self.strong_buy + self.buy + self.hold + self.sell + self.strong_sell
    }

    /// Get consensus rating as a string
    pub fn consensus(&self) -> &'static str {
        let bullish = self.strong_buy + self.buy;
        let bearish = self.sell + self.strong_sell;

        if bullish > bearish * 2 {
            "Strong Buy"
        } else if bullish > bearish {
            "Buy"
        } else if bearish > bullish * 2 {
            "Strong Sell"
        } else if bearish > bullish {
            "Sell"
        } else {
            "Hold"
        }
    }

    /// Get bullish percentage
    pub fn bullish_percent(&self) -> f64 {
        let total = self.total_analysts() as f64;
        if total == 0.0 {
            return 0.0;
        }
        ((self.strong_buy + self.buy) as f64 / total) * 100.0
    }
}

// ============================================================================
// Response Types - Upgrades/Downgrades
// ============================================================================

/// An upgrade or downgrade from an analyst firm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDowngrade {
    pub firm: String,
    pub to_grade: Option<String>,
    pub from_grade: Option<String>,
    pub action: Option<String>,
    pub date: Option<String>,
}

impl UpgradeDowngrade {
    /// Check if this is an upgrade
    pub fn is_upgrade(&self) -> bool {
        self.action
            .as_ref()
            .map(|a| a.to_lowercase().contains("upgrade") || a.to_lowercase() == "up")
            .unwrap_or(false)
    }

    /// Check if this is a downgrade
    pub fn is_downgrade(&self) -> bool {
        self.action
            .as_ref()
            .map(|a| a.to_lowercase().contains("downgrade") || a.to_lowercase() == "down")
            .unwrap_or(false)
    }
}

// ============================================================================
// Response Types - Price Targets
// ============================================================================

/// Price target data from analysts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTarget {
    pub current_price: Option<f64>,
    pub target_mean: Option<f64>,
    pub target_median: Option<f64>,
    pub target_low: Option<f64>,
    pub target_high: Option<f64>,
    pub number_of_analysts: Option<i32>,
}

impl PriceTarget {
    /// Get upside potential as percentage
    pub fn upside_percent(&self) -> Option<f64> {
        match (self.current_price, self.target_mean) {
            (Some(current), Some(target)) if current > 0.0 => {
                Some(((target - current) / current) * 100.0)
            }
            _ => None,
        }
    }

    /// Get the price range spread
    pub fn target_spread(&self) -> Option<f64> {
        match (self.target_low, self.target_high) {
            (Some(low), Some(high)) => Some(high - low),
            _ => None,
        }
    }
}

// ============================================================================
// Response Types - Earnings
// ============================================================================

/// Earnings estimate for a period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsEstimate {
    pub period: String,
    pub end_date: Option<String>,
    pub avg: Option<f64>,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub year_ago_eps: Option<f64>,
    pub number_of_analysts: Option<i32>,
    pub growth: Option<f64>,
}

/// Revenue estimate for a period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEstimate {
    pub period: String,
    pub end_date: Option<String>,
    pub avg: Option<f64>,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub year_ago_revenue: Option<f64>,
    pub number_of_analysts: Option<i32>,
    pub growth: Option<f64>,
}

/// Historical earnings data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsHistoryItem {
    pub quarter: Option<String>,
    pub date: Option<String>,
    pub eps_actual: Option<f64>,
    pub eps_estimate: Option<f64>,
    pub surprise: Option<f64>,
    pub surprise_percent: Option<f64>,
}

impl EarningsHistoryItem {
    /// Check if earnings beat estimates
    pub fn beat_estimate(&self) -> Option<bool> {
        self.surprise.map(|s| s > 0.0)
    }
}

/// EPS trend showing estimate changes over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpsTrend {
    pub period: String,
    pub current: Option<f64>,
    pub days_7_ago: Option<f64>,
    pub days_30_ago: Option<f64>,
    pub days_60_ago: Option<f64>,
    pub days_90_ago: Option<f64>,
}

impl EpsTrend {
    /// Get the change from 30 days ago
    pub fn change_30d(&self) -> Option<f64> {
        match (self.current, self.days_30_ago) {
            (Some(current), Some(ago)) => Some(current - ago),
            _ => None,
        }
    }
}

/// EPS revisions data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpsRevisions {
    pub period: String,
    pub up_last_7_days: Option<i32>,
    pub up_last_30_days: Option<i32>,
    pub down_last_7_days: Option<i32>,
    pub down_last_30_days: Option<i32>,
}

impl EpsRevisions {
    /// Get net revisions (up - down) for last 30 days
    pub fn net_revisions_30d(&self) -> Option<i32> {
        match (self.up_last_30_days, self.down_last_30_days) {
            (Some(up), Some(down)) => Some(up - down),
            _ => None,
        }
    }
}

/// Growth estimate comparing stock to peers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthEstimate {
    pub period: String,
    pub stock: Option<f64>,
    pub industry: Option<f64>,
    pub sector: Option<f64>,
    pub index: Option<f64>,
}

impl GrowthEstimate {
    /// Check if stock growth beats industry
    pub fn beats_industry(&self) -> Option<bool> {
        match (self.stock, self.industry) {
            (Some(stock), Some(industry)) => Some(stock > industry),
            _ => None,
        }
    }
}


// ============================================================================
// Aggregated Response Types
// ============================================================================

/// Complete analyst data for a symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalystData {
    pub symbol: String,
    pub recommendations: Option<Vec<RecommendationData>>,
    pub upgrades_downgrades: Option<Vec<UpgradeDowngrade>>,
    pub price_target: Option<PriceTarget>,
    pub earnings_estimates: Option<Vec<EarningsEstimate>>,
    pub revenue_estimates: Option<Vec<RevenueEstimate>>,
    pub earnings_history: Option<Vec<EarningsHistoryItem>>,
    pub eps_trend: Option<Vec<EpsTrend>>,
    pub eps_revisions: Option<Vec<EpsRevisions>>,
    pub growth_estimates: Option<Vec<GrowthEstimate>>,
}

impl AnalystData {
    /// Get the latest recommendation
    pub fn latest_recommendation(&self) -> Option<&RecommendationData> {
        self.recommendations.as_ref().and_then(|r| r.first())
    }

    /// Count recent upgrades
    pub fn upgrade_count(&self) -> usize {
        self.upgrades_downgrades
            .as_ref()
            .map(|ud| ud.iter().filter(|u| u.is_upgrade()).count())
            .unwrap_or(0)
    }

    /// Count recent downgrades
    pub fn downgrade_count(&self) -> usize {
        self.upgrades_downgrades
            .as_ref()
            .map(|ud| ud.iter().filter(|u| u.is_downgrade()).count())
            .unwrap_or(0)
    }

    /// Get earnings beat rate
    pub fn earnings_beat_rate(&self) -> Option<f64> {
        self.earnings_history.as_ref().map(|history| {
            let beats = history.iter().filter(|h| h.beat_estimate() == Some(true)).count();
            let total = history.len();
            if total == 0 {
                0.0
            } else {
                (beats as f64 / total as f64) * 100.0
            }
        })
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

/// Parse recommendations from JSON
fn parse_recommendations(json: &Value) -> Option<Vec<RecommendationData>> {
    let trend = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("recommendationTrend"))
        .and_then(|t| t.get("trend"))
        .and_then(|t| t.as_array())?;

    let recs: Vec<RecommendationData> = trend
        .iter()
        .filter_map(|item| {
            let period = item.get("period").and_then(|p| p.as_str())?.to_string();

            Some(RecommendationData {
                period,
                strong_buy: item.get("strongBuy").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                buy: item.get("buy").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                hold: item.get("hold").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                sell: item.get("sell").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                strong_sell: item.get("strongSell").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            })
        })
        .collect();

    if recs.is_empty() { None } else { Some(recs) }
}

/// Parse upgrades/downgrades from JSON
fn parse_upgrades_downgrades(json: &Value) -> Option<Vec<UpgradeDowngrade>> {
    let history = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("upgradeDowngradeHistory"))
        .and_then(|h| h.get("history"))
        .and_then(|h| h.as_array())?;

    let items: Vec<UpgradeDowngrade> = history
        .iter()
        .filter_map(|item| {
            let firm = item.get("firm").and_then(|f| f.as_str())?.to_string();

            Some(UpgradeDowngrade {
                firm,
                to_grade: item.get("toGrade").and_then(|g| g.as_str()).map(String::from),
                from_grade: item.get("fromGrade").and_then(|g| g.as_str()).map(String::from),
                action: item.get("action").and_then(|a| a.as_str()).map(String::from),
                date: item.get("epochGradeDate").and_then(|d| d.as_i64()).map(|ts| {
                    chrono::DateTime::from_timestamp(ts, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| ts.to_string())
                }),
            })
        })
        .collect();

    if items.is_empty() { None } else { Some(items) }
}

/// Parse price targets from JSON
fn parse_price_targets(json: &Value) -> Option<PriceTarget> {
    let financial = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("financialData"))?;

    Some(PriceTarget {
        current_price: financial.get("currentPrice").and_then(|p| p.as_f64()),
        target_mean: financial.get("targetMeanPrice").and_then(|p| p.as_f64()),
        target_median: financial.get("targetMedianPrice").and_then(|p| p.as_f64()),
        target_low: financial.get("targetLowPrice").and_then(|p| p.as_f64()),
        target_high: financial.get("targetHighPrice").and_then(|p| p.as_f64()),
        number_of_analysts: financial.get("numberOfAnalystOpinions").and_then(|n| n.as_i64()).map(|n| n as i32),
    })
}

/// Parse earnings history from JSON
fn parse_earnings_history(json: &Value) -> Option<Vec<EarningsHistoryItem>> {
    let history = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("earningsHistory"))
        .and_then(|h| h.get("history"))
        .and_then(|h| h.as_array())?;

    let items: Vec<EarningsHistoryItem> = history
        .iter()
        .filter_map(|item| {
            Some(EarningsHistoryItem {
                quarter: item.get("quarter").and_then(|q| q.as_str()).map(String::from),
                date: item.get("quarterDate").and_then(|d| d.as_str()).map(String::from),
                eps_actual: item.get("epsActual").and_then(|e| e.as_f64()),
                eps_estimate: item.get("epsEstimate").and_then(|e| e.as_f64()),
                surprise: item.get("epsDifference").and_then(|s| s.as_f64()),
                surprise_percent: item.get("surprisePercent").and_then(|s| s.as_f64()),
            })
        })
        .collect();

    if items.is_empty() { None } else { Some(items) }
}

/// Parse earnings/revenue estimates from earningsTrend
fn parse_earnings_estimates(json: &Value) -> Option<Vec<EarningsEstimate>> {
    let trend = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("earningsTrend"))
        .and_then(|t| t.get("trend"))
        .and_then(|t| t.as_array())?;

    let estimates: Vec<EarningsEstimate> = trend
        .iter()
        .filter_map(|item| {
            let period = item.get("period").and_then(|p| p.as_str())?.to_string();
            let earnings = item.get("earningsEstimate")?;

            Some(EarningsEstimate {
                period,
                end_date: item.get("endDate").and_then(|d| d.as_str()).map(String::from),
                avg: earnings.get("avg").and_then(|a| a.as_f64()),
                low: earnings.get("low").and_then(|l| l.as_f64()),
                high: earnings.get("high").and_then(|h| h.as_f64()),
                year_ago_eps: earnings.get("yearAgoEps").and_then(|y| y.as_f64()),
                number_of_analysts: earnings.get("numberOfAnalysts").and_then(|n| n.as_i64()).map(|n| n as i32),
                growth: earnings.get("growth").and_then(|g| g.as_f64()),
            })
        })
        .collect();

    if estimates.is_empty() { None } else { Some(estimates) }
}

/// Parse revenue estimates from earningsTrend
fn parse_revenue_estimates(json: &Value) -> Option<Vec<RevenueEstimate>> {
    let trend = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("earningsTrend"))
        .and_then(|t| t.get("trend"))
        .and_then(|t| t.as_array())?;

    let estimates: Vec<RevenueEstimate> = trend
        .iter()
        .filter_map(|item| {
            let period = item.get("period").and_then(|p| p.as_str())?.to_string();
            let revenue = item.get("revenueEstimate")?;

            Some(RevenueEstimate {
                period,
                end_date: item.get("endDate").and_then(|d| d.as_str()).map(String::from),
                avg: revenue.get("avg").and_then(|a| a.as_f64()),
                low: revenue.get("low").and_then(|l| l.as_f64()),
                high: revenue.get("high").and_then(|h| h.as_f64()),
                year_ago_revenue: revenue.get("yearAgoRevenue").and_then(|y| y.as_f64()),
                number_of_analysts: revenue.get("numberOfAnalysts").and_then(|n| n.as_i64()).map(|n| n as i32),
                growth: revenue.get("growth").and_then(|g| g.as_f64()),
            })
        })
        .collect();

    if estimates.is_empty() { None } else { Some(estimates) }
}

/// Parse EPS trend from earningsTrend
fn parse_eps_trend(json: &Value) -> Option<Vec<EpsTrend>> {
    let trend = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("earningsTrend"))
        .and_then(|t| t.get("trend"))
        .and_then(|t| t.as_array())?;

    let items: Vec<EpsTrend> = trend
        .iter()
        .filter_map(|item| {
            let period = item.get("period").and_then(|p| p.as_str())?.to_string();
            let eps = item.get("epsTrend")?;

            Some(EpsTrend {
                period,
                current: eps.get("current").and_then(|c| c.as_f64()),
                days_7_ago: eps.get("7daysAgo").and_then(|d| d.as_f64()),
                days_30_ago: eps.get("30daysAgo").and_then(|d| d.as_f64()),
                days_60_ago: eps.get("60daysAgo").and_then(|d| d.as_f64()),
                days_90_ago: eps.get("90daysAgo").and_then(|d| d.as_f64()),
            })
        })
        .collect();

    if items.is_empty() { None } else { Some(items) }
}

/// Parse EPS revisions from earningsTrend
fn parse_eps_revisions(json: &Value) -> Option<Vec<EpsRevisions>> {
    let trend = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("earningsTrend"))
        .and_then(|t| t.get("trend"))
        .and_then(|t| t.as_array())?;

    let items: Vec<EpsRevisions> = trend
        .iter()
        .filter_map(|item| {
            let period = item.get("period").and_then(|p| p.as_str())?.to_string();
            let revisions = item.get("epsRevisions")?;

            Some(EpsRevisions {
                period,
                up_last_7_days: revisions.get("upLast7days").and_then(|u| u.as_i64()).map(|n| n as i32),
                up_last_30_days: revisions.get("upLast30days").and_then(|u| u.as_i64()).map(|n| n as i32),
                down_last_7_days: revisions.get("downLast7days").and_then(|d| d.as_i64()).map(|n| n as i32),
                down_last_30_days: revisions.get("downLast30days").and_then(|d| d.as_i64()).map(|n| n as i32),
            })
        })
        .collect();

    if items.is_empty() { None } else { Some(items) }
}

/// Parse growth estimates from earningsTrend
fn parse_growth_estimates(json: &Value) -> Option<Vec<GrowthEstimate>> {
    let trend = json
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .and_then(|r| r.get("earningsTrend"))
        .and_then(|t| t.get("trend"))
        .and_then(|t| t.as_array())?;

    let items: Vec<GrowthEstimate> = trend
        .iter()
        .filter_map(|item| {
            let period = item.get("period").and_then(|p| p.as_str())?.to_string();
            let growth = item.get("growth")?;

            // Growth might be a direct value or nested
            let stock_growth = if growth.is_f64() {
                growth.as_f64()
            } else {
                growth.get("raw").and_then(|r| r.as_f64())
            };

            Some(GrowthEstimate {
                period,
                stock: stock_growth,
                industry: None, // Not always available in this endpoint
                sector: None,
                index: None,
            })
        })
        .collect();

    if items.is_empty() { None } else { Some(items) }
}


// ============================================================================
// Public API Functions
// ============================================================================

/// Get raw analyst data JSON for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `modules` - Slice of module names to fetch
///
/// # Returns
/// Raw JSON value containing analyst data
pub async fn get_analysts_raw(symbol: &str, modules: &[&str]) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;
    client.get_quote_summary(symbol, modules).await
}

/// Get analyst recommendations for a symbol
///
/// Returns buy/hold/sell ratings from analysts
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of RecommendationData with rating breakdowns
///
/// # Example
/// ```rust,ignore
/// let recs = get_recommendations("AAPL").await?;
/// if let Some(latest) = recs.first() {
///     println!("Consensus: {} ({} analysts)", latest.consensus(), latest.total_analysts());
/// }
/// ```
pub async fn get_recommendations(symbol: &str) -> Result<Vec<RecommendationData>, YahooError> {
    let json = get_analysts_raw(symbol, &["recommendationTrend"]).await?;
    parse_recommendations(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse recommendations".to_string())
    })
}

/// Get upgrades and downgrades for a symbol
///
/// Returns recent analyst rating changes
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of UpgradeDowngrade with firm and rating details
///
/// # Example
/// ```rust,ignore
/// let changes = get_upgrades_downgrades("AAPL").await?;
/// for change in changes.iter().take(5) {
///     println!("{}: {} -> {} ({})", change.firm, change.from_grade.as_deref().unwrap_or("N/A"), change.to_grade.as_deref().unwrap_or("N/A"), change.action.as_deref().unwrap_or("N/A"));
/// }
/// ```
pub async fn get_upgrades_downgrades(symbol: &str) -> Result<Vec<UpgradeDowngrade>, YahooError> {
    let json = get_analysts_raw(symbol, &["upgradeDowngradeHistory"]).await?;
    parse_upgrades_downgrades(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse upgrades/downgrades".to_string())
    })
}

/// Get price targets for a symbol
///
/// Returns analyst price target data
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// PriceTarget with mean, median, low, high targets
///
/// # Example
/// ```rust,ignore
/// let targets = get_price_targets("AAPL").await?;
/// if let Some(upside) = targets.upside_percent() {
///     println!("Upside potential: {:.1}%", upside);
/// }
/// ```
pub async fn get_price_targets(symbol: &str) -> Result<PriceTarget, YahooError> {
    let json = get_analysts_raw(symbol, &["financialData"]).await?;
    parse_price_targets(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse price targets".to_string())
    })
}

/// Get earnings history for a symbol
///
/// Returns historical earnings vs estimates
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of EarningsHistoryItem with actual vs estimated EPS
///
/// # Example
/// ```rust,ignore
/// let history = get_earnings_history("AAPL").await?;
/// for item in history.iter().take(4) {
///     let beat = if item.beat_estimate() == Some(true) { "✓" } else { "✗" };
///     println!("{}: ${:.2} vs ${:.2} {}", item.quarter.as_deref().unwrap_or("N/A"), item.eps_actual.unwrap_or(0.0), item.eps_estimate.unwrap_or(0.0), beat);
/// }
/// ```
pub async fn get_earnings_history(symbol: &str) -> Result<Vec<EarningsHistoryItem>, YahooError> {
    let json = get_analysts_raw(symbol, &["earningsHistory"]).await?;
    parse_earnings_history(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse earnings history".to_string())
    })
}

/// Get earnings estimates for a symbol
///
/// Returns forward earnings estimates
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of EarningsEstimate with avg, low, high estimates
pub async fn get_earnings_estimates(symbol: &str) -> Result<Vec<EarningsEstimate>, YahooError> {
    let json = get_analysts_raw(symbol, &["earningsTrend"]).await?;
    parse_earnings_estimates(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse earnings estimates".to_string())
    })
}

/// Get revenue estimates for a symbol
///
/// Returns forward revenue estimates
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of RevenueEstimate with avg, low, high estimates
pub async fn get_revenue_estimates(symbol: &str) -> Result<Vec<RevenueEstimate>, YahooError> {
    let json = get_analysts_raw(symbol, &["earningsTrend"]).await?;
    parse_revenue_estimates(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse revenue estimates".to_string())
    })
}

/// Get EPS trend for a symbol
///
/// Shows how EPS estimates have changed over time
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of EpsTrend with current and historical estimates
pub async fn get_eps_trend(symbol: &str) -> Result<Vec<EpsTrend>, YahooError> {
    let json = get_analysts_raw(symbol, &["earningsTrend"]).await?;
    parse_eps_trend(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse EPS trend".to_string())
    })
}

/// Get EPS revisions for a symbol
///
/// Shows analyst estimate revision activity
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of EpsRevisions with up/down revision counts
pub async fn get_eps_revisions(symbol: &str) -> Result<Vec<EpsRevisions>, YahooError> {
    let json = get_analysts_raw(symbol, &["earningsTrend"]).await?;
    parse_eps_revisions(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse EPS revisions".to_string())
    })
}

/// Get growth estimates for a symbol
///
/// Returns growth estimates compared to peers
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// Vec of GrowthEstimate with stock vs industry/sector growth
pub async fn get_growth_estimates(symbol: &str) -> Result<Vec<GrowthEstimate>, YahooError> {
    let json = get_analysts_raw(symbol, &["earningsTrend"]).await?;
    parse_growth_estimates(&json).ok_or_else(|| {
        YahooError::ParseError("Failed to parse growth estimates".to_string())
    })
}

/// Get all analyst data for a symbol
///
/// Fetches all analyst-related data in a single request
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
///
/// # Returns
/// AnalystData with all analyst information
///
/// # Example
/// ```rust,ignore
/// let data = get_all_analyst_data("AAPL").await?;
/// if let Some(rec) = data.latest_recommendation() {
///     println!("Consensus: {}", rec.consensus());
/// }
/// if let Some(target) = &data.price_target {
///     println!("Target: ${:.2}", target.target_mean.unwrap_or(0.0));
/// }
/// ```
pub async fn get_all_analyst_data(symbol: &str) -> Result<AnalystData, YahooError> {
    let modules = &[
        "recommendationTrend",
        "upgradeDowngradeHistory",
        "financialData",
        "earningsHistory",
        "earningsTrend",
    ];

    let json = get_analysts_raw(symbol, modules).await?;

    Ok(AnalystData {
        symbol: symbol.to_string(),
        recommendations: parse_recommendations(&json),
        upgrades_downgrades: parse_upgrades_downgrades(&json),
        price_target: parse_price_targets(&json),
        earnings_estimates: parse_earnings_estimates(&json),
        revenue_estimates: parse_revenue_estimates(&json),
        earnings_history: parse_earnings_history(&json),
        eps_trend: parse_eps_trend(&json),
        eps_revisions: parse_eps_revisions(&json),
        growth_estimates: parse_growth_estimates(&json),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require network access and valid Yahoo Finance authentication
    // Run with: cargo test -p package-test -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_get_recommendations() {
        let result = get_recommendations("AAPL").await;
        assert!(result.is_ok());
        let recs = result.unwrap();
        assert!(!recs.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_upgrades_downgrades() {
        let result = get_upgrades_downgrades("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_price_targets() {
        let result = get_price_targets("AAPL").await;
        assert!(result.is_ok());
        let targets = result.unwrap();
        assert!(targets.target_mean.is_some());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_earnings_history() {
        let result = get_earnings_history("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_earnings_estimates() {
        let result = get_earnings_estimates("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_revenue_estimates() {
        let result = get_revenue_estimates("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_eps_trend() {
        let result = get_eps_trend("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_all_analyst_data() {
        let result = get_all_analyst_data("AAPL").await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.symbol, "AAPL");
    }

    #[test]
    fn test_recommendation_consensus() {
        let rec = RecommendationData {
            period: "0m".to_string(),
            strong_buy: 10,
            buy: 15,
            hold: 5,
            sell: 2,
            strong_sell: 1,
        };

        assert_eq!(rec.total_analysts(), 33);
        assert_eq!(rec.consensus(), "Strong Buy");
        assert!(rec.bullish_percent() > 70.0);
    }

    #[test]
    fn test_price_target_upside() {
        let target = PriceTarget {
            current_price: Some(100.0),
            target_mean: Some(120.0),
            target_median: Some(118.0),
            target_low: Some(90.0),
            target_high: Some(150.0),
            number_of_analysts: Some(30),
        };

        assert_eq!(target.upside_percent(), Some(20.0));
        assert_eq!(target.target_spread(), Some(60.0));
    }

    #[test]
    fn test_earnings_beat() {
        let item = EarningsHistoryItem {
            quarter: Some("Q1".to_string()),
            date: None,
            eps_actual: Some(1.50),
            eps_estimate: Some(1.40),
            surprise: Some(0.10),
            surprise_percent: Some(7.14),
        };

        assert_eq!(item.beat_estimate(), Some(true));
    }
}