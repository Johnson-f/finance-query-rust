use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum YahooError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("HTTP error: {0}")]
    HttpError(u16, String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

impl ResponseError for YahooError {
    fn error_response(&self) -> HttpResponse {
        match self {
            YahooError::AuthFailed(_) => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication failed",
                "message": self.to_string()
            })),
            YahooError::NotFound(_) => HttpResponse::NotFound().json(serde_json::json!({
                "error": "Not found",
                "message": self.to_string()
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
                    "message": self.to_string()
                }))
            }
            YahooError::ParseError(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Parse error",
                "message": self.to_string()
            })),
            YahooError::NetworkError(_) => HttpResponse::BadGateway().json(serde_json::json!({
                "error": "Network error",
                "message": self.to_string()
            })),
        }
    }
}