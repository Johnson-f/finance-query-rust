use actix_web::{web, HttpResponse, Result};
use crate::models::analysts::{
    AnalysisType, EarningsEstimateResponse, EarningsHistoryResponse, PriceTargetsResponse,
    RecommendationsResponse, RevenueEstimateResponse, UpgradesDowngradesResponse,
};
use crate::service::analysts;

pub async fn get_recommendations_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::Recommendations,
    )
    .await?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let recommendations: Vec<crate::models::analysts::RecommendationData> = serde_json::from_value(
        data.get("recommendations")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No recommendations data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse recommendations: {}", e)))?;
    
    let response = RecommendationsResponse {
        symbol: symbol_str,
        recommendations,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_upgrades_downgrades_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::UpgradesDowngrades,
    )
    .await?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let upgrades_downgrades: Vec<crate::models::analysts::UpgradeDowngrade> = serde_json::from_value(
        data.get("upgrades_downgrades")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No upgrades_downgrades data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse upgrades_downgrades: {}", e)))?;
    
    let response = UpgradesDowngradesResponse {
        symbol: symbol_str,
        upgrades_downgrades,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_price_targets_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::PriceTargets,
    )
    .await?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let price_targets: crate::models::analysts::PriceTarget = serde_json::from_value(
        data.get("price_targets")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No price_targets data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse price_targets: {}", e)))?;
    
    let response = PriceTargetsResponse {
        symbol: symbol_str,
        price_targets,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_earnings_estimate_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::EarningsEstimate,
    )
    .await?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let earnings_estimate: crate::models::analysts::EarningsEstimate = serde_json::from_value(
        data.get("earnings_estimate")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No earnings_estimate data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse earnings_estimate: {}", e)))?;
    
    let response = EarningsEstimateResponse {
        symbol: symbol_str,
        earnings_estimate,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_revenue_estimate_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::RevenueEstimate,
    )
    .await?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let revenue_estimate: crate::models::analysts::RevenueEstimate = serde_json::from_value(
        data.get("revenue_estimate")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No revenue_estimate data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse revenue_estimate: {}", e)))?;
    
    let response = RevenueEstimateResponse {
        symbol: symbol_str,
        revenue_estimate,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_earnings_history_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::EarningsHistory,
    )
    .await?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let earnings_history: Vec<crate::models::analysts::EarningsHistoryItem> = serde_json::from_value(
        data.get("earnings_history")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No earnings_history data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse earnings_history: {}", e)))?;
    
    let response = EarningsHistoryResponse {
        symbol: symbol_str,
        earnings_history,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

