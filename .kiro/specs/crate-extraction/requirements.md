# Requirements Document

## Introduction

This specification defines the extraction of reusable Yahoo Finance API client functionality from the `finance-query-rust` web application into a standalone, publishable Rust crate called `finance-query-core`. The core crate will provide a clean, framework-agnostic API for interacting with Yahoo Finance, while the main application will become a thin web layer consuming this crate.

## Glossary

- **Core Crate**: The `finance-query-core` library containing reusable Yahoo Finance client functionality
- **Main Application**: The `finance-query-rust` web server that consumes the core crate
- **Yahoo Client**: The HTTP client responsible for authenticated requests to Yahoo Finance APIs
- **Auth Manager**: Component handling Yahoo Finance cookie/crumb authentication
- **Models**: Data structures representing Yahoo Finance API responses (quotes, historical data, etc.)

## Requirements

### Requirement 1

**User Story:** As a Rust developer, I want to use the Yahoo Finance client as a standalone library, so that I can integrate it into my own applications without the web server dependencies.

#### Acceptance Criteria

1. WHEN a developer adds `finance-query-core` as a dependency THEN the Core Crate SHALL compile without requiring actix-web, async-graphql, or other web framework dependencies
2. WHEN the Core Crate is built THEN the Core Crate SHALL expose a public API through `lib.rs` that re-exports all client, model, and utility types
3. WHEN a developer imports the Core Crate THEN the Core Crate SHALL provide the `YahooFinanceClient`, `YahooAuthManager`, and `FetchClient` types
4. WHEN the Core Crate defines error types THEN the Core Crate SHALL use `thiserror` for error definitions without actix-web `ResponseError` implementations

### Requirement 2

**User Story:** As a library consumer, I want well-defined data models, so that I can work with strongly-typed Yahoo Finance data.

#### Acceptance Criteria

1. WHEN the Core Crate exports models THEN the Core Crate SHALL include all quote, historical, news, search, financials, movers, indices, holders, analysts, sectors, and earnings transcript models
2. WHEN models are serialized or deserialized THEN the Core Crate SHALL implement `Serialize` and `Deserialize` traits using serde
3. WHEN a model is used in client code THEN the Core Crate SHALL derive `Debug` and `Clone` for all public model types

### Requirement 3

**User Story:** As a library consumer, I want the client to handle authentication transparently, so that I can make API calls without managing cookies and crumbs manually.

#### Acceptance Criteria

1. WHEN a `YahooFinanceClient` is created THEN the Core Crate SHALL accept an `Arc<YahooAuthManager>` and `Arc<FetchClient>` for dependency injection
2. WHEN an API request receives a 401 response THEN the Yahoo Client SHALL automatically refresh authentication and retry the request once
3. WHEN authentication refresh fails THEN the Yahoo Client SHALL return an `AuthFailed` error variant

### Requirement 4

**User Story:** As a maintainer, I want the main application to depend on the core crate, so that code duplication is eliminated and the codebase is modular.

#### Acceptance Criteria

1. WHEN the Main Application is built THEN the Main Application SHALL depend on `finance-query-core` as a workspace member
2. WHEN the Main Application imports client types THEN the Main Application SHALL use `finance_query_core::client::*` instead of local modules
3. WHEN the Main Application imports model types THEN the Main Application SHALL use `finance_query_core::models::*` instead of local modules
4. WHEN the Main Application defines web-specific error handling THEN the Main Application SHALL implement `ResponseError` for `YahooError` in its own crate

### Requirement 5

**User Story:** As a crate publisher, I want proper crate metadata, so that the crate can be published to crates.io with discoverability.

#### Acceptance Criteria

1. WHEN the Core Crate `Cargo.toml` is configured THEN the Core Crate SHALL include name, version, edition, description, license, repository, keywords, and categories fields
2. WHEN the Core Crate edition is specified THEN the Core Crate SHALL use edition "2021" (not "2024" which is invalid)
3. WHEN the Core Crate is prepared for publishing THEN the Core Crate SHALL include a README.md with usage examples
4. WHEN the Core Crate is prepared for publishing THEN the Core Crate SHALL include a LICENSE file

### Requirement 6

**User Story:** As a developer, I want the workspace configured as a Cargo workspace, so that both crates can be developed and tested together.

#### Acceptance Criteria

1. WHEN the root `Cargo.toml` is configured THEN the workspace SHALL define `[workspace]` with members including "." and "finance-query-core"
2. WHEN dependencies are shared between crates THEN the workspace SHALL use `[workspace.dependencies]` for version consistency
3. WHEN the workspace is built THEN both the Main Application and Core Crate SHALL compile successfully
