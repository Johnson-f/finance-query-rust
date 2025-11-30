//! Web error adapter for Yahoo Finance client errors.
//!
//! This module provides actix-web `ResponseError` implementation for
//! the framework-agnostic `YahooError` from finance-query-core.

use actix_web::{HttpResponse, ResponseError};
use finance_query_core::YahooError;

/// Wrapper type to implement ResponseError for YahooError.
///
/// Since we can't implement foreign traits on foreign types directly,
/// we use a newtype wrapper pattern.
#[derive(Debug)]
pub struct WebError(pub YahooError);

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for WebError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl ResponseError for WebError {
    fn error_response(&self) -> HttpResponse {
        match &self.0 {
            YahooError::AuthFailed(_) => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication failed",
                "message": self.0.to_string()
            })),
            YahooError::NotFound(_) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "Not found",
                "message": self.0.to_string()
            })),
            YahooError::RateLimited => HttpResponse::TooManyRequests().json(serde_json::json!({
                "error": "Rate limit exceeded",
                "message": "Too many requests"
            })),
            YahooError::HttpError(status, _) => {
                let status_code = actix_web::http::StatusCode::from_u16(*status)
                    .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
                HttpResponse::build(status_code).json(serde_json::json!({
                    "error": "HTTP error",
                    "message": self.0.to_string()
                }))
            }
            YahooError::ParseError(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Parse error",
                "message": self.0.to_string()
            })),
            YahooError::NetworkError(_) => HttpResponse::BadGateway().json(serde_json::json!({
                "error": "Network error",
                "message": self.0.to_string()
            })),
        }
    }
}

impl From<YahooError> for WebError {
    fn from(err: YahooError) -> Self {
        WebError(err)
    }
}

/// Extension trait to convert Result<T, YahooError> to Result<T, actix_web::Error>
pub trait IntoWebResult<T> {
    fn into_web_result(self) -> Result<T, actix_web::Error>;
}

impl<T> IntoWebResult<T> for Result<T, YahooError> {
    fn into_web_result(self) -> Result<T, actix_web::Error> {
        self.map_err(|e| WebError(e).into())
    }
}
