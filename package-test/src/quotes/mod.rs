//! Quotes module for fetching stock quote data

mod quotes;

pub use quotes::{
    // Functions
    get_detailed_quote,
    get_simple_quotes,
    get_similar_quotes,
    get_logo_url,
    // Detailed Quote Types
    DetailedQuoteResponse,
    // Simple Quote Types
    SimpleQuotesResponse,
    QuoteResponse,
    SimpleQuoteResult,
    // Similar Quotes Types
    SimilarQuotesResponse,
    FinanceResult,
    RecommendationResult,
    RecommendedSymbol,
    // Common Types
    FormattedValue,
};
