# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-12-09

### Changed
- Redesigned Yahoo authentication to stay headless (no login page) with dual crumb strategies: basic `fc.yahoo.com`/`query1` path and consent/CSRF `guce`/`query2` fallback, automatically switching on auth/429 errors.
- Hardened refresh behavior with cached crumb reuse, minimum refresh interval, and clearer logging/errors to surface invalid crumbs.
- Proxy-friendly auth and example updates: examples now respect `PROXY_URL`, and auth requests accept proxy certificates to support VPS/proxy deployments.

## [0.2.0] - 2025-12-01

### Added

#### Market Movers
- **`get_movers()`** - Fetch market movers (most active, top gainers, top losers)
- **`MoversStream`** - Real-time streaming of market movers with configurable intervals
- **`MoverCount`** enum - Control number of movers (25, 50, or 100 per category)
- **`MoversUpdate`** WebSocket type with timestamp support
- US stock filtering (automatically excludes international stocks)

#### Stock Actions
- **`get_actions()`** - Fetch dividends, splits, and capital gains
- **`get_dividends()`** - Get dividend history
- **`get_splits()`** - Get stock split history
- **`get_capital_gains()`** - Get capital gains distributions
- **`ActionsResponse`** model with helper methods

#### Options Data
- **`get_option_chain()`** - Fetch option chains for specific expiration dates
- **`get_option_expirations()`** - Get all available expiration dates
- **`OptionChain`** model with calls and puts
- **`OptionContract`** model with Greeks and implied volatility

#### Calendar & Events
- **`get_calendar()`** - Fetch earnings dates, dividend dates, and other events
- **`Calendar`** model with date parsing

#### SEC Filings
- **`get_sec_filings()`** - Fetch SEC filings (10-K, 10-Q, 8-K, etc.)
- **`SecFilingsResponse`** model with filing details and exhibits

#### ESG/Sustainability
- **`get_sustainability()`** - Fetch ESG scores and ratings
- **`SustainabilityScores`** model with environmental, social, and governance metrics

#### Industry Data
- **`get_industry()`** - Fetch industry performance and top companies
- **`Industry`** model with sector information

#### Market Status
- **`get_market_status()`** - Check if market is open/closed
- **`get_market_summary()`** - Get market summary with major indices
- **`MarketStatus`** and **`MarketSummaryResponse`** models

#### Streaming Enhancements
- **`IndexStream`** - Stream market indices in real-time
- **`QuoteStream`** improvements with better error handling
- **`SingleQuoteStream`** for individual symbol streaming

#### WebSocket Types
- Enhanced **`QuotesUpdate`** with helper methods (`contains_symbol`, `get_quote`, `is_empty`, `len`)
- Updated **`MoversUpdate`** with required fields and timestamp
- **`ProfileUpdate`** for comprehensive stock profiles
- **`MarketHours`** for market status updates
- **`MovingAverageUpdate`** for technical indicators

### Changed
- Improved error handling across all streaming types
- Better documentation with comprehensive examples
- Enhanced type safety with strongly-typed models

### Documentation
- Added comprehensive streaming guides
- Created model documentation for all new types
- Updated README with v0.2.0 features
- Added examples for all new functionality

## [0.1.0] - 2024-11-XX

### Added
- Initial release
- Basic quote fetching
- Historical data
- News
- Search functionality
- Financial statements
- Holders data
- Analyst recommendations
- Earnings transcripts
- Sector performance
- Basic streaming support

[0.3.0]: https://github.com/johnson-f/finance-query-rust/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/johnson-f/finance-query-rust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/johnson-f/finance-query-rust/releases/tag/v0.1.0
