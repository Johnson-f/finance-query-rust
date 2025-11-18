use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_ws::handle;
use crate::service::websocket::{ConnectionManagerAddr, handle_websocket_session, StartTask};
use crate::AppState;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use super::common::REFRESH_INTERVAL;

/// News WebSocket endpoint - streams general market news
pub async fn news_handler(
    req: HttpRequest,
    body: web::Payload,
    app_state: web::Data<AppState>,
    manager: web::Data<ConnectionManagerAddr>,
) -> Result<HttpResponse> {
    info!("WebSocket news connection established");
    
    let channel = "news".to_string();
    
    // Start background task to fetch and broadcast data
    let manager_clone = manager.get_ref().clone();
    let channel_clone = channel.clone();
    let fetch_client_clone = app_state.fetch_client.clone();
    
    // Create data fetching task
    let task = tokio::spawn(async move {
        loop {
            // Clone Arc values for each iteration to avoid holding references across await
            let fetch = fetch_client_clone.clone();
            
            let data_result = fetch_news_data(&fetch).await;
            
            match data_result {
                Ok(data) => {
                    manager_clone.do_send(crate::service::websocket::BroadcastMessage {
                        channel: channel_clone.clone(),
                        message: data,
                    });
                }
                Err(e) => {
                    error!("Failed to fetch news data: {}", e);
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

async fn fetch_news_data(
    fetch_client: &Arc<crate::client::FetchClient>,
) -> Result<Value, crate::client::error::YahooError> {
    use crate::service;
    
    let news = service::scrape_general_news(fetch_client).await?;
    
    Ok(json!(news))
}