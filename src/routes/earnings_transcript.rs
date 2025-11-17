use actix_web::{web, HttpResponse, Result};
use crate::service;

pub async fn get_earnings_calls_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let calls = service::get_earnings_calls_list(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbol,
    )
    .await?;

    Ok(HttpResponse::Ok().json(calls))
}

pub async fn get_earnings_transcript_handler(
    path: web::Path<(String, String)>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let (_symbol, event_id) = path.into_inner();
    
    // First, we need to get the company_id (quartrId) for the symbol
    // This is a simplified version - you may need to fetch quote_type first
    let company_id = "unknown"; // TODO: Fetch from get_quote_type
    
    let transcript = service::get_earnings_transcript(
        &app_state.yahoo_client,
        &event_id,
        company_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(transcript))
}