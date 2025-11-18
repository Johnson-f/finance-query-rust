use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_ws::handle;
use crate::service::websocket::{ConnectionManagerAddr, handle_quotes_websocket_session};
use crate::AppState;
use tracing::info;

/// Quotes WebSocket endpoint - streams simple quotes for multiple symbols
/// Client must send comma-separated symbols as the first message (e.g., "AAPL,MSFT,GOOGL")
pub async fn quotes_handler(
    req: HttpRequest,
    body: web::Payload,
    app_state: web::Data<AppState>,
    manager: web::Data<ConnectionManagerAddr>,
) -> Result<HttpResponse> {
    info!("WebSocket quotes connection established, waiting for symbols from client");
    
    let session_id = req.peer_addr().map(|a| a.port() as usize).unwrap_or(0);
    let manager_addr = manager.get_ref().clone();
    let yahoo_client = app_state.yahoo_client.clone();
    let fetch_client = app_state.fetch_client.clone();
    
    let (response, session, msg_stream) = handle(&req, body)?;
    
    // Spawn the WebSocket handler
    actix_web::rt::spawn(async move {
        handle_quotes_websocket_session(session, msg_stream, session_id, manager_addr, yahoo_client, fetch_client).await;
    });
    
    Ok(response)
}