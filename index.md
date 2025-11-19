---
layout: default
title: Finance Query Rust API
---

# Finance Query Rust API

A high-performance financial data API built with Rust and Actix-Web, providing real-time stock quotes, historical data, financial statements, news, and more from Yahoo Finance.

## Features

- 🚀 **High Performance**: Built with Rust and Actix-Web for maximum throughput
- 📊 **Comprehensive Data**: Stock quotes, historical prices, financials, earnings, news, and more
- 🔄 **Real-time WebSockets**: Live updates for quotes, indices, movers, and news
- 🎯 **Type-Safe**: Leverages Rust's type system for reliability
- 🔒 **Rate Limiting**: Built-in IP-based rate limiting with Redis
- 💾 **Caching**: Redis-based caching for improved performance
- 🌐 **CORS Enabled**: Ready for frontend integration

## Quick Start

### Prerequisites

- Rust 1.70+ (Rust 2021 edition)
- Redis (optional, for caching and rate limiting)

### Installation

```bash
# Clone the repository
git clone https://github.com/Verdenroz/finance-query-rust.git
cd finance-query-rust

# For development
# Run the script
./start.sh

# Checking for errors after making changes
cargo check
```

The API will be available at `http://localhost:8080` in development  
The API will be available at `https://api.tradstry.com` in production

## API Endpoints

### Health Checks
- `GET /health` - Comprehensive health check
- `GET /ping` - Basic health check

### Stock Data
- `GET /v1/quotes?symbols=AAPL,MSFT` - Get stock quotes
- `GET /v1/simple-quotes?symbols=AAPL` - Simplified quotes
- `GET /v1/historical/{symbol}` - Historical price data
- `GET /v1/search?q=apple` - Search for symbols

### Market Data
- `GET /v1/indices` - Market indices
- `GET /v1/actives` - Most active stocks
- `GET /v1/gainers` - Top gainers
- `GET /v1/losers` - Top losers
- `GET /v1/sectors` - Sector performance

### Financial Data
- `GET /v1/financials/{symbol}` - Financial statements
- `GET /v1/earnings/{symbol}/calls` - Earnings calls
- `GET /v1/earnings/{symbol}/transcript` - Earnings transcripts
- `GET /v1/holders/{symbol}/major` - Major holders
- `GET /v1/analysis/{symbol}/recommendations` - Analyst recommendations

### News
- `GET /v1/news` - Financial news

### WebSockets
- `WS /v1/ws/quotes` - Real-time quotes
- `WS /v1/ws/indices` - Real-time indices
- `WS /v1/ws/movers` - Real-time movers
- `WS /v1/ws/news` - Real-time news
- `WS /v1/ws/profile/{symbol}` - Real-time profile updates

## Documentation

- [Getting Started](/docs/getting-started)
- [API Documentation](/docs/api)
- [Architecture](/docs/development/architecture)
- [Contributing](/docs/development/contributing)
- [CURL Examples](/docs/development/CURL_EXAMPLES)

## Environment Variables

All environment variables are optional. The application will run with sensible defaults if not provided.

| Variable | Description | Default |
|----------|-------------|---------|
| `REDIS_URL` | External Redis connection string (e.g., Sevalla Redis) | None |
| `RATE_LIMIT_PER_DAY` | Daily request limit per IP | 10000 |
| `PROXY_URL` | Proxy server URL for HTTP requests | None |
| `RUST_LOG` | Logging level (trace/debug/info/warn/error) | info |

## Deployment

### Railway

1. Connect your GitHub repository to Railway
2. Railway will auto-detect the Rust project
3. Set environment variables in Railway dashboard:
   - `REDIS_URL` (if using Redis)
   - `RATE_LIMIT_PER_DAY` (optional)
   - `RUST_LOG` (optional)

The application will automatically build and deploy.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/Verdenroz/finance-query-rust/blob/main/LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [Contributing Guide](/docs/development/contributing) for guidelines.

## Support

- [Issue Tracker](https://github.com/Verdenroz/finance-query-rust/issues)
- [Documentation](https://johnson-f.github.io/finance-query-rust/)

