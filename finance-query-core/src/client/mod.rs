//! Yahoo Finance client module.
//!
//! This module provides the HTTP client components for interacting with
//! Yahoo Finance APIs, including authentication management and error handling.

pub mod error;
pub mod fetch_client;
pub mod scraper;
pub mod yahoo_auth;
pub mod yahoo_client;

// Re-export types for convenience
pub use error::YahooError;
pub use fetch_client::FetchClient;
pub use yahoo_auth::YahooAuthManager;
pub use yahoo_client::YahooFinanceClient;
