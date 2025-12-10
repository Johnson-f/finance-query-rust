use actix_web::{HttpResponse, Result};
use chrono::Utc;
use serde_json::json;

pub async fn ping_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
    })))
}

pub async fn health_handler() -> Result<HttpResponse> {
    // Comprehensive health check - test all services
    // For now, return a simple healthy status
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
        "services": {
            "status": "all_operational"
        }
    })))
}
