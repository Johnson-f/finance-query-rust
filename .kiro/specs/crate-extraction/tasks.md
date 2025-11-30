# Implementation Plan

- [x] 1. Configure Cargo workspace
  - [x] 1.1 Update root Cargo.toml to define workspace with members
    - Add `[workspace]` section with members `["."]` and `["finance-query-core"]`
    - Add `[workspace.dependencies]` for shared dependencies (reqwest, serde, chrono, thiserror, tokio, etc.)
    - _Requirements: 6.1, 6.2_
  - [x] 1.2 Update finance-query-core/Cargo.toml with proper metadata and dependencies
    - Fix edition to "2021" (not "2024")
    - Add description, license, repository, keywords, categories
    - Add dependencies using `workspace = true` syntax
    - _Requirements: 5.1, 5.2_

- [x] 2. Set up core crate structure and error types
  - [x] 2.1 Create framework-agnostic error type in finance-query-core
    - Copy `src/client/error.rs` to `finance-query-core/src/client/error.rs`
    - Remove `actix_web` imports and `ResponseError` implementation
    - Keep only `thiserror` derive
    - _Requirements: 1.4_
  - [x] 2.2 Create client module structure
    - Create `finance-query-core/src/client/mod.rs` with public exports
    - _Requirements: 1.2, 1.3_

- [x] 3. Move client components to core crate
  - [x] 3.1 Move FetchClient to core crate
    - Copy `src/client/fetch_client.rs` to `finance-query-core/src/client/fetch_client.rs`
    - Update imports to use local error module
    - _Requirements: 1.3_
  - [x] 3.2 Move YahooAuthManager to core crate
    - Copy `src/client/yahoo_auth.rs` to `finance-query-core/src/client/yahoo_auth.rs`
    - Update imports to use local error module
    - _Requirements: 1.3, 3.1_
  - [x] 3.3 Move YahooFinanceClient to core crate
    - Copy `src/client/yahoo_client.rs` to `finance-query-core/src/client/yahoo_client.rs`
    - Update imports to use local modules
    - _Requirements: 1.3, 3.1, 3.2, 3.3_
  - [x] 3.4 Move scraper to core crate
    - Copy `src/client/scraper.rs` to `finance-query-core/src/client/scraper.rs`
    - Update imports
    - _Requirements: 1.2_

- [x] 4. Move models to core crate
  - [x] 4.1 Move quote models
    - Copy `src/models/quote.rs` to `finance-query-core/src/models/quote.rs`
    - Ensure Debug, Clone, Serialize, Deserialize derives
    - _Requirements: 2.1, 2.2, 2.3_
  - [x] 4.2 Move historical models
    - Copy `src/models/historical.rs` to `finance-query-core/src/models/historical.rs`
    - _Requirements: 2.1, 2.2, 2.3_
  - [x] 4.3 Move remaining models (news, search, financials, movers, indices, holders, analysts, sectors, earnings_transcripts)
    - Copy all remaining model files to `finance-query-core/src/models/`
    - _Requirements: 2.1, 2.2, 2.3_
  - [x] 4.4 Create models module with re-exports
    - Create `finance-query-core/src/models/mod.rs` with all public exports
    - _Requirements: 1.2, 2.1_

- [x] 5. Move utilities to core crate
  - [x] 5.1 Move utils module
    - Copy `src/utils/` to `finance-query-core/src/utils/`
    - Create mod.rs with exports
    - _Requirements: 1.2_

- [x] 6. Create core crate lib.rs with public API
  - [x] 6.1 Create lib.rs with module declarations and re-exports
    - Declare client, models, utils modules
    - Re-export key types: YahooFinanceClient, YahooAuthManager, FetchClient, YahooError
    - _Requirements: 1.2, 1.3_
  - [x] 6.2 Write property test for model serialization round-trip
    - **Property 1: Model Serialization Round-Trip**
    - **Validates: Requirements 2.2**
    - Add proptest dev-dependency
    - Test SimpleQuote, Quote, HistoricalData serialization round-trips

- [x] 7. Checkpoint - Verify core crate compiles
  - Ensure all tests pass, ask the user if questions arise.
  - Write test for all models, make sure all test pass

- [-] 8. Update main application to use core crate
  - [x] 8.1 Add finance-query-core as dependency in root Cargo.toml
    - Add `finance-query-core = { path = "finance-query-core" }`
    - _Requirements: 4.1_
  - [x] 8.2 Create web error adapter in main app
    - Create `src/error.rs` implementing `ResponseError` for `YahooError`
    - Import `YahooError` from `finance_query_core`
    - _Requirements: 4.4_
  - [x] 8.3 Update main app client imports
    - Replace `crate::client::*` with `finance_query_core::client::*`
    - Update all files in `src/service/`, `src/routes/`, `src/graphql/`
    - _Requirements: 4.2_
  - [x] 8.4 Update main app model imports
    - Replace `crate::models::*` with `finance_query_core::models::*`
    - Update all files using models
    - _Requirements: 4.3_
  - [-] 8.5 Remove old client and models directories from main app
    - Delete `src/client/` directory
    - Delete `src/models/` directory
    - Update `src/main.rs` to remove old module declarations
    - _Requirements: 4.2, 4.3_

  - [x] 8.6 Remove old client and models directories from main app
    - Integrate websocket support in the crate, search the web on how do to this. ProfileUpdate, MoversUpdate, MarketHours, simple-quotes
    - Put the code on the websocket folder on the finance-query-core
- [x] 9. Checkpoint - Verify workspace compiles
  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. Add crate publishing metadata
  - [x] 10.1 Create README.md for core crate
    - Add usage examples showing client initialization and API calls
    - Document available models and error types
    - _Requirements: 5.3_
  - [x] 10.2 Copy LICENSE file to core crate
    - Copy root LICENSE to `finance-query-core/LICENSE`
    - _Requirements: 5.4_

- [ ] 11. Final Checkpoint - Verify everything works
  - Ensure all tests pass, ask the user if questions arise.
