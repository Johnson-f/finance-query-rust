use crate::error::IntoWebResult;
use crate::service::movers;
use actix_web::{HttpResponse, Result, web};
use finance_query_core::models::movers::MoverCount;
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
    let count = MoverCount::parse(&query.count).unwrap_or(MoverCount::Fifty);

    let movers_list = movers::get_actives(&app_state.yahoo_client, count)
        .await
        .into_web_result()?;

    Ok(HttpResponse::Ok().json(movers_list))
}

pub async fn get_gainers_handler(
    query: web::Query<MoversQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let count = MoverCount::parse(&query.count).unwrap_or(MoverCount::Fifty);

    let movers_list = movers::get_gainers(&app_state.yahoo_client, count)
        .await
        .into_web_result()?;

    Ok(HttpResponse::Ok().json(movers_list))
}

pub async fn get_losers_handler(
    query: web::Query<MoversQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let count = MoverCount::parse(&query.count).unwrap_or(MoverCount::Fifty);

    let movers_list = movers::get_losers(&app_state.yahoo_client, count)
        .await
        .into_web_result()?;

    Ok(HttpResponse::Ok().json(movers_list))
}
