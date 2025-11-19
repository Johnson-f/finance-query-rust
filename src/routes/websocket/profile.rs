use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_ws::handle;
use crate::service::websocket::{ConnectionManagerAddr, handle_websocket_session, StartTask};
use crate::AppState;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::time::sleep;
use tracing::{error, info};

use super::common::REFRESH_INTERVAL;

/// Profile WebSocket endpoint - streams quote, similar, sector, news for a symbol
pub async fn profile_handler(
    req: HttpRequest,
    body: web::Payload,
    path: web::Path<String>,
    app_state: web::Data<AppState>,
    manager: web::Data<ConnectionManagerAddr>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner().to_uppercase();
    let channel = format!("profile:{}", symbol);
    
    info!("WebSocket profile connection for symbol: {}", symbol);
    
    // Create data fetching task (matches Python pattern - task passed to connection manager)
    let manager_clone = manager.get_ref().clone();
    let channel_clone = channel.clone();
    let yahoo_client_clone = app_state.yahoo_client.clone();
    let fetch_client_clone = app_state.fetch_client.clone();
    let symbol_clone = symbol.clone();
    
    let task = tokio::spawn(async move {
        loop {
            // Clone Arc values for each iteration to avoid holding references across await
            let yahoo = yahoo_client_clone.clone();
            let fetch = fetch_client_clone.clone();
            let sym = symbol_clone.clone();
            
            let data_result = fetch_profile_data(&yahoo, &fetch, &sym).await;
            
            match data_result {
                Ok(data) => {
                    manager_clone.do_send(crate::service::websocket::BroadcastMessage {
                        channel: channel_clone.clone(),
                        message: data,
                    });
                }
                Err(e) => {
                    error!("Failed to fetch profile data for {}: {}", sym, e);
                }
            }
            sleep(REFRESH_INTERVAL).await;
        }
    });
    
    // Pass task to connection manager (will be started when first connection to channel is made)
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

async fn fetch_profile_data(
    yahoo_client: &Arc<crate::client::YahooFinanceClient>,
    fetch_client: &Arc<crate::client::FetchClient>,
    symbol: &str,
) -> Result<Value, crate::client::error::YahooError> {
    use crate::service;
    
    let symbols = vec![symbol];
    let quotes_task = service::get_quotes(yahoo_client, fetch_client, &symbols);
    let similar_task = service::get_similar_quotes(yahoo_client, fetch_client, symbol, 10);
    let sector_task = service::get_sector_for_symbol(yahoo_client, fetch_client, symbol);
    let news_task = service::scrape_news_for_quote(fetch_client, symbol);
    
    let (quotes_result, similar_result, sector_result, news_result) = tokio::join!(
        quotes_task,
        similar_task,
        sector_task,
        news_task
    );
    
    let quote = quotes_result.ok()
        .and_then(|q| q.first().cloned())
        .map(|q| json!(q));
    let similar = similar_result.ok().map(|s| json!(s));
    let sector = sector_result.ok().map(|s| json!(s));
    let news = news_result.ok().map(|n| json!(n));
    
    Ok(json!({
        "quote": quote,
        "similar": similar,
        "sectorPerformance": sector,
        "news": news,
    }))
}