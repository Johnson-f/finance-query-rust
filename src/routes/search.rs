use crate::error::IntoWebResult;
use crate::service;
use actix_web::{HttpResponse, Result, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
    #[serde(default = "default_hits")]
    hits: usize,
}

fn default_hits() -> usize {
    6
}

pub async fn search_handler(
    query: web::Query<SearchQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let results = service::search(&app_state.yahoo_client, &query.q, query.hits)
        .await
        .into_web_result()?;

    Ok(HttpResponse::Ok().json(results))
}
