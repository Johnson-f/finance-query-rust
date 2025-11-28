use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_ws::handle;
use crate::service::websocket::{ConnectionManagerAddr, handle_moving_average_websocket_session};
use crate::AppState;
use tracing::info;

/// Moving average WebSocket endpoint - streams real-time moving averages
/// Client must send subscription requests as JSON: {"symbol": "AAPL", "type": "sma", "period": 20}
pub async fn moving_average_handler(
    req: HttpRequest,
    body: web::Payload,
    app_state: web::Data<AppState>,
    manager: web::Data<ConnectionManagerAddr>,
) -> Result<HttpResponse> {
    info!("Moving average WebSocket connection established");
    
    let session_id = req.peer_addr().map(|a| a.port() as usize).unwrap_or(0);
    let manager_addr = manager.get_ref().clone();
    let yahoo_client = app_state.yahoo_client.clone();
    let fetch_client = app_state.fetch_client.clone();
    let price_buffer = app_state.price_buffer_manager.clone();
    
    let (response, session, msg_stream) = handle(&req, body)?;
    
    // Spawn the WebSocket handler
    actix_web::rt::spawn(async move {
        handle_moving_average_websocket_session(
            session,
            msg_stream,
            session_id,
            manager_addr,
            yahoo_client,
            fetch_client,
            price_buffer,
        ).await;
    });
    
    Ok(response)
}

