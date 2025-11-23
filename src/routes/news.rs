use actix_web::{web, HttpResponse, Result};
use crate::service;
use crate::service::caching::{news_key, TTL_NEWS};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct NewsQuery {
    symbol: Option<String>,
}

pub async fn get_news_handler(
    query: web::Query<NewsQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let cache_key = news_key(query.symbol.as_deref());
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    // Cache miss - fetch from API
    let news = if let Some(symbol) = &query.symbol {
        service::scrape_news_for_quote(
            &app_state.fetch_client,
            symbol,
        )
        .await?
    } else {
        service::scrape_general_news(
            &app_state.fetch_client,
        )
        .await?
    };

    // Cache the result
    let news_json: Value = serde_json::to_value(&news)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &news_json, TTL_NEWS).await;

    Ok(HttpResponse::Ok().json(news))
}

/// Handler for /v1/news/{symbol} - accepts symbol as path parameter
pub async fn get_news_by_symbol_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let cache_key = news_key(Some(&symbol));
    
    // Check cache first
    if let Some(cached) = app_state.cache_service.get::<Value>(&cache_key).await {
        return Ok(HttpResponse::Ok().json(cached));
    }
    
    // Cache miss - fetch from API
    let news = service::scrape_news_for_quote(
        &app_state.fetch_client,
        &symbol,
    )
    .await?;

    // Cache the result
    let news_json: Value = serde_json::to_value(&news)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to serialize response: {}", e)))?;
    app_state.cache_service.set(&cache_key, &news_json, TTL_NEWS).await;

    Ok(HttpResponse::Ok().json(news))
}