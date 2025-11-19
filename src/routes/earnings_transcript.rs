use actix_web::{web, HttpResponse, Result};
use crate::service;
use crate::service::caching::{earnings_transcript_key, TTL_EARNINGS_TRANSCRIPT};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct EarningsTranscriptQuery {
    #[serde(default)]
    quarter: Option<String>,
    #[serde(default)]
    year: Option<i32>,
}

pub async fn get_earnings_calls_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = earnings_transcript_key(&symbol, "calls");
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    // Cache miss - fetch from API
    let calls = service::get_earnings_calls_list(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbol,
    )
    .await?;

    // Cache the result
    let calls_json: Value = serde_json::to_value(&calls)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &calls_json, TTL_EARNINGS_TRANSCRIPT).await;

    Ok(HttpResponse::Ok().json(calls))
}

pub async fn get_earnings_transcript_handler(
    path: web::Path<String>,
    query: web::Query<EarningsTranscriptQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let transcript_type = format!("transcript:{}:{}", 
        query.quarter.as_deref().unwrap_or("latest"),
        query.year.map(|y| y.to_string()).unwrap_or_else(|| "latest".to_string())
    );
    let cache_key = earnings_transcript_key(&symbol, &transcript_type);
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    // Cache miss - fetch from API
    let transcript = service::get_earnings_transcript(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbol,
        query.quarter.clone(),
        query.year,
    )
    .await?;

    // Cache the result
    let transcript_json: Value = serde_json::to_value(&transcript)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &transcript_json, TTL_EARNINGS_TRANSCRIPT).await;

    Ok(HttpResponse::Ok().json(transcript))
}