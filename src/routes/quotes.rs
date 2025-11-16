use actix_web::{web, HttpResponse, Result};
use crate::client::{FetchClient, YahooFinanceClient};
use crate::models::{Quote, SimpleQuote};
use crate::service;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct QuotesQuery {
    symbols: String,
}

pub async fn get_quotes_handler(
    query: web::Query<QuotesQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbols: Vec<&str> = query.symbols.split(',').map(|s| s.trim()).collect();
    
    let quotes = service::get_quotes(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbols,
    )
    .await
    .map_err(|e| e.error_response())?;

    Ok(HttpResponse::Ok().json(quotes))
}

pub async fn get_simple_quotes_handler(
    query: web::Query<QuotesQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbols: Vec<&str> = query.symbols.split(',').map(|s| s.trim()).collect();
    
    let quotes = service::get_simple_quotes(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbols,
    )
    .await
    .map_err(|e| e.error_response())?;

    Ok(HttpResponse::Ok().json(quotes))
}

