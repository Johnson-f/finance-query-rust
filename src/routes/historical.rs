use actix_web::{web, HttpResponse, Result};
use crate::models::{TimeRange, Interval};
use crate::models::historical::IndicatorType;
use crate::service;
use crate::service::historical::calculate_indicators;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HistoricalQuery {
    range: String,
    interval: String,
    #[serde(default)]
    indicators: Option<String>,
    #[serde(default)]
    period: Option<String>,
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

    // Validate minute intervals can only be used with 1d or 5d ranges
    validate_interval_range_compatibility(&interval, &time_range)?;

    let mut historical = service::get_historical(
        &app_state.yahoo_client,
        &symbol,
        time_range,
        interval,
    )
    .await?;

    // Calculate indicators if requested
    if let Some(indicators_str) = &query.indicators {
        let requested_indicators = IndicatorType::parse_list(indicators_str);
        
        if requested_indicators.is_empty() {
            return Err(actix_web::error::ErrorBadRequest("Invalid indicator type. Supported: sma, ema"));
        }
        
        // Parse periods (comma-separated, e.g., "10,20,50")
        let periods_str = query.period.as_deref().unwrap_or("20");
        let periods = parse_periods(periods_str)?;
        
        if periods.is_empty() {
            return Err(actix_web::error::ErrorBadRequest("At least one period must be specified"));
        }
        
        historical = calculate_indicators(historical, &periods, &requested_indicators);
    }

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
        "3m" => Ok(Interval::ThreeMinutes),
        "5m" => Ok(Interval::FiveMinutes),
        "10m" => Ok(Interval::TenMinutes),
        "15m" => Ok(Interval::FifteenMinutes),
        "20m" => Ok(Interval::TwentyMinutes),
        "30m" => Ok(Interval::ThirtyMinutes),
        "65m" => Ok(Interval::SixtyFiveMinutes),
        "95m" => Ok(Interval::NinetyFiveMinutes),
        "1h" => Ok(Interval::OneHour),
        "1d" => Ok(Interval::Daily),
        "1wk" => Ok(Interval::Weekly),
        "1mo" => Ok(Interval::Monthly),
        _ => Err(()),
    }
}

/// Validates that minute intervals (1m, 3m, 5m, 10m, 15m, 20m, 30m, 65m) 
/// can only be used with 1d or 5d ranges
fn validate_interval_range_compatibility(
    interval: &Interval,
    time_range: &TimeRange,
) -> Result<(), actix_web::Error> {
    // List of minute intervals that are restricted to 1d and 5d only
    let restricted_minute_intervals = matches!(
        interval,
        Interval::OneMinute
            | Interval::ThreeMinutes
            | Interval::FiveMinutes
            | Interval::TenMinutes
            | Interval::FifteenMinutes
            | Interval::TwentyMinutes
            | Interval::ThirtyMinutes
            | Interval::SixtyFiveMinutes
    );

    if restricted_minute_intervals {
        let allowed_ranges = matches!(time_range, TimeRange::Day | TimeRange::FiveDays);
        
        if !allowed_ranges {
            return Err(actix_web::error::ErrorBadRequest(format!(
                "The interval '{}' can only be used with ranges '1d' or '5d'. Please use one of these ranges or choose a different interval.",
                interval.as_str()
            )));
        }
    }

    Ok(())
}

/// Parse comma-separated periods string into a vector of periods
/// Example: "10,20,50" -> [10, 20, 50]
fn parse_periods(periods_str: &str) -> Result<Vec<usize>, actix_web::Error> {
    let periods: Result<Vec<usize>, _> = periods_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<usize>()
                .map_err(|_| actix_web::error::ErrorBadRequest(format!("Invalid period value: '{}'. Periods must be positive integers.", s)))
        })
        .collect();
    
    let periods = periods?;
    
    // Validate all periods are greater than 0
    for period in &periods {
        if *period == 0 {
            return Err(actix_web::error::ErrorBadRequest("Period must be greater than 0"));
        }
    }
    
    if periods.is_empty() {
        return Err(actix_web::error::ErrorBadRequest("At least one period must be specified"));
    }
    
    Ok(periods)
}

