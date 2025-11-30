use actix_web::{web, HttpResponse, Result};
use finance_query_core::models::indices::{Index, Region};
use crate::error::IntoWebResult;
use crate::service::indices;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IndicesQuery {
    #[serde(default)]
    index: Option<String>,
    #[serde(default)]
    region: Option<String>,
}

fn parse_index(s: &str) -> Option<Index> {
    Index::from_str(s)
}

fn parse_region(s: &str) -> Option<Region> {
    match s.to_uppercase().as_str() {
        "US" => Some(Region::UnitedStates),
        "NA" => Some(Region::NorthAmerica),
        "SA" => Some(Region::SouthAmerica),
        "EU" => Some(Region::Europe),
        "AS" => Some(Region::Asia),
        "AF" => Some(Region::Africa),
        "ME" => Some(Region::MiddleEast),
        "OCE" => Some(Region::Oceania),
        "GLOBAL" => Some(Region::Global),
        _ => None,
    }
}

pub async fn get_indices_handler(
    query: web::Query<IndicesQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    // Parse indices from query parameter
    let indices: Option<Vec<Index>> = if let Some(index_str) = &query.index {
        let parsed: Vec<Index> = index_str
            .split(',')
            .map(|s| s.trim())
            .filter_map(parse_index)
            .collect();
        if parsed.is_empty() {
            None
        } else {
            Some(parsed)
        }
    } else {
        None
    };
    
    // Parse region from query parameter
    let region: Option<Region> = query.region.as_ref().and_then(|r| parse_region(r));
    
    let indices_list = indices::get_indices(
        &app_state.yahoo_client,
        &app_state.fetch_client,
        indices,
        region,
    )
    .await
    .into_web_result()?;

    Ok(HttpResponse::Ok().json(indices_list))
}