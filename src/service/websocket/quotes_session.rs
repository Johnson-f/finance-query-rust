use actix_ws::{Message, Session};
use crate::service::websocket::connection_manager::{BroadcastMessage, Connect, Disconnect, ConnectionManagerAddr, StartTask};
use crate::service::quotes::get_simple_quotes;
use crate::client::YahooFinanceClient;
use crate::client::FetchClient;
use futures_util::StreamExt;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
const REFRESH_INTERVAL: Duration = Duration::from_secs(5);

/// Handle a WebSocket session for quotes endpoint that receives symbols from client
pub async fn handle_quotes_websocket_session(
    mut session: Session,
    mut ws_stream: actix_ws::MessageStream,
    id: usize,
    manager: ConnectionManagerAddr,
    yahoo_client: Arc<YahooFinanceClient>,
    fetch_client: Arc<FetchClient>,
) {
    debug!("Quotes WebSocket session {} started, waiting for symbols", id);
    
    let mut last_heartbeat = Instant::now();
    let mut heartbeat_interval = tokio::time::interval(HEARTBEAT_INTERVAL);
    heartbeat_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    
    let mut symbols_received = false;
    let mut channel: Option<String> = None;
    let mut send_task: Option<tokio::task::JoinHandle<()>> = None;
    
    // Main loop: wait for symbols, then handle messages
    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        last_heartbeat = Instant::now();
                        
                        if !symbols_received {
                            // First message should contain symbols
                            let symbols_text = text.to_string();
                            debug!("Received symbols from session {}: {}", id, symbols_text);
                            
                            // Parse symbols from comma-separated string
                            let symbols: Vec<String> = symbols_text
                                .split(',')
                                .map(|s| s.trim().to_uppercase())
                                .filter(|s| !s.is_empty())
                                .collect::<std::collections::HashSet<_>>()
                                .into_iter()
                                .collect();
                            
                            if symbols.is_empty() {
                                error!("No valid symbols received from client");
                                let _ = session.close(None).await;
                                break;
                            }
                            
                            info!("WebSocket quotes subscription for symbols: {:?} (count: {})", symbols, symbols.len());
                            
                            let channel_name = format!("quotes:{}", symbols.join(","));
                            channel = Some(channel_name.clone());
                            
                            // Create channel for receiving broadcast messages
                            let (broadcast_tx, mut broadcast_rx) = mpsc::unbounded_channel::<Value>();
                            
                            // Register with connection manager
                            manager.do_send(Connect {
                                session_id: id,
                                sender: broadcast_tx.clone(),
                                channel: channel_name.clone(),
                            });
                            
                            // Fetch initial data
                            let initial_quotes = match fetch_quotes_for_symbols(&yahoo_client, &fetch_client, &symbols).await {
                                Ok(quotes) => quotes,
                                Err(e) => {
                                    error!("Failed to fetch initial quotes: {}", e);
                                    Vec::new()
                                }
                            };
                            
                            // Send initial data
                            let json_str = match serde_json::to_string(&initial_quotes) {
                                Ok(s) => s,
                                Err(e) => {
                                    error!("Failed to serialize initial quotes: {}", e);
                                    let _ = session.close(None).await;
                                    break;
                                }
                            };
                            
                            if let Err(e) = session.text(json_str).await {
                                error!("Failed to send initial quotes to session {}: {}", id, e);
                                break;
                            }
                            
                            // Start background task
                            let manager_clone = manager.clone();
                            let channel_task = channel_name.clone();
                            let yahoo_for_task = yahoo_client.clone();
                            let fetch_for_task = fetch_client.clone();
                            let symbols_for_task = symbols.clone();
                            
                            let task = tokio::spawn(async move {
                                loop {
                                    let quotes_result = fetch_quotes_for_symbols(&yahoo_for_task, &fetch_for_task, &symbols_for_task).await;
                                    
                                    match quotes_result {
                                        Ok(quotes) => {
                                            manager_clone.do_send(BroadcastMessage {
                                                channel: channel_task.clone(),
                                                message: json!(quotes),
                                            });
                                        }
                                        Err(e) => {
                                            error!("Failed to fetch quotes for {:?}: {}", symbols_for_task, e);
                                        }
                                    }
                                    sleep(REFRESH_INTERVAL).await;
                                }
                            });
                            
                            // Send task to connection manager
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
                                            error!("Failed to serialize message for session {}: {}", id, e);
                                            continue;
                                        }
                                    };
                                    
                                    if let Err(e) = session_send.text(json_str).await {
                                        error!("Failed to send message to session {}: {}", id, e);
                                        break;
                                    }
                                }
                            });
                            
                            send_task = Some(broadcast_task);
                            symbols_received = true;
                        } else {
                            // Subsequent messages are just keep-alive
                            debug!("Received keep-alive message from session {}", id);
                        }
                    }
                    Some(Ok(Message::Binary(_))) => {
                        warn!("Received binary message from session {}, ignoring", id);
                    }
                    Some(Ok(Message::Ping(bytes))) => {
                        last_heartbeat = Instant::now();
                        if let Err(e) = session.pong(&bytes).await {
                            error!("Failed to send pong to session {}: {}", id, e);
                            break;
                        }
                    }
                    Some(Ok(Message::Pong(_))) => {
                        last_heartbeat = Instant::now();
                    }
                    Some(Ok(Message::Close(reason))) => {
                        debug!("Quotes WebSocket session {} closing: {:?}", id, reason);
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error for session {}: {}", id, e);
                        break;
                    }
                    None => {
                        debug!("WebSocket stream ended for session {}", id);
                        break;
                    }
                    _ => {}
                }
            }
            
            // Handle heartbeat
            _ = heartbeat_interval.tick() => {
                // Check if client has timed out
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    warn!("WebSocket client {} failed to send heartbeat, disconnecting", id);
                    break;
                }
                
                // Send ping to client
                if let Err(e) = session.ping(b"").await {
                    error!("Failed to send ping to session {}: {}", id, e);
                    break;
                }
            }
        }
    }
    
    // Cleanup
    debug!("Quotes WebSocket session {} stopping", id);
    
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

/// Fetch quotes for symbols and convert to JSON format
async fn fetch_quotes_for_symbols(
    yahoo_client: &Arc<YahooFinanceClient>,
    fetch_client: &Arc<FetchClient>,
    symbols: &[String],
) -> Result<Vec<Value>, crate::client::error::YahooError> {
    let symbol_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
    let quotes = get_simple_quotes(yahoo_client, fetch_client, &symbol_refs).await?;
    
    let mut result = Vec::new();
    for quote in quotes {
        let mut quote_dict = json!({
            "symbol": quote.symbol,
            "name": quote.name,
            "price": quote.price.to_string(),
            "change": quote.change,
            "percentChange": quote.percent_change,
        });

        // Add optional fields if they exist
        if let Some(pre_market) = quote.pre_market_price {
            quote_dict["preMarketPrice"] = json!(pre_market.to_string());
        }

        if let Some(after_hours) = quote.after_hours_price {
            quote_dict["afterHoursPrice"] = json!(after_hours.to_string());
        }

        // Logo removed for WebSocket performance

        result.push(quote_dict);
    }
    
    Ok(result)
}
