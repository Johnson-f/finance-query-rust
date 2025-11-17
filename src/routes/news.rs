use actix_web::{web, HttpResponse, Result};
use crate::service;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewsQuery {
    symbol: Option<String>,
}

pub async fn get_news_handler(
    query: web::Query<NewsQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
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

    Ok(HttpResponse::Ok().json(news))
}