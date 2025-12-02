//! News data module for fetching financial news articles

mod news;

pub use news::{
    // Functions
    get_news_for_symbol,
    get_news_for_symbols,
    get_news_raw,
    get_market_news,
    get_sector_news,
    get_trending_news,
    search_news,
    // Types
    NewsArticle,
    NewsResponse,
};
