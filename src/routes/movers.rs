use actix_web::{web, HttpResponse, Result};
use crate::models::movers::MoverCount;
use crate::service::movers;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoversQuery {
    #[serde(default = "default_count")]
    count: String,
}

fn default_count() -> String {
    "50".to_string()
}

pub async fn get_actives_handler(
    query: web::Query<MoversQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let count = MoverCount::from_str(&query.count)
        .unwrap_or(MoverCount::Fifty);
    
    let movers_list = movers::get_actives(
        &app_state.yahoo_client,
        count,
    )
    .await?;

    Ok(HttpResponse::Ok().json(movers_list))
}

pub async fn get_gainers_handler(
    query: web::Query<MoversQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let count = MoverCount::from_str(&query.count)
        .unwrap_or(MoverCount::Fifty);
    
    let movers_list = movers::get_gainers(
        &app_state.yahoo_client,
        count,
    )
    .await?;

    Ok(HttpResponse::Ok().json(movers_list))
}

pub async fn get_losers_handler(
    query: web::Query<MoversQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let count = MoverCount::from_str(&query.count)
        .unwrap_or(MoverCount::Fifty);
    
    let movers_list = movers::get_losers(
        &app_state.yahoo_client,
        count,
    )
    .await?;

    Ok(HttpResponse::Ok().json(movers_list))
}