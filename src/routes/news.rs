use actix_web::{web, HttpResponse, Result};
use crate::service;

pub async fn get_news_for_symbol_handler(
    path: web::Path<String>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let news = service::scrape_news_for_quote(
        &app_state.fetch_client,
        &symbol,
    )
    .await
    .map_err(|e| e.error_response())?;

    Ok(HttpResponse::Ok().json(news))
}

pub async fn get_general_news_handler(
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let news = service::scrape_general_news(
        &app_state.fetch_client,
    )
    .await
    .map_err(|e| e.error_response())?;

    Ok(HttpResponse::Ok().json(news))
}

