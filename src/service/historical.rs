use crate::service::websocket::indicator::moving_average::{
    MovingAverageType, calculate_ma_series,
};
use crate::service::websocket::indicator::price_buffer::PricePoint;
use finance_query_core::client::YahooFinanceClient;
use finance_query_core::client::error::YahooError;
use finance_query_core::models::historical::IndicatorType;
use finance_query_core::models::{HistoricalData, HistoricalResponse, Interval, TimeRange};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn};

/// Intervals that Yahoo Finance natively supports
fn is_natively_supported(interval: &Interval) -> bool {
    matches!(
        interval,
        Interval::OneMinute
            | Interval::FiveMinutes
            | Interval::FifteenMinutes
            | Interval::ThirtyMinutes
            | Interval::OneHour
            | Interval::Daily
            | Interval::Weekly
            | Interval::Monthly
    )
}

/// Get the number of minutes for an interval (for resampling)
fn interval_minutes(interval: &Interval) -> Option<u32> {
    match interval {
        Interval::OneMinute => Some(1),
        Interval::ThreeMinutes => Some(3),
        Interval::FiveMinutes => Some(5),
        Interval::TenMinutes => Some(10),
        Interval::FifteenMinutes => Some(15),
        Interval::TwentyMinutes => Some(20),
        Interval::ThirtyMinutes => Some(30),
        Interval::SixtyFiveMinutes => Some(65),
        Interval::NinetyFiveMinutes => Some(95),
        Interval::OneHour => Some(60),
        _ => None, // Daily, Weekly, Monthly are not minute-based
    }
}

pub async fn get_historical(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
    time_range: TimeRange,
    interval: Interval,
) -> Result<HistoricalResponse, YahooError> {
    // Special case: max range with 1d interval requires chunking
    if matches!(time_range, TimeRange::Max) && matches!(interval, Interval::Daily) {
        return get_historical_max_daily(yahoo_client, symbol).await;
    }

    // Special case: max range with 1wk interval requires chunking
    if matches!(time_range, TimeRange::Max) && matches!(interval, Interval::Weekly) {
        return get_historical_max_weekly(yahoo_client, symbol).await;
    }

    // Check if interval is natively supported by Yahoo Finance
    if !is_natively_supported(&interval) {
        // Need to resample from 1m data
        if let Some(target_minutes) = interval_minutes(&interval) {
            warn!(
                "Interval '{}' is not natively supported by Yahoo Finance. Resampling from 1m data.",
                interval.as_str()
            );
            return get_historical_resampled(yahoo_client, symbol, time_range, target_minutes)
                .await;
        } else {
            // This shouldn't happen, but handle it gracefully
            return Err(YahooError::HttpError(
                400,
                format!("Unsupported interval: {}", interval.as_str()),
            ));
        }
    }

    // Native interval, fetch directly
    let data = yahoo_client
        .get_chart(symbol, interval.as_str(), time_range.as_str())
        .await?;

    parse_historical_data(data)
}

/// Fetches entire history with 1d interval by chunking into 10-year periods
/// First detects the stock's actual start date by fetching max range with 1mo interval
async fn get_historical_max_daily(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
) -> Result<HistoricalResponse, YahooError> {
    info!(
        "Fetching max range with 1d interval for {}, using optimized chunking strategy",
        symbol
    );

    // Step 1: Detect earliest available date by fetching max with 1mo interval
    let earliest_data = yahoo_client.get_chart(symbol, "1mo", "max").await?;

    let earliest_response = parse_historical_data(earliest_data)?;

    // Find the earliest timestamp (keys are now in RFC3339 format)
    let earliest_timestamp = earliest_response
        .data
        .keys()
        .filter_map(|k| {
            // Try parsing as RFC3339 first, then fallback to Unix timestamp string
            chrono::DateTime::parse_from_rfc3339(k)
                .map(|dt| dt.timestamp())
                .ok()
                .or_else(|| {
                    k.parse::<i64>()
                        .ok()
                        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
                        .map(|dt| dt.timestamp())
                })
        })
        .min()
        .unwrap_or_else(|| {
            // Fallback: use 1970-01-01 if we can't determine
            chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
                .unwrap()
                .timestamp()
        });

    info!(
        "Detected earliest data point for {}: {} ({})",
        symbol,
        earliest_timestamp,
        chrono::DateTime::from_timestamp(earliest_timestamp, 0)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "unknown".to_string())
    );

    // Step 2: Calculate chunks and fetch data
    let mut all_data = HashMap::new();
    let now = chrono::Utc::now().timestamp();

    // Chunk size: 10 years (in seconds)
    const CHUNK_SIZE: i64 = 10 * 365 * 24 * 60 * 60;

    let mut period1 = earliest_timestamp;
    let mut chunk_count = 0;

    loop {
        let period2 = (period1 + CHUNK_SIZE).min(now);

        debug!(
            "Fetching chunk {} for {}: {} to {} ({})",
            chunk_count + 1,
            symbol,
            period1,
            period2,
            chrono::DateTime::from_timestamp(period1, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "unknown".to_string())
        );

        // Fetch this chunk
        let chunk_data = yahoo_client
            .get_chart_with_periods(symbol, "1d", period1, period2)
            .await?;

        let chunk_response = parse_historical_data(chunk_data)?;

        // If this chunk is empty, we might have reached the end
        if chunk_response.data.is_empty() {
            debug!("Empty chunk received for {}, stopping", symbol);
            break;
        }

        // Merge into all_data (HashMap will automatically handle duplicates by timestamp)
        all_data.extend(chunk_response.data);

        chunk_count += 1;

        // If we've reached the current time, we're done
        if period2 >= now {
            break;
        }

        // Move to next chunk (add 1 second to avoid overlap)
        period1 = period2 + 1;
    }

    info!(
        "Successfully fetched {} chunks for {} (total {} data points)",
        chunk_count,
        symbol,
        all_data.len()
    );

    Ok(HistoricalResponse { data: all_data })
}

/// Fetches entire history with 1wk interval by chunking into 10-year periods
/// First detects the stock's actual start date by fetching max range with 1mo interval
async fn get_historical_max_weekly(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
) -> Result<HistoricalResponse, YahooError> {
    info!(
        "Fetching max range with 1wk interval for {}, using optimized chunking strategy",
        symbol
    );

    // Step 1: Detect earliest available date by fetching max with 1mo interval
    let earliest_data = yahoo_client.get_chart(symbol, "1mo", "max").await?;

    let earliest_response = parse_historical_data(earliest_data)?;

    // Find the earliest timestamp (keys are now in RFC3339 format)
    let earliest_timestamp = earliest_response
        .data
        .keys()
        .filter_map(|k| {
            // Try parsing as RFC3339 first, then fallback to Unix timestamp string
            chrono::DateTime::parse_from_rfc3339(k)
                .map(|dt| dt.timestamp())
                .ok()
                .or_else(|| {
                    k.parse::<i64>()
                        .ok()
                        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
                        .map(|dt| dt.timestamp())
                })
        })
        .min()
        .unwrap_or_else(|| {
            // Fallback: use 1970-01-01 if we can't determine
            chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
                .unwrap()
                .timestamp()
        });

    info!(
        "Detected earliest data point for {}: {} ({})",
        symbol,
        earliest_timestamp,
        chrono::DateTime::from_timestamp(earliest_timestamp, 0)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "unknown".to_string())
    );

    // Step 2: Calculate chunks and fetch data
    let mut all_data = HashMap::new();
    let now = chrono::Utc::now().timestamp();

    // Chunk size: 10 years (in seconds)
    const CHUNK_SIZE: i64 = 10 * 365 * 24 * 60 * 60;

    let mut period1 = earliest_timestamp;
    let mut chunk_count = 0;

    loop {
        let period2 = (period1 + CHUNK_SIZE).min(now);

        debug!(
            "Fetching chunk {} for {} (weekly): {} to {} ({})",
            chunk_count + 1,
            symbol,
            period1,
            period2,
            chrono::DateTime::from_timestamp(period1, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "unknown".to_string())
        );

        // Fetch this chunk with weekly interval
        let chunk_data = yahoo_client
            .get_chart_with_periods(symbol, "1wk", period1, period2)
            .await?;

        let chunk_response = parse_historical_data(chunk_data)?;

        // If this chunk is empty, we might have reached the end
        if chunk_response.data.is_empty() {
            debug!("Empty chunk received for {} (weekly), stopping", symbol);
            break;
        }

        // Merge into all_data (HashMap will automatically handle duplicates by timestamp)
        all_data.extend(chunk_response.data);

        chunk_count += 1;

        // If we've reached the current time, we're done
        if period2 >= now {
            break;
        }

        // Move to next chunk (add 1 second to avoid overlap)
        period1 = period2 + 1;
    }

    info!(
        "Successfully fetched {} chunks for {} (weekly, total {} data points)",
        chunk_count,
        symbol,
        all_data.len()
    );

    Ok(HistoricalResponse { data: all_data })
}

fn parse_historical_data(data: Value) -> Result<HistoricalResponse, YahooError> {
    let mut history_map = HashMap::new();

    if let Some(chart) = data.get("chart")
        && let Some(results) = chart.get("result").and_then(|r| r.as_array())
        && let Some(result) = results.first()
        && let Some(timestamps) = result.get("timestamp").and_then(|t| t.as_array())
        && let Some(indicators) = result.get("indicators")
        && let Some(quote) = indicators.get("quote").and_then(|q| q.as_array())
        && let Some(quote_data) = quote.first()
    {
        let empty_vec = vec![];
        let opens = quote_data
            .get("open")
            .and_then(|o| o.as_array())
            .unwrap_or(&empty_vec);
        let highs = quote_data
            .get("high")
            .and_then(|h| h.as_array())
            .unwrap_or(&empty_vec);
        let lows = quote_data
            .get("low")
            .and_then(|l| l.as_array())
            .unwrap_or(&empty_vec);
        let closes = quote_data
            .get("close")
            .and_then(|c| c.as_array())
            .unwrap_or(&empty_vec);
        let volumes = quote_data
            .get("volume")
            .and_then(|v| v.as_array())
            .unwrap_or(&empty_vec);

        let adj_closes = indicators
            .get("adjclose")
            .and_then(|a| a.as_array())
            .and_then(|a| a.first())
            .and_then(|a| a.get("adjclose").and_then(|ac| ac.as_array()));

        for (i, timestamp) in timestamps.iter().enumerate() {
            if let Some(ts) = timestamp.as_i64() {
                let open = opens.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let high = highs.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let low = lows.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let close = closes.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
                let volume = volumes.get(i).and_then(|v| v.as_i64()).unwrap_or(0);
                let adj_close = adj_closes.and_then(|ac| ac.get(i).and_then(|v| v.as_f64()));

                // Convert Unix timestamp to RFC3339 format
                let rfc3339_timestamp = chrono::DateTime::from_timestamp(ts, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_else(|| ts.to_string()); // Fallback to Unix string if conversion fails

                history_map.insert(
                    rfc3339_timestamp,
                    HistoricalData {
                        open,
                        high,
                        low,
                        close,
                        volume,
                        adj_close,
                        sma: None,
                        ema: None,
                    },
                );
            }
        }
    }

    Ok(HistoricalResponse { data: history_map })
}

/// Fetches 1m data and resamples it to the target interval (in minutes)
async fn get_historical_resampled(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
    time_range: TimeRange,
    target_minutes: u32,
) -> Result<HistoricalResponse, YahooError> {
    info!(
        "Resampling historical data for {}: fetching 1m data and aggregating to {}m intervals",
        symbol, target_minutes
    );

    // Fetch 1m data
    let data = yahoo_client
        .get_chart(symbol, "1m", time_range.as_str())
        .await?;

    let one_minute_data = parse_historical_data(data)?;

    if one_minute_data.data.is_empty() {
        return Ok(HistoricalResponse {
            data: HashMap::new(),
        });
    }

    // Convert to sorted vector of (timestamp, data) tuples
    let mut sorted_data: Vec<(i64, HistoricalData)> = one_minute_data
        .data
        .into_iter()
        .filter_map(|(ts_str, data)| {
            // Parse timestamp
            let timestamp = chrono::DateTime::parse_from_rfc3339(&ts_str)
                .map(|dt| dt.timestamp())
                .ok()
                .or_else(|| ts_str.parse::<i64>().ok())?;

            Some((timestamp, data))
        })
        .collect();

    // Sort by timestamp
    sorted_data.sort_by_key(|(ts, _)| *ts);

    // Resample: group by target_minutes intervals
    let data_count = sorted_data.len();
    let mut resampled = HashMap::new();
    let mut current_bucket_start: Option<i64> = None;
    let mut current_bucket: Vec<(i64, HistoricalData)> = Vec::new();

    for (timestamp, data) in sorted_data {
        // Calculate bucket start time (round down to nearest target_minutes boundary)
        let bucket_start =
            (timestamp / (target_minutes as i64 * 60)) * (target_minutes as i64 * 60);

        match current_bucket_start {
            Some(start) if start == bucket_start => {
                // Same bucket, add to current bucket
                current_bucket.push((timestamp, data));
            }
            _ => {
                // New bucket or first bucket
                // Process previous bucket if exists
                if let Some(start) = current_bucket_start
                    && let Some(aggregated) = aggregate_bucket(&current_bucket)
                {
                    let rfc3339_timestamp = chrono::DateTime::from_timestamp(start, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_else(|| start.to_string());
                    resampled.insert(rfc3339_timestamp, aggregated);
                }

                // Start new bucket
                current_bucket_start = Some(bucket_start);
                current_bucket = vec![(timestamp, data)];
            }
        }
    }

    // Process last bucket
    if let Some(start) = current_bucket_start
        && let Some(aggregated) = aggregate_bucket(&current_bucket)
    {
        let rfc3339_timestamp = chrono::DateTime::from_timestamp(start, 0)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| start.to_string());
        resampled.insert(rfc3339_timestamp, aggregated);
    }

    info!(
        "Resampled {} 1m data points to {} {}m intervals",
        data_count,
        resampled.len(),
        target_minutes
    );

    Ok(HistoricalResponse { data: resampled })
}

/// Aggregates a bucket of 1m data points into a single OHLCV candle
fn aggregate_bucket(bucket: &[(i64, HistoricalData)]) -> Option<HistoricalData> {
    if bucket.is_empty() {
        return None;
    }

    // Open: first open in the bucket
    let open = bucket.first()?.1.open;

    // High: maximum high in the bucket
    let high = bucket
        .iter()
        .map(|(_, data)| data.high)
        .fold(f64::NEG_INFINITY, f64::max);

    // Low: minimum low in the bucket
    let low = bucket
        .iter()
        .map(|(_, data)| data.low)
        .fold(f64::INFINITY, f64::min);

    // Close: last close in the bucket
    let close = bucket.last()?.1.close;

    // Volume: sum of all volumes
    let volume: i64 = bucket.iter().map(|(_, data)| data.volume).sum();

    // Adj_close: last adj_close in the bucket (if available)
    let adj_close = bucket.last()?.1.adj_close;

    Some(HistoricalData {
        open,
        high,
        low,
        close,
        volume,
        adj_close,
        sma: None,
        ema: None,
    })
}

/// Calculate indicators for historical data based on requested indicator types and periods
/// Returns a new HistoricalResponse with only the requested indicators included
/// periods: Vector of periods to calculate (e.g., [10, 20, 50] for EMA10, EMA20, EMA50)
pub fn calculate_indicators(
    historical: HistoricalResponse,
    periods: &[usize],
    requested_indicators: &HashSet<IndicatorType>,
) -> HistoricalResponse {
    // Convert historical data to sorted PricePoint vector
    let mut price_points: Vec<PricePoint> = historical
        .data
        .iter()
        .filter_map(|(ts_str, data)| {
            // Try parsing as RFC3339 first, then fallback to Unix timestamp string
            let timestamp = chrono::DateTime::parse_from_rfc3339(ts_str)
                .map(|dt| dt.timestamp())
                .ok()
                .or_else(|| ts_str.parse::<i64>().ok());

            timestamp.map(|ts| PricePoint {
                price: data.close,
                timestamp: ts,
            })
        })
        .collect();

    // Sort by timestamp (oldest first)
    price_points.sort_by_key(|p| p.timestamp);

    // Calculate indicators for each period
    // Structure: HashMap<period_string, HashMap<timestamp, value>>
    let mut sma_maps: HashMap<String, HashMap<i64, f64>> = HashMap::new();
    let mut ema_maps: HashMap<String, HashMap<i64, f64>> = HashMap::new();

    for &period in periods {
        let period_str = period.to_string();

        // Skip this period if we don't have enough data points
        if price_points.len() < period {
            continue; // Skip this period, but continue with others
        }

        if requested_indicators.contains(&IndicatorType::SMA) {
            let sma_series = calculate_ma_series(&price_points, MovingAverageType::SMA, period);
            let sma_map: HashMap<i64, f64> = sma_series.into_iter().collect();
            sma_maps.insert(period_str.clone(), sma_map);
        }

        if requested_indicators.contains(&IndicatorType::EMA) {
            let ema_series = calculate_ma_series(&price_points, MovingAverageType::EMA, period);
            let ema_map: HashMap<i64, f64> = ema_series.into_iter().collect();
            ema_maps.insert(period_str.clone(), ema_map);
        }
    }

    // Build new response with only requested indicators
    let mut new_data = HashMap::new();

    for (ts_str, mut data) in historical.data {
        // Parse timestamp to match with MA values
        let timestamp = chrono::DateTime::parse_from_rfc3339(&ts_str)
            .map(|dt| dt.timestamp())
            .ok()
            .or_else(|| ts_str.parse::<i64>().ok());

        if let Some(ts) = timestamp {
            // Build SMA HashMap for this timestamp
            if requested_indicators.contains(&IndicatorType::SMA) {
                let mut sma_values = HashMap::new();
                for period_str in periods.iter().map(|p| p.to_string()) {
                    if let Some(sma_map) = sma_maps.get(&period_str)
                        && let Some(value) = sma_map.get(&ts)
                    {
                        sma_values.insert(period_str, *value);
                    }
                }
                data.sma = if sma_values.is_empty() {
                    None
                } else {
                    Some(sma_values)
                };
            } else {
                data.sma = None;
            }

            // Build EMA HashMap for this timestamp
            if requested_indicators.contains(&IndicatorType::EMA) {
                let mut ema_values = HashMap::new();
                for period_str in periods.iter().map(|p| p.to_string()) {
                    if let Some(ema_map) = ema_maps.get(&period_str)
                        && let Some(value) = ema_map.get(&ts)
                    {
                        ema_values.insert(period_str, *value);
                    }
                }
                data.ema = if ema_values.is_empty() {
                    None
                } else {
                    Some(ema_values)
                };
            } else {
                data.ema = None;
            }
        } else {
            // If timestamp parsing failed, set indicators to None
            data.sma = None;
            data.ema = None;
        }

        new_data.insert(ts_str, data);
    }

    HistoricalResponse { data: new_data }
}
