use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_ws::handle;
use crate::service::websocket::{ConnectionManagerAddr, handle_websocket_session, StartTask};
use crate::service::market::MarketSchedule;
use serde_json::json;
use std::sync::Arc;
use tokio::time::sleep;
use tracing::info;

use super::common::REFRESH_INTERVAL;

/// Hours WebSocket endpoint - streams market status
pub async fn hours_handler(
    req: HttpRequest,
    body: web::Payload,
    manager: web::Data<ConnectionManagerAddr>,
) -> Result<HttpResponse> {
    info!("WebSocket hours connection established");
    
    let channel = "hours".to_string();
    let market_schedule = Arc::new(MarketSchedule::new());
    
    // Create data fetching task (matches Python pattern)
    let manager_clone = manager.get_ref().clone();
    let channel_clone = channel.clone();
    let schedule_clone = market_schedule.clone();
    
    let task = tokio::spawn(async move {
        loop {
            let (status, reason) = schedule_clone.get_market_status();
            let timestamp = chrono::Utc::now().to_rfc3339();
            
            let data = json!({
                "status": status.as_str(),
                "reason": reason,
                "timestamp": timestamp,
            });
            
            manager_clone.do_send(crate::service::websocket::BroadcastMessage {
                channel: channel_clone.clone(),
                message: data,
            });
            
            sleep(REFRESH_INTERVAL).await;
        }
    });
    
    // Pass task to connection manager
    manager.get_ref().do_send(StartTask {
        channel: channel.clone(),
        task,
    });
    
    let session_id = req.peer_addr().map(|a| a.port() as usize).unwrap_or(0);
    let manager_addr = manager.get_ref().clone();
    
    let (response, session, msg_stream) = handle(&req, body)?;
    
    // Spawn the WebSocket handler
    actix_web::rt::spawn(async move {
        handle_websocket_session(session, msg_stream, session_id, manager_addr, channel).await;
    });
    
    Ok(response)
}