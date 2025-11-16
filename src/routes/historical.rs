use actix_web::{web, HttpResponse, Result};
use crate::models::{TimeRange, Interval};
use crate::service;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HistoricalQuery {
    range: String,
    interval: String,
}

pub async fn get_historical_handler(
    path: web::Path<String>,
    query: web::Query<HistoricalQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let time_range = parse_time_range(&query.range)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid time range"))?;
    let interval = parse_interval(&query.interval)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid interval"))?;

    let historical = service::get_historical(
        &app_state.yahoo_client,
        &symbol,
        time_range,
        interval,
    )
    .await?;

    Ok(HttpResponse::Ok().json(historical))
}

fn parse_time_range(s: &str) -> Result<TimeRange, ()> {
    match s {
        "1d" => Ok(TimeRange::Day),
        "5d" => Ok(TimeRange::FiveDays),
        "1mo" => Ok(TimeRange::OneMonth),
        "3mo" => Ok(TimeRange::ThreeMonths),
        "6mo" => Ok(TimeRange::SixMonths),
        "1y" => Ok(TimeRange::Year),
        "2y" => Ok(TimeRange::TwoYears),
        "5y" => Ok(TimeRange::FiveYears),
        "10y" => Ok(TimeRange::TenYears),
        "ytd" => Ok(TimeRange::Ytd),
        "max" => Ok(TimeRange::Max),
        _ => Err(()),
    }
}

fn parse_interval(s: &str) -> Result<Interval, ()> {
    match s {
        "1m" => Ok(Interval::OneMinute),
        "5m" => Ok(Interval::FiveMinutes),
        "15m" => Ok(Interval::FifteenMinutes),
        "30m" => Ok(Interval::ThirtyMinutes),
        "1h" => Ok(Interval::OneHour),
        "1d" => Ok(Interval::Daily),
        "1wk" => Ok(Interval::Weekly),
        "1mo" => Ok(Interval::Monthly),
        _ => Err(()),
    }
}

