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
pub use client::{FetchClient, YahooAuthManager, YahooError, YahooFinanceClient};

// Re-export websocket types
pub use websocket::{MarketHours, MoversUpdate, MovingAverageUpdate, ProfileUpdate, QuotesUpdate};

// Re-export streaming types
pub use streaming::{IndexStream, MoversStream, QuoteStream, SingleQuoteStream};

// Re-export quote models
pub use models::{DetailedQuote, Quote, SimpleQuote};

// Re-export historical models
pub use models::{HistoricalData, HistoricalResponse, IndicatorType, Interval, TimeRange};

// Re-export news models
pub use models::News;

// Re-export search models
pub use models::{SearchResponse, SearchResult};

// Re-export financial models
pub use models::{FinancialStatement, Frequency, StatementType};

// Re-export movers models
pub use models::{MarketMover, MoverCount};

// Re-export indices models
pub use models::{get_index_regions, Index, MarketIndex, Region};

// Re-export holders models
pub use models::{
    HolderType, HoldersData, InsiderPurchase, InsiderPurchasesResponse, InsiderRosterMember,
    InsiderRosterResponse, InsiderTransaction, InsiderTransactionsResponse, InstitutionalHolder,
    InstitutionalHoldersResponse, MajorHoldersBreakdown, MajorHoldersResponse, MutualFundHolder,
    MutualFundHoldersResponse,
};

// Re-export analysts models
pub use models::{
    AnalysisType, EarningsEstimate, EarningsEstimateResponse, EarningsHistoryItem,
    EarningsHistoryResponse, EpsRevisions, EpsRevisionsResponse, EpsTrend, EpsTrendResponse,
    GrowthEstimate, GrowthEstimatesResponse, PriceTarget, PriceTargetsResponse, RecommendationData,
    RecommendationsResponse, RevenueEstimate, RevenueEstimateResponse, UpgradeDowngrade,
    UpgradesDowngradesResponse,
};

// Re-export sectors models
pub use models::{MarketSector, MarketSectorDetails, Sector};

// Re-export earnings transcripts models
pub use models::{
    EarningsCallListing, EarningsCallsList, EarningsTranscript, Quarter, TranscriptParagraph,
    TranscriptSpeaker,
};

// Re-export actions models
pub use models::{ActionsResponse, CapitalGain, Dividend, StockSplit};

// Re-export options models
pub use models::{OptionChain, OptionContract, OptionExpirations};

// Re-export calendar models
pub use models::Calendar;

// Re-export SEC filings models
pub use models::{SecExhibit, SecFiling, SecFilingsResponse};

// Re-export sustainability/ESG models
pub use models::SustainabilityScores;

// Re-export industry models
pub use models::{Industry, IndustryCompany};

// Re-export market models
pub use models::{MarketStatus, MarketSummaryItem, MarketSummaryResponse};
