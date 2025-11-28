use crate::client::YahooFinanceClient;
use crate::client::error::YahooError;
use crate::models::{HistoricalData, HistoricalResponse, TimeRange, Interval};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info};

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
    info!("Fetching max range with 1d interval for {}, using optimized chunking strategy", symbol);
    
    // Step 1: Detect earliest available date by fetching max with 1mo interval
    let earliest_data = yahoo_client
        .get_chart(symbol, "1mo", "max")
        .await?;
    
    let earliest_response = parse_historical_data(earliest_data)?;
    
    // Find the earliest timestamp (keys are now in RFC3339 format)
    let earliest_timestamp = earliest_response.data.keys()
        .filter_map(|k| {
            // Try parsing as RFC3339 first, then fallback to Unix timestamp string
            chrono::DateTime::parse_from_rfc3339(k)
                .map(|dt| dt.timestamp())
                .ok()
                .or_else(|| {
                    k.parse::<i64>().ok()
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
    
    info!("Detected earliest data point for {}: {} ({})", 
          symbol, 
          earliest_timestamp,
          chrono::DateTime::from_timestamp(earliest_timestamp, 0)
              .map(|dt| dt.to_rfc3339())
              .unwrap_or_else(|| "unknown".to_string()));
    
    // Step 2: Calculate chunks and fetch data
    let mut all_data = HashMap::new();
    let now = chrono::Utc::now().timestamp();
    
    // Chunk size: 10 years (in seconds)
    const CHUNK_SIZE: i64 = 10 * 365 * 24 * 60 * 60;
    
    let mut period1 = earliest_timestamp;
    let mut chunk_count = 0;
    
    loop {
        let period2 = (period1 + CHUNK_SIZE).min(now);
        
        debug!("Fetching chunk {} for {}: {} to {} ({})", 
               chunk_count + 1, 
               symbol, 
               period1, 
               period2,
               chrono::DateTime::from_timestamp(period1, 0)
                   .map(|dt| dt.to_rfc3339())
                   .unwrap_or_else(|| "unknown".to_string()));
        
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
    
    info!("Successfully fetched {} chunks for {} (total {} data points)", 
          chunk_count, symbol, all_data.len());
    
    Ok(HistoricalResponse { data: all_data })
}

fn parse_historical_data(data: Value) -> Result<HistoricalResponse, YahooError> {
    let mut history_map = HashMap::new();

    if let Some(chart) = data.get("chart") {
        if let Some(results) = chart.get("result").and_then(|r| r.as_array()) {
            if let Some(result) = results.first() {
                if let Some(timestamps) = result.get("timestamp").and_then(|t| t.as_array()) {
                    if let Some(indicators) = result.get("indicators") {
                        if let Some(quote) = indicators.get("quote").and_then(|q| q.as_array()) {
                            if let Some(quote_data) = quote.first() {
                                let empty_vec = vec![];
                                let opens = quote_data.get("open").and_then(|o| o.as_array()).unwrap_or(&empty_vec);
                                let highs = quote_data.get("high").and_then(|h| h.as_array()).unwrap_or(&empty_vec);
                                let lows = quote_data.get("low").and_then(|l| l.as_array()).unwrap_or(&empty_vec);
                                let closes = quote_data.get("close").and_then(|c| c.as_array()).unwrap_or(&empty_vec);
                                let volumes = quote_data.get("volume").and_then(|v| v.as_array()).unwrap_or(&empty_vec);

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
                                        let adj_close = adj_closes
                                            .and_then(|ac| ac.get(i).and_then(|v| v.as_f64()));

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
                                            },
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(HistoricalResponse { data: history_map })
}

