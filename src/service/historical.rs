use crate::client::YahooFinanceClient;
use crate::client::error::YahooError;
use crate::models::{HistoricalData, HistoricalResponse, TimeRange, Interval};
use serde_json::Value;
use std::collections::HashMap;

pub async fn get_historical(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
    time_range: TimeRange,
    interval: Interval,
) -> Result<HistoricalResponse, YahooError> {
    let data = yahoo_client
        .get_chart(symbol, interval.as_str(), time_range.as_str())
        .await?;

    parse_historical_data(data)
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

                                        history_map.insert(
                                            ts.to_string(),
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

