mod client;
mod middleware;
mod models;
mod routes;
mod service;
mod utils;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use client::{FetchClient, YahooAuthManager, YahooFinanceClient};
use middleware::rate_limit::RateLimitManager;
use service::caching::CacheService;
use service::websocket::ConnectionManager;
use service::websocket::indicator::price_buffer::PriceBufferManager;
use std::sync::Arc;
use tracing::info;
use tracing_actix_web::TracingLogger;

pub struct AppState {
    pub yahoo_auth_manager: Arc<YahooAuthManager>,
    pub fetch_client: Arc<FetchClient>,
    pub yahoo_client: Arc<YahooFinanceClient>,
    pub connection_manager: web::Data<service::websocket::ConnectionManagerAddr>,
    pub cache_service: Arc<CacheService>,
    pub rate_limit_manager: Arc<RateLimitManager>,
    pub price_buffer_manager: Arc<PriceBufferManager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenv::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Finance Query Rust server...");

    // Get proxy from environment (optional)
    let proxy = std::env::var("PROXY_URL").ok();

    // Initialize fetch client
    let fetch_client = Arc::new(
        FetchClient::new(proxy.clone())
            .expect("Failed to create fetch client"),
    );

    // Initialize Yahoo auth manager with shared cookie jar from fetch client
    let yahoo_auth_manager = Arc::new(YahooAuthManager::new(
        proxy.clone(),
        fetch_client.cookie_jar().clone(),
    ));

    // Prime authentication on startup
    info!("Priming Yahoo authentication...");
    if let Err(e) = yahoo_auth_manager.refresh().await {
        eprintln!("Warning: Failed to prime Yahoo authentication: {}", e);
        eprintln!("Server will continue, but first request may be slower");
    } else {
        info!("Yahoo authentication primed successfully");
    }

    // Initialize Yahoo Finance client
    let yahoo_client = Arc::new(YahooFinanceClient::new(
        yahoo_auth_manager.clone(),
        fetch_client.clone(),
    ));

    // Initialize WebSocket connection manager
    let connection_manager = ConnectionManager::default().start();
    let connection_manager_data = web::Data::new(connection_manager);

    // Initialize Redis cache service
    let redis_url = std::env::var("REDIS_URL").ok();
    let cache_service = Arc::new(CacheService::new(redis_url.clone()).await);

    // Load rate limit configuration from environment
    let rate_limit_per_day = std::env::var("RATE_LIMIT_PER_DAY")
        .ok()
        .and_then(|v| v.parse::<u64>().ok());

    // Initialize rate limit manager
    let rate_limit_manager = Arc::new(
        RateLimitManager::new(redis_url.clone(), rate_limit_per_day).await
    );
    
    info!(
        "Rate limiting configured: {} requests per day per IP",
        rate_limit_manager.limit_per_day()
    );

    // Initialize price buffer manager for moving averages
    let price_buffer_manager = Arc::new(PriceBufferManager::new(1000));

    // Create app state
    let app_state = web::Data::new(AppState {
        yahoo_auth_manager,
        fetch_client,
        yahoo_client,
        connection_manager: connection_manager_data.clone(),
        cache_service,
        rate_limit_manager: rate_limit_manager.clone(),
        price_buffer_manager,
    });

    info!("Starting HTTP server on 0.0.0.0:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .app_data(connection_manager_data.clone())
            .wrap(cors)
            .wrap(TracingLogger::default())
            .wrap(middleware::rate_limit::RateLimitMiddleware::new(rate_limit_manager.clone()))
            .configure(routes::configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}