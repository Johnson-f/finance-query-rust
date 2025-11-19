use actix_web::{
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::{header::HeaderValue, StatusCode},
    Error,
};
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
    sync::Arc,
};
use tracing::{error, warn};

const DEFAULT_RATE_LIMIT_PER_DAY: u64 = 10_000;
const RATE_LIMIT_TTL_SECONDS: u64 = 86_400; // 24 hours

#[derive(Clone)]
pub struct RateLimitManager {
    connection: Option<Arc<ConnectionManager>>,
    limit_per_day: u64,
}

#[derive(Serialize, Deserialize)]
struct RateLimitInfo {
    count: u64,
    limit: u64,
    reset_in: u64,
}

impl RateLimitManager {
    pub async fn new(redis_url: Option<String>, limit_per_day: Option<u64>) -> Self {
        let limit = limit_per_day.unwrap_or(DEFAULT_RATE_LIMIT_PER_DAY);
        
        let connection = if let Some(url) = redis_url {
            match redis::Client::open(url.as_str()) {
                Ok(client) => {
                    match redis::aio::ConnectionManager::new(client).await {
                        Ok(conn) => {
                            tracing::info!("Rate limiting using Redis");
                            Some(Arc::new(conn))
                        }
                        Err(e) => {
                            warn!("Failed to connect to Redis for rate limiting: {}. Rate limiting will be disabled.", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to create Redis client for rate limiting: {}. Rate limiting will be disabled.", e);
                    None
                }
            }
        } else {
            warn!("REDIS_URL not set. Rate limiting will be disabled.");
            None
        };

        Self {
            connection,
            limit_per_day: limit,
        }
    }

    pub async fn check_and_increment(&self, ip: &str) -> Result<RateLimitResult, RateLimitError> {
        if self.connection.is_none() {
            // If Redis is not available, allow the request (graceful degradation)
            return Ok(RateLimitResult {
                allowed: true,
                count: 0,
                remaining: self.limit_per_day,
                reset_in: RATE_LIMIT_TTL_SECONDS,
            });
        }

        let key = format!("rate_limit:{}", ip);
        let conn = self.connection.as_ref().unwrap();
        let mut conn = (**conn).clone();

        // Get current count
        let count: u64 = match conn.get(&key).await {
            Ok(val) => val,
            Err(e) if e.kind() == redis::ErrorKind::TypeError => {
                // Key doesn't exist, start at 0
                0
            }
            Err(e) => {
                error!("Redis error getting rate limit for {}: {}", ip, e);
                // On Redis error, allow the request (graceful degradation)
                return Ok(RateLimitResult {
                    allowed: true,
                    count: 0,
                    remaining: self.limit_per_day,
                    reset_in: RATE_LIMIT_TTL_SECONDS,
                });
            }
        };

        // Check if limit exceeded
        if count >= self.limit_per_day {
            // Get TTL to calculate reset time
            let ttl: i64 = conn.ttl(&key).await.unwrap_or(RATE_LIMIT_TTL_SECONDS as i64);
            let reset_in = if ttl > 0 { ttl as u64 } else { RATE_LIMIT_TTL_SECONDS };

            return Ok(RateLimitResult {
                allowed: false,
                count,
                remaining: 0,
                reset_in,
            });
        }

        // Increment count
        let new_count = count + 1;
        match conn.set_ex::<_, _, ()>(&key, new_count, RATE_LIMIT_TTL_SECONDS).await {
            Ok(_) => {
                Ok(RateLimitResult {
                    allowed: true,
                    count: new_count,
                    remaining: self.limit_per_day.saturating_sub(new_count),
                    reset_in: RATE_LIMIT_TTL_SECONDS,
                })
            }
            Err(e) => {
                error!("Redis error setting rate limit for {}: {}", ip, e);
                // On Redis error, allow the request (graceful degradation)
                Ok(RateLimitResult {
                    allowed: true,
                    count: new_count,
                    remaining: self.limit_per_day.saturating_sub(new_count),
                    reset_in: RATE_LIMIT_TTL_SECONDS,
                })
            }
        }
    }

    pub fn limit_per_day(&self) -> u64 {
        self.limit_per_day
    }
}

#[derive(Debug)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub count: u64,
    pub remaining: u64,
    pub reset_in: u64,
}

#[derive(Debug)]
pub enum RateLimitError {
    RedisError(String),
}

pub struct RateLimitMiddleware {
    rate_limit_manager: Arc<RateLimitManager>,
}

impl RateLimitMiddleware {
    pub fn new(rate_limit_manager: Arc<RateLimitManager>) -> Self {
        Self {
            rate_limit_manager,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddlewareService {
            service: Rc::new(service),
            rate_limit_manager: self.rate_limit_manager.clone(),
        }))
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: Rc<S>,
    rate_limit_manager: Arc<RateLimitManager>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip rate limiting for /ping and /health endpoints
        let path = req.path();
        if path == "/ping" || path == "/health" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                fut.await.map(|res| res.map_into_boxed_body())
            });
        }

        // Extract client IP before moving req
        let ip = extract_client_ip(&req);
        let rate_limit_manager = self.rate_limit_manager.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // Check rate limit first (before processing request)
            let rate_limit_result = rate_limit_manager.check_and_increment(&ip).await;
            
            match rate_limit_result {
                Ok(result) => {
                    if !result.allowed {
                        // Rate limit exceeded - create error response without processing request
                        let (req, _) = req.into_parts();
                        let resp = actix_web::HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                            .json(serde_json::json!({
                                "detail": "Rate limit exceeded",
                                "rate_limit_info": {
                                    "count": result.count,
                                    "remaining": result.remaining,
                                    "reset_in": result.reset_in,
                                    "limit": rate_limit_manager.limit_per_day(),
                                }
                            }));
                        let resp = ServiceResponse::new(req, resp).map_into_boxed_body();
                        return Ok(resp);
                    }

                    // Rate limit OK - process the request
                    let fut = service.call(req);
                    let mut res = fut.await?.map_into_boxed_body();

                    // Add rate limit headers
                    res.headers_mut().insert(
                        actix_web::http::header::HeaderName::from_static("x-ratelimit-limit"),
                        HeaderValue::from_str(
                            &rate_limit_manager.limit_per_day().to_string(),
                        )
                        .unwrap(),
                    );
                    res.headers_mut().insert(
                        actix_web::http::header::HeaderName::from_static("x-ratelimit-remaining"),
                        HeaderValue::from_str(&result.remaining.to_string())
                            .unwrap(),
                    );
                    res.headers_mut().insert(
                        actix_web::http::header::HeaderName::from_static("x-ratelimit-reset"),
                        HeaderValue::from_str(&result.reset_in.to_string())
                            .unwrap(),
                    );

                    Ok(res)
                }
                Err(e) => {
                    error!("Rate limit error: {:?}", e);
                    // On error, allow the request (graceful degradation)
                    let fut = service.call(req);
                    fut.await.map(|res| res.map_into_boxed_body())
                }
            }
        })
    }
}

fn extract_client_ip(req: &ServiceRequest) -> String {
    // Check X-Forwarded-For header first (for proxies/load balancers)
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            // X-Forwarded-For can contain multiple IPs, take the first one
            if let Some(first_ip) = forwarded_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }

    // Check X-Real-IP header (alternative proxy header)
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            return real_ip_str.to_string();
        }
    }

    // Fall back to connection info
    if let Some(peer_addr) = req.peer_addr() {
        return peer_addr.ip().to_string();
    }

    // Last resort: use a default
    "unknown".to_string()
}