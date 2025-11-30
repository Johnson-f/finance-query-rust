use actix_web::{web, HttpResponse, Result};
use finance_query_core::models::holders::{
    HolderType, InsiderPurchasesResponse, InsiderRosterResponse, InsiderTransactionsResponse,
    InstitutionalHoldersResponse, MajorHoldersResponse, MutualFundHoldersResponse,
};
use crate::error::IntoWebResult;
use crate::service::holders;
use crate::service::caching::{holders_key, TTL_HOLDERS};
use serde_json::Value;

pub async fn get_major_holders_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = holders_key(&symbol, "major");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::Major,
    )
    .await
    .into_web_result()?;
    
    let response = MajorHoldersResponse {
        symbol: data.symbol,
        breakdown: data.major_breakdown
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No major breakdown data"))?,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_HOLDERS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_institutional_holders_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = holders_key(&symbol, "institutional");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::Institutional,
    )
    .await
    .into_web_result()?;
    
    let response = InstitutionalHoldersResponse {
        symbol: data.symbol,
        holders: data.institutional_holders
            .unwrap_or_default(),
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_HOLDERS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_mutualfund_holders_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = holders_key(&symbol, "mutualfund");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::MutualFund,
    )
    .await
    .into_web_result()?;
    
    let response = MutualFundHoldersResponse {
        symbol: data.symbol,
        holders: data.mutualfund_holders
            .unwrap_or_default(),
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_HOLDERS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_insider_transactions_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = holders_key(&symbol, "insider_transactions");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::InsiderTransactions,
    )
    .await
    .into_web_result()?;
    
    let response = InsiderTransactionsResponse {
        symbol: data.symbol,
        transactions: data.insider_transactions
            .unwrap_or_default(),
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_HOLDERS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_insider_purchases_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = holders_key(&symbol, "insider_purchases");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::InsiderPurchases,
    )
    .await
    .into_web_result()?;
    
    let response = InsiderPurchasesResponse {
        symbol: data.symbol,
        summary: data.insider_purchases
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No insider purchases data"))?,
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_HOLDERS).await;
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_insider_roster_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = holders_key(&symbol, "insider_roster");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::InsiderRoster,
    )
    .await
    .into_web_result()?;
    
    let response = InsiderRosterResponse {
        symbol: data.symbol,
        roster: data.insider_roster
            .unwrap_or_default(),
    };
    
    // Cache the result
    let response_json: Value = serde_json::to_value(&response)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &response_json, TTL_HOLDERS).await;
    
    Ok(HttpResponse::Ok().json(response))
}