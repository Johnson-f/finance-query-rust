# Logo Fetcher

The logo module lets you fetch company logo URLs using the built-in `LogoFetcher`. It calls [logo.dev](https://logo.dev) behind the scenes and comes with in-memory caching, timeouts, and a circuit breaker so repeated failures do not flood the external service.

## Quick Start

```rust
use finance_query_core::{FetchClient, LogoFetcher};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Optional: set PROXY_URL to send traffic through a proxy
    let fetch_client = Arc::new(FetchClient::new(std::env::var("PROXY_URL").ok())?);

    // Construct the logo fetcher (thread-safe, cheap to clone)
    let logos = LogoFetcher::new(fetch_client.clone());

    if logos.is_enabled() {
        if let Some(url) = logos.fetch_logo("AAPL", None).await {
            println!("AAPL logo URL: {}", url);
        } else {
            println!("Logo not found or service unavailable");
        }
    } else {
        println!("Logo fetching disabled");
    }

    Ok(())
}
```

`fetch_logo(symbol, website)` returns `Option<String>` containing the resolved logo URL. The `website` argument is currently ignored and kept for backward compatibility.

## Configuration

- `DISABLE_LOGO_FETCHING`: set to `true` to short-circuit all logo requests (default: enabled).
- `LOGO_TIMEOUT_SECONDS`: per-request timeout in seconds, minimum 0.1s (default: 2).
- `LOGO_CIRCUIT_BREAKER_THRESHOLD`: consecutive failures before opening the breaker (default: 5).
- `LOGO_CIRCUIT_BREAKER_TIMEOUT`: cooldown period in seconds once the breaker is open (default: 300).
- Cache TTL: 24 hours per symbol (stored in-memory; reset when the process restarts).

## Behavior

- Uses `LogoFetcher::new(fetch_client: Arc<FetchClient>)`; share the same fetch client as the rest of your Yahoo stack so proxies and cookies stay consistent.
- Requests go to `https://img.logo.dev/ticker/{SYMBOL}?token=...&format=png&fallback=404&size=50&theme=dark`.
- Circuit breaker blocks new requests after repeated failures until the cooldown expires.
- A successful fetch caches the final URL; subsequent calls return immediately from cache until TTL expiry.
- Returns `None` on empty symbols, when disabled, when the breaker is open, on timeouts, or on non-success HTTP responses.

## Using With `YahooFinanceClient`

You can reuse the client's underlying fetcher so logo requests share proxy/auth state:

```rust
use finance_query_core::{
    FetchClient, LogoFetcher, YahooAuthManager, YahooFinanceClient
};
use std::sync::Arc;

let proxy = std::env::var("PROXY_URL").ok();
let fetch_client = Arc::new(FetchClient::new(proxy.clone())?);
let cookie_jar = fetch_client.cookie_jar().clone();
let auth = Arc::new(YahooAuthManager::new(proxy, cookie_jar));
let client = Arc::new(YahooFinanceClient::new(auth, fetch_client.clone()));

let logo_fetcher = LogoFetcher::new(client.fetch_client());
let url = logo_fetcher.fetch_logo("GOOGL", None).await;
println!("GOOGL logo: {:?}", url);
```

## Streaming Logos

`QuoteStream` and `SingleQuoteStream` automatically enrich `SimpleQuote.logo` using `LogoFetcher` when you construct the stream:

```rust
use finance_query_core::{QuoteStream, YahooFinanceClient};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::Duration;

// Assume `client: Arc<YahooFinanceClient>` is already created
let mut stream = QuoteStream::create(
    client.clone(),
    vec!["AAPL".into(), "MSFT".into()],
    Duration::from_secs(10),
);

if let Some(Ok(update)) = stream.next().await {
    for quote in update.quotes {
        println!("{} logo: {:?}", quote.symbol, quote.logo);
    }
}
```

Logos may be absent (`None`) if the service is disabled, rate-limited, or does not have an entry for that symbol.

