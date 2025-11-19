use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_ws::handle;
use crate::service::websocket::{ConnectionManagerAddr, handle_websocket_session, StartTask};
use crate::AppState;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::time::sleep;
use tracing::{error, info};

use super::common::REFRESH_INTERVAL;

/// Indices WebSocket endpoint - streams US market indices
pub async fn indices_handler(
    req: HttpRequest,
    body: web::Payload,
    app_state: web::Data<AppState>,
    manager: web::Data<ConnectionManagerAddr>,
) -> Result<HttpResponse> {
    info!("WebSocket indices connection established");
    
    let channel = "indices".to_string();
    
    // Start background task to fetch and broadcast data
    let manager_clone = manager.get_ref().clone();
    let channel_clone = channel.clone();
    let yahoo_client_clone = app_state.yahoo_client.clone();
    let fetch_client_clone = app_state.fetch_client.clone();
    
    // Create data fetching task
    let task = tokio::spawn(async move {
        loop {
            // Clone Arc values for each iteration to avoid holding references across await
            let yahoo = yahoo_client_clone.clone();
            let fetch = fetch_client_clone.clone();
            
            let data_result = fetch_indices_data(&yahoo, &fetch).await;
            
            match data_result {
                Ok(data) => {
                    manager_clone.do_send(crate::service::websocket::BroadcastMessage {
                        channel: channel_clone.clone(),
                        message: data,
                    });
                }
                Err(e) => {
                    error!("Failed to fetch indices data: {}", e);
                }
            }
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

async fn fetch_indices_data(
    yahoo_client: &Arc<crate::client::YahooFinanceClient>,
    fetch_client: &Arc<crate::client::FetchClient>,
) -> Result<Value, crate::client::error::YahooError> {
    use crate::service;
    use crate::models::indices::Index;
    
    // Only fetch DJIA, NASDAQ, and S&P 500
    let indices_to_fetch = vec![
        Index::Dji,   // Dow Jones Industrial Average
        Index::Ixic,  // NASDAQ Composite
        Index::Gspc,  // S&P 500
    ];
    
    let indices = service::get_indices(yahoo_client, fetch_client, Some(indices_to_fetch), None).await?;
    
    Ok(json!(indices))
}