use actix_web::{web, HttpResponse, Result};
use crate::error::IntoWebResult;
use crate::service;
use finance_query_core::models::DetailedQuote;
use serde::Deserialize;

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
    .into_web_result()?;

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
    .into_web_result()?;

    Ok(HttpResponse::Ok().json(quotes))
}

pub async fn get_detailed_quotes_handler(
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
    .into_web_result()?;

    // Convert Quote to DetailedQuote (camelCase serialization)
    let detailed_quotes: Vec<DetailedQuote> = quotes.into_iter().map(DetailedQuote::from).collect();

    Ok(HttpResponse::Ok().json(detailed_quotes))
}

