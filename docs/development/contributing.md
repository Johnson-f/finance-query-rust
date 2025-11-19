# Contributor Guide

Thank you for your interest in improving FinanceQuery Rust.
This project is open-source under the [MIT license] and
welcomes contributions in the form of bug reports, feature requests, and pull requests.

Here is a list of important resources for contributors:

- [Source Code]
- [Documentation]
- [Issue Tracker]
- [Code of Conduct]

[MIT license]: https://opensource.org/licenses/MIT
[Documentation]: https://verdenroz.github.io/finance-query-rust/
[Source Code]: https://github.com/Verdenroz/finance-query-rust
[Issue Tracker]: https://github.com/Verdenroz/finance-query-rust/issues
[Code of Conduct]: https://github.com/Verdenroz/finance-query-rust/CODE_OF_CONDUCT.md

## How to report a bug

Report bugs on the [Issue Tracker].

When filing an issue, make sure to answer these questions:

- Which operating system and Rust version are you using?
- Which version of this project are you using?
- What did you do?
- What did you expect to see?
- What did you see instead?

The best way to get your bug fixed is to provide a test case,
and/or steps to reproduce the issue.

## How to request a feature

Request features on the [Issue Tracker].

## How to set up your development environment

You need Rust 1.70 or newer (Rust 2021 edition). We recommend using [rustup](https://rustup.rs/) to manage your Rust installation.

To check your Rust version:
```bash
rustc --version
```

If you need to update Rust:
```bash
rustup update
```

### Quick Setup

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/Verdenroz/finance-query-rust.git
   cd finance-query-rust
   ```

3. **Install dependencies**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

5. **Run the development server**:
   ```bash
   cargo run
   ```

The server will start at `http://localhost:8080` by default.

### Setting up environment variables

Create a `.env` file in the project root with the following variables:

```env
# Redis configuration (optional)
REDIS_URL=redis://localhost:6379  # Used for caching and rate limiting
# If not set, caching and rate limiting will be disabled (graceful degradation)

# Rate limiting configuration
RATE_LIMIT_PER_DAY=10000  # Daily request limit per IP (default: 10,000)
# If not set, defaults to 10,000 requests per day per IP

# Proxy configuration (optional)
PROXY_URL=http://proxy.example.com:8080  # Proxy server URL for HTTP requests
# Used by FetchClient and YahooAuthManager for web scraping

# Logging configuration
RUST_LOG=info  # Log level: trace, debug, info, warn, error (default: info)
# Controls tracing output level via tracing-subscriber

# Logo fetching configuration (optional)
DISABLE_LOGO_FETCHING=false  # Set to "true" to disable logo fetching entirely
LOGO_TIMEOUT_SECONDS=1  # Timeout in seconds for logo requests (default: 1)
```

**Note**: All environment variables are optional. The application will run with sensible defaults if they are not provided.

## How to test the project

Run the full test suite:

```bash
cargo test
```

You can also run specific test files:

```bash
cargo test --test test_quotes
```

Or run tests with output:

```bash
cargo test -- --nocapture
```

Unit tests are located in the `tests/` directory (if present),
and are written using Rust's built-in testing framework.

## Local development

### Running the Development Server

To run the application locally:

```bash
cargo run
```

This will start the API server at `http://localhost:8080`.

**Note**: Actix-Web doesn't have built-in auto-reload. For development with auto-reload, you can use `cargo watch`:

```bash
# Install cargo-watch if not already installed
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

Alternatively, you can use `cargo watch` to run other commands:
```bash
# Watch and run tests
cargo watch -x test

# Watch and check compilation
cargo watch -x check
```

### Docker Development

You can also use Docker:

```bash
# Build the Docker image
docker build -t finance-query-rust .

# Run the container
docker run -p 8080:8080 finance-query-rust
```

#### Docker with Environment Variables

**Runtime configuration**:
```bash
docker run -p 8080:8080 \
  -e RUST_LOG=debug \
  -e REDIS_URL=redis://host.docker.internal:6379 \
  -e RATE_LIMIT_PER_DAY=10000 \
  -e PROXY_URL=http://proxy:8080 \
  -e DISABLE_LOGO_FETCHING=false \
  -e LOGO_TIMEOUT_SECONDS=1 \
  finance-query-rust
```

**Docker Compose example**:
```yaml
version: '3.8'
services:
  api:
    build: .
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
      - REDIS_URL=redis://redis:6379
      - RATE_LIMIT_PER_DAY=10000
      - DISABLE_LOGO_FETCHING=false
      - LOGO_TIMEOUT_SECONDS=1
    depends_on:
      - redis
  redis:
    image: redis:alpine
    ports:
      - "6379:6379"
```

## How to submit changes

Open a [pull request] to submit changes to this project.

Your pull request needs to meet the following guidelines for acceptance:

- The test suite must pass without errors and warnings.
- Include unit tests for new functionality.
- If your changes add functionality, update the documentation accordingly.
- Follow the existing code style (use `cargo fmt` for formatting).
- Ensure all lints pass (`cargo clippy`).

Feel free to submit early, though—we can always iterate on this.

### Code Quality Checks

#### Formatting

Format your code using Rust's built-in formatter:

```bash
cargo fmt
```

#### Linting

Run Clippy to catch common mistakes and improve code quality:

```bash
# Run Clippy with all lints
cargo clippy -- -D warnings

# Run Clippy with suggestions
cargo clippy -- -W clippy::all
```

#### Building

Ensure your code compiles without warnings:

```bash
cargo build
```

For release builds:

```bash
cargo build --release
```

#### Running All Checks

You can run all checks at once:

```bash
# Format, lint, test, and build
cargo fmt && cargo clippy -- -D warnings && cargo test && cargo build
```

It is recommended to open an issue before starting work on anything.
This will allow a chance to talk it over with the owners and validate your approach.

### Branch Workflow

FinanceQuery Rust follows a structured branch workflow:

1. **Feature branches**: Create a branch for your feature or bugfix. Branch names should be descriptive and follow this format: `feat/your-feature-name` or `fix/issue-description`.

2. **Staging branch**: All feature branches must be merged into the `staging` branch first for integration testing.

3. **Master branch**: The `master` branch contains production-ready code. Pull requests to `master` are only accepted from the `staging` branch and are automatically restricted by our CI workflow.

This workflow ensures that code in the master branch has been properly reviewed and tested in staging before deployment to production.

```
feature/your-feature --> staging --> master
```

Please do not attempt to merge feature branches directly to master as these pull requests will be automatically rejected.

## Project architecture

Please review the [architecture document](architecture.md) to understand the project's structure before contributing.

## Rust-Specific Guidelines

### Error Handling

- **Custom Errors**: Use `thiserror` for library errors (see `src/client/error.rs` for example)
- **Error Types**: Define error enums that implement `thiserror::Error`
- **Result Types**: Prefer `Result<T, E>` over panicking
- **Error Propagation**: Use `?` operator for error propagation
- **HTTP Errors**: Implement `actix_web::ResponseError` for automatic HTTP conversion
- **Error Messages**: Provide meaningful error messages

### Async Programming

- **Async Functions**: Use `async/await` for I/O-bound operations
- **Tokio Runtime**: All async operations use Tokio runtime (via `#[actix_web::main]`)
- **Shared State**: Use `Arc<T>` for shared immutable data across async tasks
- **Send + Sync**: Be mindful of `Send` and `Sync` bounds for async contexts
- **Actix Actors**: Use Actix actor system for WebSocket management and concurrent message handling

### Testing

- **Unit Tests**: Write unit tests in the same file as the code using `#[cfg(test)]`
- **Integration Tests**: Write integration tests in the `tests/` directory
- **Async Tests**: Use `#[tokio::test]` for async test functions
- **Mocking**: Mock external dependencies where appropriate
- **Test Organization**: Group related tests in modules

### Performance

- **Benchmarking**: Use `cargo bench` for benchmarking (if benchmarks are set up)
- **Profiling**: Profile with `cargo flamegraph` or similar tools
- **Zero-Cost Abstractions**: Prefer zero-cost abstractions
- **Memory Management**: Use `Arc` for multi-threaded shared ownership, `Rc` for single-threaded
- **Avoid Clones**: Minimize unnecessary clones; use references and `Arc` instead

### Code Style

- **Formatting**: Always run `cargo fmt` before committing
- **Linting**: Fix all Clippy warnings (`cargo clippy -- -D warnings`)
- **Naming**: Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- **Documentation**: Document public APIs with doc comments (`///`)
- **Imports**: Organize imports logically (std, external crates, internal modules)

[pull request]: https://github.com/Verdenroz/finance-query-rust/pulls

