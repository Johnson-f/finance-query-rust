use actix_ws::{Message, Session};
use crate::service::websocket::connection_manager::{BroadcastMessage, Connect, Disconnect, ConnectionManagerAddr, StartTask};
use crate::service::quotes::get_simple_quotes;
use crate::service::get_historical;
use crate::client::{YahooFinanceClient, FetchClient};
use crate::service::websocket::indicator::price_buffer::{PriceBufferManager, PricePoint};
use crate::service::websocket::indicator::moving_average::{MovingAverageType, calculate_ma, calculate_ma_series};
use crate::models::{TimeRange, Interval};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
const REFRESH_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Deserialize, Debug, Clone)]
struct SubscriptionRequest {
    symbol: String,
    #[serde(rename = "type")]
    ma_type: String,  // "sma" or "ema"
    day: usize,  // period in days/periods
    timeframe: String,  // "1d", "1wk", "1d & 1m", "1d & 5m", etc.
}

#[derive(Serialize)]
struct MovingAverageUpdate {
    symbol: String,
    #[serde(rename = "type")]
    ma_type: String,
    day: usize,
    timeframe: String,
    // Historical series: array of {timestamp, value} pairs
    series: Vec<MAPoint>,
    // Current/latest value (for backward compatibility)
    value: Option<f64>,
    price: f64,
    timestamp: String,
}

#[derive(Serialize)]
struct MAPoint {
    timestamp: String,  // RFC3339 / ISO 8601 format
    value: f64,
}

/// Parse timeframe string and return (Interval, TimeRange)
/// Examples:
/// - "1wk" -> (Interval::Weekly, TimeRange::Max)
/// - "1d" -> (Interval::Daily, TimeRange::Max)
/// - "1d & 1m" -> (Interval::OneMinute, TimeRange::Day)
/// - "1d & 5m" -> (Interval::FiveMinutes, TimeRange::Day)
fn parse_timeframe(timeframe: &str) -> (Interval, TimeRange) {
    let timeframe_lower = timeframe.to_lowercase();
    let timeframe_lower = timeframe_lower.trim();
    
    // Check for intraday patterns: "1d & 1m", "1d & 5m", etc.
    if timeframe_lower.contains("&") {
        let parts: Vec<&str> = timeframe_lower.split('&').map(|s| s.trim()).collect();
        if parts.len() == 2 && parts[0] == "1d" {
            let interval_str = parts[1].trim();
            let interval = match interval_str {
                "1m" => Interval::OneMinute,
                "3m" => Interval::ThreeMinutes,
                "5m" => Interval::FiveMinutes,
                "10m" => Interval::TenMinutes,
                "15m" => Interval::FifteenMinutes,
                "20m" => Interval::TwentyMinutes,
                "30m" => Interval::ThirtyMinutes,
                "65m" => Interval::SixtyFiveMinutes,
                "95m" => Interval::NinetyFiveMinutes,
                "1h" => Interval::OneHour,
                _ => {
                    warn!("Unknown intraday interval: {}, defaulting to 1m", interval_str);
                    Interval::OneMinute
                }
            };
            return (interval, TimeRange::Day);
        }
    }
    
    // Daily or weekly intervals
    match timeframe_lower {
        "1d" => (Interval::Daily, TimeRange::Max),
        "1wk" => (Interval::Weekly, TimeRange::Max),
        _ => {
            warn!("Unknown timeframe: {}, defaulting to 1d", timeframe);
            (Interval::Daily, TimeRange::Max)
        }
    }
}

/// Handle WebSocket session for moving averages
pub async fn handle_moving_average_websocket_session(
    mut session: Session,
    mut ws_stream: actix_ws::MessageStream,
    id: usize,
    manager: ConnectionManagerAddr,
    yahoo_client: Arc<YahooFinanceClient>,
    fetch_client: Arc<FetchClient>,
    price_buffer: Arc<PriceBufferManager>,
) {
    debug!("Moving average WebSocket session {} started", id);
    
    let mut last_heartbeat = Instant::now();
    let mut heartbeat_interval = tokio::time::interval(HEARTBEAT_INTERVAL);
    heartbeat_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    
    let mut subscriptions: Vec<SubscriptionRequest> = Vec::new();
    let mut channel: Option<String> = None;
    let mut send_task: Option<tokio::task::JoinHandle<()>> = None;
    
    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        last_heartbeat = Instant::now();
                        
                        // Parse subscription request
                        match serde_json::from_str::<SubscriptionRequest>(&text) {
                            Ok(sub) => {
                                info!("Session {} subscribing to {} {} {} {} for {}", 
                                    id, sub.symbol, sub.ma_type, sub.day, sub.timeframe, sub.symbol);
                                
                                subscriptions.push(sub.clone());
                                
                                // Create channel name from subscriptions
                                let channel_name = format!("ma:{}", 
                                    subscriptions.iter()
                                        .map(|s| format!("{}:{}:{}:{}", s.symbol, s.ma_type, s.day, s.timeframe))
                                        .collect::<Vec<_>>()
                                        .join(",")
                                );
                                
                                if channel.is_none() {
                                    channel = Some(channel_name.clone());
                                    
                                    // Create broadcast channel
                                    let (broadcast_tx, mut broadcast_rx) = mpsc::unbounded_channel::<Value>();
                                    
                                    // Register with connection manager
                                    manager.do_send(Connect {
                                        session_id: id,
                                        sender: broadcast_tx.clone(),
                                        channel: channel_name.clone(),
                                    });
                                    
                                    // Initialize price buffers with historical data
                                    initialize_buffers(&price_buffer, &yahoo_client, &fetch_client, &subscriptions).await;
                                    
                                    // Send initial data
                                    if let Ok(initial_data) = calculate_all_mas(&price_buffer, &subscriptions).await {
                                        let json_str = serde_json::to_string(&initial_data).unwrap_or_default();
                                        let _ = session.text(json_str).await;
                                    }
                                    
                                    // Start background task
                                    let manager_clone = manager.clone();
                                    let channel_task = channel_name.clone();
                                    let yahoo_for_task = yahoo_client.clone();
                                    let fetch_for_task = fetch_client.clone();
                                    let buffer_for_task = price_buffer.clone();
                                    let subs_for_task = subscriptions.clone();
                                    
                                    let task = tokio::spawn(async move {
                                        loop {
                                            // Group subscriptions by interval type
                                            let mut daily_subs = Vec::new();
                                            let mut weekly_subs = Vec::new();
                                            let mut intraday_subs = Vec::new();
                                            
                                            for sub in &subs_for_task {
                                                let (interval, _) = parse_timeframe(&sub.timeframe);
                                                match interval {
                                                    Interval::Daily => daily_subs.push(sub),
                                                    Interval::Weekly => weekly_subs.push(sub),
                                                    _ => intraday_subs.push(sub),
                                                }
                                            }
                                            
                                            let mut should_broadcast = false;
                                            
                                            // Handle daily intervals - only update once per day at market close
                                            if !daily_subs.is_empty() {
                                                let symbols: Vec<&str> = daily_subs.iter()
                                                    .map(|s| s.symbol.as_str())
                                                    .collect::<HashSet<_>>()
                                                    .into_iter()
                                                    .collect();
                                                
                                                if let Ok(quotes) = get_simple_quotes(&yahoo_for_task, &fetch_for_task, &symbols).await {
                                                    for quote in &quotes {
                                                        if let Ok(price) = quote.price.parse::<f64>() {
                                                            // For each daily subscription for this symbol, try to add price
                                                            for sub in &daily_subs {
                                                                if sub.symbol == quote.symbol {
                                                                    let buffer_key = format!("{}:{}", sub.symbol, sub.timeframe);
                                                                    if buffer_for_task.add_price(&buffer_key, price, true, false).await {
                                                                        should_broadcast = true;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            // Handle weekly intervals - only update once per week (Friday after market close)
                                            if !weekly_subs.is_empty() {
                                                let symbols: Vec<&str> = weekly_subs.iter()
                                                    .map(|s| s.symbol.as_str())
                                                    .collect::<HashSet<_>>()
                                                    .into_iter()
                                                    .collect();
                                                
                                                if let Ok(quotes) = get_simple_quotes(&yahoo_for_task, &fetch_for_task, &symbols).await {
                                                    for quote in &quotes {
                                                        if let Ok(price) = quote.price.parse::<f64>() {
                                                            // For each weekly subscription for this symbol, try to add price
                                                            for sub in &weekly_subs {
                                                                if sub.symbol == quote.symbol {
                                                                    let buffer_key = format!("{}:{}", sub.symbol, sub.timeframe);
                                                                    if buffer_for_task.add_price(&buffer_key, price, false, true).await {
                                                                        should_broadcast = true;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            // Handle intraday intervals - update more frequently
                                            if !intraday_subs.is_empty() {
                                                let symbols: Vec<&str> = intraday_subs.iter()
                                                    .map(|s| s.symbol.as_str())
                                                    .collect::<HashSet<_>>()
                                                    .into_iter()
                                                    .collect();
                                                
                                                if let Ok(quotes) = get_simple_quotes(&yahoo_for_task, &fetch_for_task, &symbols).await {
                                                    for quote in &quotes {
                                                        if let Ok(price) = quote.price.parse::<f64>() {
                                                            // For each intraday subscription for this symbol, add price
                                                            for sub in &intraday_subs {
                                                                if sub.symbol == quote.symbol {
                                                                    let buffer_key = format!("{}:{}", sub.symbol, sub.timeframe);
                                                                    if buffer_for_task.add_price(&buffer_key, price, false, false).await {
                                                                        should_broadcast = true;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            // Only broadcast if a new price was added
                                            if should_broadcast {
                                                if let Ok(ma_data) = calculate_all_mas(&buffer_for_task, &subs_for_task).await {
                                                    manager_clone.do_send(BroadcastMessage {
                                                        channel: channel_task.clone(),
                                                        message: json!(ma_data),
                                                    });
                                                }
                                            }
                                            
                                            sleep(REFRESH_INTERVAL).await;
                                        }
                                    });
                                    
                                    manager.do_send(StartTask {
                                        channel: channel_name,
                                        task,
                                    });
                                    
                                    // Spawn task to send broadcast messages
                                    let mut session_send = session.clone();
                                    let broadcast_task = tokio::spawn(async move {
                                        while let Some(message) = broadcast_rx.recv().await {
                                            let json_str = match serde_json::to_string(&message) {
                                                Ok(s) => s,
                                                Err(e) => {
                                                    error!("Failed to serialize message: {}", e);
                                                    continue;
                                                }
                                            };
                                            
                                            if let Err(e) = session_send.text(json_str).await {
                                                error!("Failed to send message: {}", e);
                                                break;
                                            }
                                        }
                                    });
                                    
                                    send_task = Some(broadcast_task);
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse subscription request: {}", e);
                                let _ = session.text(json!({"error": "Invalid subscription format"}).to_string()).await;
                            }
                        }
                    }
                    Some(Ok(Message::Ping(bytes))) => {
                        last_heartbeat = Instant::now();
                        let _ = session.pong(&bytes).await;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        last_heartbeat = Instant::now();
                    }
                    Some(Ok(Message::Close(_))) => break,
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    None => break,
                    _ => {}
                }
            }
            
            // Handle heartbeat
            _ = heartbeat_interval.tick() => {
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    warn!("Client {} timed out", id);
                    break;
                }
                let _ = session.ping(b"").await;
            }
        }
    }
    
    // Cleanup
    if let Some(task) = send_task {
        task.abort();
    }
    
    if let Some(channel_name) = channel {
        manager.do_send(Disconnect {
            session_id: id,
            channel: channel_name,
        });
    }
}

async fn initialize_buffers(
    buffer: &PriceBufferManager,
    yahoo_client: &Arc<YahooFinanceClient>,
    _fetch_client: &Arc<FetchClient>,
    subscriptions: &[SubscriptionRequest],
) {
    // Group subscriptions by (symbol, timeframe) to avoid duplicate fetches
    use std::collections::HashMap;
    let mut symbol_timeframes: HashMap<(String, String), (Interval, TimeRange)> = HashMap::new();
    
    for sub in subscriptions {
        let (interval, time_range) = parse_timeframe(&sub.timeframe);
        symbol_timeframes.insert((sub.symbol.clone(), sub.timeframe.clone()), (interval, time_range));
    }
    
    // Fetch historical data for each unique (symbol, timeframe) combination
    for ((symbol, timeframe), (interval, time_range)) in symbol_timeframes {
        match get_historical(yahoo_client.as_ref(), &symbol, time_range, interval).await {
            Ok(historical) => {
                // Convert historical data to PricePoint vector
                // Timestamps are stored as RFC3339 strings, need to parse them
                let mut prices: Vec<PricePoint> = historical.data.iter()
                    .filter_map(|(ts_str, data)| {
                        // Try parsing as RFC3339 first, then fallback to Unix timestamp string
                        let timestamp = chrono::DateTime::parse_from_rfc3339(ts_str)
                            .map(|dt| dt.timestamp())
                            .ok()
                            .or_else(|| {
                                ts_str.parse::<i64>().ok()
                            });
                        
                        timestamp.map(|ts| PricePoint {
                            price: data.close,
                            timestamp: ts,
                        })
                    })
                    .collect();
                
                // Sort by timestamp (oldest first)
                prices.sort_by_key(|p| p.timestamp);
                
                // Create a unique key for this symbol+timeframe combination
                let buffer_key = format!("{}:{}", symbol, timeframe);
                let prices_len = prices.len();
                if !prices.is_empty() {
                    buffer.initialize_from_historical(&buffer_key, prices).await;
                    info!("Initialized price buffer for {} (timeframe: {}) with {} data points", symbol, timeframe, prices_len);
                } else {
                    warn!("No historical data found for {} (timeframe: {})", symbol, timeframe);
                }
            }
            Err(e) => {
                error!("Failed to fetch historical data for {} (timeframe: {}): {}", symbol, timeframe, e);
            }
        }
    }
}

async fn calculate_all_mas(
    buffer: &PriceBufferManager,
    subscriptions: &[SubscriptionRequest],
) -> Result<Vec<MovingAverageUpdate>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    for sub in subscriptions {
        // Use symbol:timeframe as the buffer key
        let buffer_key = format!("{}:{}", sub.symbol, sub.timeframe);
        let prices = buffer.get_prices(&buffer_key).await;
        
        let ma_type = match sub.ma_type.to_lowercase().as_str() {
            "sma" => MovingAverageType::SMA,
            "ema" => MovingAverageType::EMA,
            _ => {
                warn!("Invalid MA type: {}, skipping", sub.ma_type);
                continue;
            }
        };
        
        // Calculate full historical series
        let ma_series = calculate_ma_series(&prices, ma_type, sub.day);
        let series: Vec<MAPoint> = ma_series
            .into_iter()
            .map(|(timestamp, value)| {
                // Convert Unix timestamp (i64) to RFC3339 string
                let dt = chrono::DateTime::from_timestamp(timestamp, 0)
                    .unwrap_or_else(|| chrono::Utc::now());
                MAPoint {
                    timestamp: dt.to_rfc3339(),
                    value,
                }
            })
            .collect();
        
        // Get latest value (for backward compatibility)
        let ma_value = calculate_ma(&prices, ma_type, sub.day);
        let current_price = prices.last().map(|p| p.price).unwrap_or(0.0);
        
        results.push(MovingAverageUpdate {
            symbol: sub.symbol.clone(),
            ma_type: sub.ma_type.clone(),
            day: sub.day,
            timeframe: sub.timeframe.clone(),
            series,
            value: ma_value,
            price: current_price,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
    }
    
    Ok(results)
}