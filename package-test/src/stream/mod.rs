//! Streaming functionality for real-time financial data.
//!
//! This module provides examples of using the streaming APIs from finance-query-core
//! to continuously fetch financial data at configurable intervals.

pub mod index;
pub mod mover;
pub mod quote;

pub use index::*;
pub use mover::*;
pub use quote::*;

// Re-export commonly used types for convenience
pub use finance_query_core::{MoverCount, QuoteStream, IndexStream, MoversStream, SingleQuoteStream};
