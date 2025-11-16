use actix_web::{web, HttpResponse, Result};
use crate::service;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimilarQuotesQuery {
    symbol: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    10
}

pub async fn get_similar_quotes_handler(
    query: web::Query<SimilarQuotesQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    // Validate limit (1-20, matching Python implementation)
    let limit = query.limit.clamp(1, 20);
    let symbol = query.symbol.trim().to_uppercase();
    
    if symbol.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "detail": "Symbol parameter is required"
        })));
    }
    
    let quotes = service::get_similar_quotes(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbol,
        limit,
    )
    .await?;

    if quotes.is_empty() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "detail": format!("No similar stocks found or invalid symbol: {}", symbol)
        })));
    }

    Ok(HttpResponse::Ok().json(quotes))
}

