use actix_web::{web, HttpResponse, Result};
use crate::models::holders::{
    HolderType, InsiderPurchasesResponse, InsiderRosterResponse, InsiderTransactionsResponse,
    InstitutionalHoldersResponse, MajorHoldersResponse, MutualFundHoldersResponse,
};
use crate::service::holders;

pub async fn get_major_holders_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::Major,
    )
    .await?;
    
    let response = MajorHoldersResponse {
        symbol: data.symbol,
        breakdown: data.major_breakdown
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No major breakdown data"))?,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_institutional_holders_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::Institutional,
    )
    .await?;
    
    let response = InstitutionalHoldersResponse {
        symbol: data.symbol,
        holders: data.institutional_holders
            .unwrap_or_default(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_mutualfund_holders_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::MutualFund,
    )
    .await?;
    
    let response = MutualFundHoldersResponse {
        symbol: data.symbol,
        holders: data.mutualfund_holders
            .unwrap_or_default(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_insider_transactions_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::InsiderTransactions,
    )
    .await?;
    
    let response = InsiderTransactionsResponse {
        symbol: data.symbol,
        transactions: data.insider_transactions
            .unwrap_or_default(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_insider_purchases_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::InsiderPurchases,
    )
    .await?;
    
    let response = InsiderPurchasesResponse {
        symbol: data.symbol,
        summary: data.insider_purchases
            .ok_or_else(|| actix_web::error::ErrorInternalServerError("No insider purchases data"))?,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_insider_roster_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let data = holders::get_holders_data(
        &app_state.yahoo_client,
        &symbol,
        HolderType::InsiderRoster,
    )
    .await?;
    
    let response = InsiderRosterResponse {
        symbol: data.symbol,
        roster: data.insider_roster
            .unwrap_or_default(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}