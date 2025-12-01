//! # finance-query-core
//!
//! A Rust client library for Yahoo Finance API.
//!
//! This crate provides a framework-agnostic client for fetching financial data
//! from Yahoo Finance, including quotes, historical data, financials, and more.
//!
//! ## Features
//!
//! - Framework-agnostic design - use with any Rust application
//! - Automatic authentication handling with cookie/crumb management
//! - Strongly-typed data models for all Yahoo Finance responses
//! - Comprehensive error handling
//!
//! ## Example
//!
//! ```rust,ignore
//! use finance_query_core::{YahooFinanceClient, YahooAuthManager, FetchClient, TimeRange, Interval};
//! use std::sync::Arc;
//!
//! // Create client components
//! let fetch_client = Arc::new(FetchClient::new());
//! let auth_manager = Arc::new(YahooAuthManager::new(fetch_client.clone()));
//! let client = YahooFinanceClient::new(auth_manager, fetch_client);
//!
//! // Fetch a quote
//! let quote = client.get_quote("AAPL").await?;
//! ```

pub mod client;
pub mod models;
pub mod streaming;
pub mod utils;
pub mod websocket;

// Re-export client types
pub use client::{FetchClient, YahooAuthManager, YahooFinanceClient, YahooError};

// Re-export websocket types
pub use websocket::{QuotesUpdate, ProfileUpdate, MoversUpdate, MarketHours, MovingAverageUpdate};

// Re-export streaming types
pub use streaming::{QuoteStream, SingleQuoteStream, IndexStream, MoversStream};

// Re-export quote models
pub use models::{Quote, SimpleQuote, DetailedQuote};

// Re-export historical models
pub use models::{HistoricalData, HistoricalResponse, TimeRange, Interval, IndicatorType};

// Re-export news models
pub use models::News;

// Re-export search models
pub use models::{SearchResult, SearchResponse};

// Re-export financial models
pub use models::{FinancialStatement, StatementType, Frequency};

// Re-export movers models
pub use models::{MoverCount, MarketMover};

// Re-export indices models
pub use models::{Region, Index, MarketIndex, get_index_regions};

// Re-export holders models
pub use models::{
    HolderType, MajorHoldersBreakdown, InstitutionalHolder, MutualFundHolder,
    InsiderTransaction, InsiderPurchase, InsiderRosterMember,
    MajorHoldersResponse, InstitutionalHoldersResponse, MutualFundHoldersResponse,
    InsiderTransactionsResponse, InsiderPurchasesResponse, InsiderRosterResponse,
    HoldersData,
};

// Re-export analysts models
pub use models::{
    AnalysisType, RecommendationData, UpgradeDowngrade, PriceTarget,
    EarningsEstimate, RevenueEstimate, EarningsHistoryItem,
    RecommendationsResponse, UpgradesDowngradesResponse, PriceTargetsResponse,
    EarningsEstimateResponse, RevenueEstimateResponse, EarningsHistoryResponse,
    EpsTrend, EpsRevisions, GrowthEstimate,
    EpsTrendResponse, EpsRevisionsResponse, GrowthEstimatesResponse,
};

// Re-export sectors models
pub use models::{Sector, MarketSector, MarketSectorDetails};

// Re-export earnings transcripts models
pub use models::{
    Quarter, EarningsCallListing, EarningsCallsList, TranscriptSpeaker,
    TranscriptParagraph, EarningsTranscript,
};

// Re-export actions models
pub use models::{ActionsResponse, Dividend, StockSplit, CapitalGain};

// Re-export options models
pub use models::{OptionChain, OptionContract, OptionExpirations};

// Re-export calendar models
pub use models::Calendar;

// Re-export SEC filings models
pub use models::{SecFiling, SecFilingsResponse, SecExhibit};

// Re-export sustainability/ESG models
pub use models::SustainabilityScores;

// Re-export industry models
pub use models::{Industry, IndustryCompany};

// Re-export market models
pub use models::{MarketStatus, MarketSummaryItem, MarketSummaryResponse};
