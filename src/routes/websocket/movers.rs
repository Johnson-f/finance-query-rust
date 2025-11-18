use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_ws::handle;
use crate::service::websocket::{ConnectionManagerAddr, handle_websocket_session, StartTask};
use crate::AppState;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use super::common::REFRESH_INTERVAL;

/// Filter movers to US-only stocks (symbols without dots or with US exchange suffixes)
fn filter_us_movers(movers: Vec<crate::models::movers::MarketMover>) -> Vec<crate::models::movers::MarketMover> {
    movers
        .into_iter()
        .filter(|mover| {
            // US stocks typically:
            // - Don't have dots (e.g., AAPL, MSFT)
            // - Or have specific US exchange suffixes like .OB (OTC), .PK (Pink Sheets)
            // - Exclude foreign exchanges like .TO (Toronto), .L (London), .T (Tokyo), etc.
            let symbol = &mover.symbol;
            !symbol.contains('.') || 
            symbol.ends_with(".OB") || 
            symbol.ends_with(".PK") ||
            symbol.ends_with(".OTC")
        })
        .collect()
}

/// Movers WebSocket endpoint - streams actives, gainers, losers (US-only)
pub async fn movers_handler(
    req: HttpRequest,
    body: web::Payload,
    app_state: web::Data<AppState>,
    manager: web::Data<ConnectionManagerAddr>,
) -> Result<HttpResponse> {
    info!("WebSocket movers connection established");
    
    let channel = "movers".to_string();
    
    // Start background task to fetch and broadcast data
    let manager_clone = manager.get_ref().clone();
    let channel_clone = channel.clone();
    let yahoo_client_clone = app_state.yahoo_client.clone();
    
    // Create data fetching task
    let task = tokio::spawn(async move {
        loop {
            // Clone Arc values for each iteration to avoid holding references across await
            let yahoo = yahoo_client_clone.clone();
            
            let data_result = fetch_movers_data(&yahoo).await;
            
            match data_result {
                Ok(data) => {
                    manager_clone.do_send(crate::service::websocket::BroadcastMessage {
                        channel: channel_clone.clone(),
                        message: data,
                    });
                }
                Err(e) => {
                    error!("Failed to fetch movers data: {}", e);
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

async fn fetch_movers_data(
    yahoo_client: &Arc<crate::client::YahooFinanceClient>,
) -> Result<Value, crate::client::error::YahooError> {
    use crate::service;
    use crate::models::movers::MoverCount;
    
    let actives_task = service::get_actives(yahoo_client, MoverCount::Fifty);
    let gainers_task = service::get_gainers(yahoo_client, MoverCount::Fifty);
    let losers_task = service::get_losers(yahoo_client, MoverCount::Fifty);
    
    let (actives_result, gainers_result, losers_result) = tokio::join!(
        actives_task,
        gainers_task,
        losers_task
    );
    
    // Filter movers to US-only
    let actives = actives_result.ok().map(|a| filter_us_movers(a)).map(|a| json!(a));
    let gainers = gainers_result.ok().map(|g| filter_us_movers(g)).map(|g| json!(g));
    let losers = losers_result.ok().map(|l| filter_us_movers(l)).map(|l| json!(l));
    
    Ok(json!({
        "actives": actives,
        "gainers": gainers,
        "losers": losers,
    }))
}