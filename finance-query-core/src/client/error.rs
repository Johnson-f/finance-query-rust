//! Error types for Yahoo Finance client operations.
//!
//! This module provides framework-agnostic error types that can be used
//! in any Rust application without web framework dependencies.

/// Error type for Yahoo Finance client operations.
///
/// This enum represents all possible errors that can occur when
/// interacting with the Yahoo Finance API.
#[derive(Debug, thiserror::Error)]
pub enum YahooError {
    /// Authentication with Yahoo Finance failed.
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    /// The requested resource was not found.
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Rate limit has been exceeded.
    #[error("Rate limit exceeded")]
    RateLimited,

    /// HTTP error with status code and message.
    #[error("HTTP error: {0}")]
    HttpError(u16, String),

    /// Failed to parse response data.
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Network-level error from reqwest.
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
