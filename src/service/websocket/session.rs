use actix_ws::{Message, Session};
use crate::service::websocket::connection_manager::{BroadcastMessage, Connect, Disconnect, ConnectionManagerAddr};
use futures_util::StreamExt;
use serde_json::Value;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, error, warn};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

/// Handle a WebSocket session for a specific channel
pub async fn handle_websocket_session(
    mut session: Session,
    mut ws_stream: actix_ws::MessageStream,
    id: usize,
    manager: ConnectionManagerAddr,
    channel: String,
) {
    debug!("WebSocket session {} started for channel: {}", id, channel);
    
    // Create channel for receiving broadcast messages
    let (tx, mut rx) = mpsc::unbounded_channel::<Value>();
    
    // Register with connection manager
    manager.do_send(Connect {
        session_id: id,
        sender: tx.clone(),
        channel: channel.clone(),
    });
    
    let mut last_heartbeat = Instant::now();
    let mut heartbeat_interval = tokio::time::interval(HEARTBEAT_INTERVAL);
    heartbeat_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    
    // Spawn task to send broadcast messages
    let mut session_send = session.clone();
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
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
    
    // Main loop: handle incoming messages and heartbeats
    loop {
        tokio::select! {
            // Handle incoming WebSocket messages
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        last_heartbeat = Instant::now();
                        debug!("Received text message from session {}: {}", id, text);
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
                        debug!("WebSocket session {} closing: {:?}", id, reason);
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
    debug!("WebSocket session {} stopping for channel: {}", id, channel);
    send_task.abort();
    
    // Unregister from connection manager
    manager.do_send(Disconnect {
        session_id: id,
        channel,
    });
}
