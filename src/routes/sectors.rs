use actix_web::{web, HttpResponse, Result};
use crate::models::sectors::{MarketSector, MarketSectorDetails, Sector};
use crate::service;
use std::str::FromStr;

pub async fn get_sectors_handler(
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let sectors = service::get_sectors(&app_state.fetch_client).await?;
    Ok(HttpResponse::Ok().json(sectors))
}

pub async fn get_sector_for_symbol_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    let sector = service::get_sector_for_symbol(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        &symbol,
    )
    .await?;
    Ok(HttpResponse::Ok().json(sector))
}

pub async fn get_sector_details_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let sector_str = path.into_inner();
    let sector = Sector::from_str(&sector_str)
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Invalid sector: {}", e)))?;
    
    let details = service::get_sector_details(&app_state.fetch_client, sector).await?;
    Ok(HttpResponse::Ok().json(details))
}

