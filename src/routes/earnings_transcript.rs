use actix_web::{web, HttpResponse, Result};
use crate::service;
use serde::Deserialize;

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
    
    let calls = service::get_earnings_calls_list(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbol,
    )
    .await?;

    Ok(HttpResponse::Ok().json(calls))
}

pub async fn get_earnings_transcript_handler(
    path: web::Path<String>,
    query: web::Query<EarningsTranscriptQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let transcript = service::get_earnings_transcript(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbol,
        query.quarter.clone(),
        query.year,
    )
    .await?;

    Ok(HttpResponse::Ok().json(transcript))
}