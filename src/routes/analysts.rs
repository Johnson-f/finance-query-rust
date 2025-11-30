use actix_web::{web, HttpResponse, Result};
use finance_query_core::models::analysts::{
    AnalysisType, EarningsEstimate, EarningsEstimateResponse, EarningsHistoryItem, EarningsHistoryResponse, 
    PriceTarget, PriceTargetsResponse, RecommendationData, RecommendationsResponse, 
    RevenueEstimate, RevenueEstimateResponse, UpgradeDowngrade, UpgradesDowngradesResponse,
};
use crate::error::IntoWebResult;
use crate::service::analysts;
use crate::service::caching::{analysts_key, TTL_ANALYSTS};
use serde_json::Value;

pub async fn get_recommendations_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = analysts_key(&symbol, "recommendations");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::Recommendations,
    )
    .await
    .into_web_result()?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let recommendations: Vec<RecommendationData> = serde_json::from_value(
        data.get("recommendations")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No recommendations data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse recommendations: {}", e)))?;
    
    let response = RecommendationsResponse {
        symbol: symbol_str,
        recommendations,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_ANALYSTS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_upgrades_downgrades_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = analysts_key(&symbol, "upgrades_downgrades");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::UpgradesDowngrades,
    )
    .await
    .into_web_result()?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let upgrades_downgrades: Vec<UpgradeDowngrade> = serde_json::from_value(
        data.get("upgrades_downgrades")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No upgrades_downgrades data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse upgrades_downgrades: {}", e)))?;
    
    let response = UpgradesDowngradesResponse {
        symbol: symbol_str,
        upgrades_downgrades,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_ANALYSTS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_price_targets_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = analysts_key(&symbol, "price_targets");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::PriceTargets,
    )
    .await
    .into_web_result()?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let price_targets: PriceTarget = serde_json::from_value(
        data.get("price_targets")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No price_targets data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse price_targets: {}", e)))?;
    
    let response = PriceTargetsResponse {
        symbol: symbol_str,
        price_targets,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_ANALYSTS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_earnings_estimate_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = analysts_key(&symbol, "earnings_estimate");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::EarningsEstimate,
    )
    .await
    .into_web_result()?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let earnings_estimate: EarningsEstimate = serde_json::from_value(
        data.get("earnings_estimate")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No earnings_estimate data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse earnings_estimate: {}", e)))?;
    
    let response = EarningsEstimateResponse {
        symbol: symbol_str,
        earnings_estimate,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_ANALYSTS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_revenue_estimate_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = analysts_key(&symbol, "revenue_estimate");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::RevenueEstimate,
    )
    .await
    .into_web_result()?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let revenue_estimate: RevenueEstimate = serde_json::from_value(
        data.get("revenue_estimate")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No revenue_estimate data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse revenue_estimate: {}", e)))?;
    
    let response = RevenueEstimateResponse {
        symbol: symbol_str,
        revenue_estimate,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_ANALYSTS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_earnings_history_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = analysts_key(&symbol, "earnings_history");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = analysts::get_analysis_data(
        &app_state.yahoo_client,
        &symbol,
        AnalysisType::EarningsHistory,
    )
    .await
    .into_web_result()?;
    
    let symbol_str = data.get("symbol")
        .and_then(|s| s.as_str())
        .unwrap_or(&symbol)
        .to_string();
    
    let earnings_history: Vec<EarningsHistoryItem> = serde_json::from_value(
        data.get("earnings_history")
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No earnings_history data"))?
            .clone()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to parse earnings_history: {}", e)))?;
    
    let response = EarningsHistoryResponse {
        symbol: symbol_str,
        earnings_history,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_ANALYSTS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

