use finance_query_core::client::{error::YahooError, YahooFinanceClient};
use finance_query_core::models::analysts::{
    AnalysisType, EarningsEstimate, EarningsHistoryItem, PriceTarget, RecommendationData,
    RevenueEstimate, UpgradeDowngrade,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use tracing::info;

// Module mapping for Yahoo Finance quoteSummary API
fn get_modules_for_analysis_type(analysis_type: AnalysisType) -> Vec<&'static str> {
    match analysis_type {
        AnalysisType::Recommendations => vec!["recommendationTrend"],
        AnalysisType::UpgradesDowngrades => vec!["upgradeDowngradeHistory"],
        AnalysisType::PriceTargets => vec!["financialData"],
        AnalysisType::EarningsEstimate => vec!["earningsTrend"],
        AnalysisType::RevenueEstimate => vec!["earningsTrend"],
        AnalysisType::EarningsHistory => vec!["earningsHistory"],
    }
}

/// Get analysis data for a symbol
pub async fn get_analysis_data(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
    analysis_type: AnalysisType,
) -> Result<HashMap<String, serde_json::Value>, YahooError> {
    info!("Fetching {} analysis data for {}", analysis_type.as_str(), symbol);
    
    let modules = get_modules_for_analysis_type(analysis_type);
    let modules_refs: Vec<&str> = modules.to_vec();
    
    // Fetch data from Yahoo Finance
    let response = yahoo_client
        .get_quote_summary(&symbol.to_uppercase(), &modules_refs)
        .await?;
    
    // Extract the result
    let result = response
        .get("quoteSummary")
        .and_then(|qs| qs.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .ok_or_else(|| {
            YahooError::ParseError(format!(
                "No {} data found for {}",
                analysis_type.as_str(),
                symbol
            ))
        })?;
    
    // Parse based on analysis type
    let parsed_data = match analysis_type {
        AnalysisType::Recommendations => {
            let recommendations = parse_recommendations(result)?;
            serde_json::to_value(recommendations).map_err(|e| {
                YahooError::ParseError(format!("Failed to serialize recommendations: {}", e))
            })?
        }
        AnalysisType::UpgradesDowngrades => {
            let upgrades_downgrades = parse_upgrades_downgrades(result)?;
            serde_json::to_value(upgrades_downgrades).map_err(|e| {
                YahooError::ParseError(format!("Failed to serialize upgrades_downgrades: {}", e))
            })?
        }
        AnalysisType::PriceTargets => {
            let price_targets = parse_price_targets(result)?;
            serde_json::to_value(price_targets).map_err(|e| {
                YahooError::ParseError(format!("Failed to serialize price_targets: {}", e))
            })?
        }
        AnalysisType::EarningsEstimate => {
            let earnings_estimate = parse_earnings_estimate(result)?;
            serde_json::to_value(earnings_estimate).map_err(|e| {
                YahooError::ParseError(format!("Failed to serialize earnings_estimate: {}", e))
            })?
        }
        AnalysisType::RevenueEstimate => {
            let revenue_estimate = parse_revenue_estimate(result)?;
            serde_json::to_value(revenue_estimate).map_err(|e| {
                YahooError::ParseError(format!("Failed to serialize revenue_estimate: {}", e))
            })?
        }
        AnalysisType::EarningsHistory => {
            let earnings_history = parse_earnings_history(result)?;
            serde_json::to_value(earnings_history).map_err(|e| {
                YahooError::ParseError(format!("Failed to serialize earnings_history: {}", e))
            })?
        }
    };
    
    let field_name = match analysis_type {
        AnalysisType::Recommendations => "recommendations",
        AnalysisType::UpgradesDowngrades => "upgrades_downgrades",
        AnalysisType::PriceTargets => "price_targets",
        AnalysisType::EarningsEstimate => "earnings_estimate",
        AnalysisType::RevenueEstimate => "revenue_estimate",
        AnalysisType::EarningsHistory => "earnings_history",
    };
    
    let mut result_map = HashMap::new();
    result_map.insert("symbol".to_string(), serde_json::json!(symbol.to_uppercase()));
    result_map.insert(field_name.to_string(), parsed_data);
    
    info!("Successfully parsed {} analysis data for {}", analysis_type.as_str(), symbol);
    Ok(result_map)
}

fn parse_recommendations(data: &Value) -> Result<Vec<RecommendationData>, YahooError> {
    let empty_vec = Vec::new();
    let trend_list = data
        .get("recommendationTrend")
        .and_then(|rt| rt.get("trend"))
        .and_then(|t| t.as_array())
        .unwrap_or(&empty_vec);
    
    let mut recommendations = Vec::new();
    
    for trend_data in trend_list {
        let recommendation = RecommendationData {
            period: trend_data
                .get("period")
                .and_then(|p| p.as_str())
                .unwrap_or("")
                .to_string(),
            strong_buy: trend_data.get("strongBuy").and_then(|v| v.as_i64().map(|i| i as i32)),
            buy: trend_data.get("buy").and_then(|v| v.as_i64().map(|i| i as i32)),
            hold: trend_data.get("hold").and_then(|v| v.as_i64().map(|i| i as i32)),
            sell: trend_data.get("sell").and_then(|v| v.as_i64().map(|i| i as i32)),
            strong_sell: trend_data.get("strongSell").and_then(|v| v.as_i64().map(|i| i as i32)),
        };
        recommendations.push(recommendation);
    }
    
    Ok(recommendations)
}

fn parse_upgrades_downgrades(data: &Value) -> Result<Vec<UpgradeDowngrade>, YahooError> {
    let empty_vec = Vec::new();
    let history_list = data
        .get("upgradeDowngradeHistory")
        .and_then(|udh| udh.get("history"))
        .and_then(|h| h.as_array())
        .unwrap_or(&empty_vec);
    
    let mut upgrades_downgrades = Vec::new();
    
    for item in history_list {
        let epoch_time = item.get("epochGradeDate").and_then(|e| e.as_i64());
        let grade_date = epoch_time.and_then(|ts| DateTime::from_timestamp(ts, 0));
        
        let upgrade_downgrade = UpgradeDowngrade {
            firm: item
                .get("firm")
                .and_then(|f| f.as_str())
                .unwrap_or("")
                .to_string(),
            to_grade: item.get("toGrade").and_then(|tg| tg.as_str()).map(|s| s.to_string()),
            from_grade: item.get("fromGrade").and_then(|fg| fg.as_str()).map(|s| s.to_string()),
            action: item.get("action").and_then(|a| a.as_str()).map(|s| s.to_string()),
            date: grade_date,
        };
        upgrades_downgrades.push(upgrade_downgrade);
    }
    
    Ok(upgrades_downgrades)
}

fn safe_extract_value(value: Option<&Value>) -> Option<f64> {
    value.and_then(|v| {
        if let Some(obj) = v.as_object() {
            obj.get("raw").and_then(|r| r.as_f64())
        } else if let Some(num) = v.as_f64() {
            Some(num)
        } else {
            v.as_i64().map(|num| num as f64)
        }
    })
}

fn parse_price_targets(data: &Value) -> Result<PriceTarget, YahooError> {
    let financial_data = data.get("financialData").unwrap_or(&Value::Null);
    
    Ok(PriceTarget {
        current: safe_extract_value(financial_data.get("currentPrice")),
        mean: safe_extract_value(financial_data.get("targetMeanPrice")),
        median: safe_extract_value(financial_data.get("targetMedianPrice")),
        low: safe_extract_value(financial_data.get("targetLowPrice")),
        high: safe_extract_value(financial_data.get("targetHighPrice")),
    })
}

fn parse_earnings_estimate(data: &Value) -> Result<EarningsEstimate, YahooError> {
    let empty_vec = Vec::new();
    let trend_list = data
        .get("earningsTrend")
        .and_then(|et| et.get("trend"))
        .and_then(|t| t.as_array())
        .unwrap_or(&empty_vec);
    
    let mut estimates_dict = HashMap::new();
    
    for trend_data in trend_list {
        let period = trend_data
            .get("period")
            .and_then(|p| p.as_str())
            .unwrap_or("")
            .to_string();
        
        let earnings_estimate = trend_data.get("earningsEstimate").unwrap_or(&Value::Null);
        
        let mut estimate_data = HashMap::new();
        
        if let Some(avg) = safe_extract_value(earnings_estimate.get("avg")) {
            estimate_data.insert("avg".to_string(), serde_json::json!(avg));
        }
        if let Some(low) = safe_extract_value(earnings_estimate.get("low")) {
            estimate_data.insert("low".to_string(), serde_json::json!(low));
        }
        if let Some(high) = safe_extract_value(earnings_estimate.get("high")) {
            estimate_data.insert("high".to_string(), serde_json::json!(high));
        }
        if let Some(num_analysts) = earnings_estimate
            .get("numberOfAnalysts")
            .and_then(|noa| noa.get("raw"))
            .and_then(|r| r.as_i64())
        {
            estimate_data.insert("numberOfAnalysts".to_string(), serde_json::json!(num_analysts));
        }
        if let Some(year_ago) = safe_extract_value(earnings_estimate.get("yearAgoEps")) {
            estimate_data.insert("yearAgoEps".to_string(), serde_json::json!(year_ago));
        }
        if let Some(growth) = safe_extract_value(earnings_estimate.get("growth")) {
            estimate_data.insert("growth".to_string(), serde_json::json!(growth));
        }
        
        let mut json_map = serde_json::Map::new();
        for (k, v) in estimate_data {
            json_map.insert(k, v);
        }
        estimates_dict.insert(period, serde_json::Value::Object(json_map));
    }
    
    Ok(EarningsEstimate {
        estimates: estimates_dict,
    })
}

fn parse_revenue_estimate(data: &Value) -> Result<RevenueEstimate, YahooError> {
    let empty_vec = Vec::new();
    let trend_list = data
        .get("earningsTrend")
        .and_then(|et| et.get("trend"))
        .and_then(|t| t.as_array())
        .unwrap_or(&empty_vec);
    
    let mut estimates_dict = HashMap::new();
    
    for trend_data in trend_list {
        let period = trend_data
            .get("period")
            .and_then(|p| p.as_str())
            .unwrap_or("")
            .to_string();
        
        let revenue_estimate = trend_data.get("revenueEstimate").unwrap_or(&Value::Null);
        
        let mut estimate_data = HashMap::new();
        
        if let Some(avg) = safe_extract_value(revenue_estimate.get("avg")) {
            estimate_data.insert("avg".to_string(), serde_json::json!(avg));
        }
        if let Some(low) = safe_extract_value(revenue_estimate.get("low")) {
            estimate_data.insert("low".to_string(), serde_json::json!(low));
        }
        if let Some(high) = safe_extract_value(revenue_estimate.get("high")) {
            estimate_data.insert("high".to_string(), serde_json::json!(high));
        }
        if let Some(num_analysts) = revenue_estimate
            .get("numberOfAnalysts")
            .and_then(|noa| noa.get("raw"))
            .and_then(|r| r.as_i64())
        {
            estimate_data.insert("numberOfAnalysts".to_string(), serde_json::json!(num_analysts));
        }
        if let Some(year_ago) = safe_extract_value(revenue_estimate.get("yearAgoRevenue")) {
            estimate_data.insert("yearAgoRevenue".to_string(), serde_json::json!(year_ago));
        }
        if let Some(growth) = safe_extract_value(revenue_estimate.get("growth")) {
            estimate_data.insert("growth".to_string(), serde_json::json!(growth));
        }
        
        let mut json_map = serde_json::Map::new();
        for (k, v) in estimate_data {
            json_map.insert(k, v);
        }
        estimates_dict.insert(period, serde_json::Value::Object(json_map));
    }
    
    Ok(RevenueEstimate {
        estimates: estimates_dict,
    })
}

fn parse_earnings_history(data: &Value) -> Result<Vec<EarningsHistoryItem>, YahooError> {
    let empty_vec = Vec::new();
    let history_list = data
        .get("earningsHistory")
        .and_then(|eh| eh.get("history"))
        .and_then(|h| h.as_array())
        .unwrap_or(&empty_vec);
    
    let mut earnings_history = Vec::new();
    
    for item in history_list {
        let quarter = item
            .get("quarter")
            .and_then(|q| q.get("raw"))
            .and_then(|r| r.as_i64())
            .and_then(|ts| DateTime::from_timestamp(ts, 0))
            .unwrap_or_else(Utc::now);
        
        let earnings_item = EarningsHistoryItem {
            date: quarter,
            eps_actual: safe_extract_value(item.get("epsActual")),
            eps_estimate: safe_extract_value(item.get("epsEstimate")),
            surprise: safe_extract_value(item.get("epsDifference")),
            surprise_percent: safe_extract_value(item.get("surprisePercent")),
        };
        
        earnings_history.push(earnings_item);
    }
    
    Ok(earnings_history)
}

