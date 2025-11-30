//! Yahoo Finance API client.
//!
//! This module provides the main client for interacting with Yahoo Finance APIs.

use crate::client::error::YahooError;
use crate::client::fetch_client::FetchClient;
use crate::client::yahoo_auth::YahooAuthManager;
use reqwest::cookie::CookieStore;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Main client for Yahoo Finance API requests.
///
/// This client handles authenticated requests to Yahoo Finance,
/// automatically managing crumb tokens and retrying on auth failures.
pub struct YahooFinanceClient {
    auth_manager: Arc<YahooAuthManager>,
    fetch_client: Arc<FetchClient>,
}

impl YahooFinanceClient {
    /// Create a new YahooFinanceClient.
    ///
    /// # Arguments
    /// * `auth_manager` - Shared authentication manager
    /// * `fetch_client` - Shared HTTP client
    pub fn new(auth_manager: Arc<YahooAuthManager>, fetch_client: Arc<FetchClient>) -> Self {
        Self {
            auth_manager,
            fetch_client,
        }
    }

    async fn yahoo_request(
        &self,
        url: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<reqwest::Response, YahooError> {
        // First attempt with cached crumb
        match self.yahoo_request_inner(url, params).await {
            Ok(response) => Ok(response),
            Err(YahooError::AuthFailed(msg)) => {
                // Got 401, force refresh and retry once
                warn!(
                    "Got 401 Unauthorized: {}. Forcing auth refresh and retrying once",
                    msg
                );
                self.auth_manager.refresh().await?;
                info!("Auth refreshed, retrying request");
                self.yahoo_request_inner(url, params).await
            }
            Err(e) => Err(e),
        }
    }


    async fn yahoo_request_inner(
        &self,
        url: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<reqwest::Response, YahooError> {
        debug!("Getting crumb for Yahoo request");
        let (cookie_jar, crumb) = self.auth_manager.get_or_refresh().await?;
        debug!("Got crumb (length: {}): {}", crumb.len(), &crumb);

        // Log cookies for debugging
        if let Ok(url_parsed) = url::Url::parse(url) {
            if let Some(cookie_header) = cookie_jar.cookies(&url_parsed) {
                if let Ok(cookie_str) = cookie_header.to_str() {
                    let cookie_count = cookie_str.split(';').count();
                    debug!(
                        "Using {} cookies for request to {}",
                        cookie_count,
                        url_parsed.host_str().unwrap_or("unknown")
                    );
                    debug!("Cookie header length: {} bytes", cookie_str.len());

                    // Log individual cookie names (not values for security)
                    for cookie in cookie_str.split(';') {
                        if let Some(name) = cookie.trim().split('=').next() {
                            debug!("  Cookie: {}", name);
                        }
                    }
                } else {
                    warn!("Could not read cookie header");
                }
            } else {
                warn!(
                    "No cookies found in jar for {}",
                    url_parsed.host_str().unwrap_or("unknown")
                );
            }
        }

        // Create a new client with the cookie jar from auth manager
        // IMPORTANT: Also use the proxy for API requests
        let mut builder = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .cookie_provider(cookie_jar.clone())
            .redirect(reqwest::redirect::Policy::limited(10));

        // Add proxy configuration if available
        if let Some(proxy_url) = self.fetch_client.auth_proxy() {
            debug!(
                "Using proxy for Yahoo API request: {}...",
                &proxy_url.chars().take(30).collect::<String>()
            );
            builder = builder
                .proxy(reqwest::Proxy::all(proxy_url).map_err(YahooError::NetworkError)?)
                .danger_accept_invalid_certs(true);
        }

        let client = builder.build().map_err(YahooError::NetworkError)?;

        let mut request = client
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            )
            .header("Accept", "application/json")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Referer", "https://finance.yahoo.com/")
            .query(&[("crumb", crumb.as_str())]);

        if let Some(params) = params {
            request = request.query(params);
        }

        debug!("Sending Yahoo request to: {}", url);
        let response = request.send().await.map_err(YahooError::NetworkError)?;

        let status = response.status();
        debug!("Yahoo API response status: {}", status);

        if status == 401 {
            error!("Yahoo API returned 401 Unauthorized. Crumb may be invalid or expired.");
            // Log response body for debugging
            if let Ok(body) = response.text().await {
                debug!(
                    "401 response body (first 200 chars): {}",
                    body.chars().take(200).collect::<String>()
                );
            }
            return Err(YahooError::AuthFailed("Yahoo auth failed".to_string()));
        }
        if status == 404 {
            return Err(YahooError::NotFound("Yahoo symbol not found".to_string()));
        }
        if status == 429 {
            return Err(YahooError::RateLimited);
        }
        if !status.is_success() {
            return Err(YahooError::HttpError(
                status.as_u16(),
                format!(
                    "HTTP {}: {}",
                    status,
                    response.status().canonical_reason().unwrap_or("Unknown")
                ),
            ));
        }

        Ok(response)
    }


    async fn json(
        &self,
        url: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value, YahooError> {
        debug!("Making JSON request to: {}", url);
        if let Some(params) = params {
            debug!("Request params: {:?}", params);
        }
        let response = self.yahoo_request(url, params).await?;
        let status = response.status();
        info!("Received response with status: {}", status);
        let text = response.text().await.map_err(YahooError::NetworkError)?;
        debug!("Response text length: {} bytes", text.len());
        debug!(
            "Response preview (first 500 chars): {}",
            &text.chars().take(500).collect::<String>()
        );
        serde_json::from_str(&text).map_err(|e| {
            error!(
                "Failed to parse JSON response from {}: {}. Response text: {}",
                url,
                e,
                &text.chars().take(200).collect::<String>()
            );
            YahooError::ParseError(format!("Failed to parse JSON response from {}: {}", url, e))
        })
    }

    /// Get detailed quote data for a symbol.
    pub async fn get_quote(&self, symbol: &str) -> Result<Value, YahooError> {
        let url = format!(
            "https://query2.finance.yahoo.com/v10/finance/quoteSummary/{}",
            symbol
        );
        let params = [(
            "modules",
            "assetProfile,price,summaryDetail,defaultKeyStatistics,calendarEvents,quoteUnadjustedPerformanceOverview",
        )];
        self.json(&url, Some(&params)).await
    }

    /// Get simple quote data for multiple symbols.
    pub async fn get_simple_quotes(&self, symbols: &[&str]) -> Result<Value, YahooError> {
        info!("Fetching simple quotes for symbols: {:?}", symbols);
        let url = "https://query1.finance.yahoo.com/v7/finance/quote";
        let symbols_str = symbols.join(",");
        let params = [("symbols", symbols_str.as_str())];
        let result = self.json(url, Some(&params)).await;
        match &result {
            Ok(data) => {
                info!("Successfully received quote data");
                debug!(
                    "Quote data keys: {:?}",
                    data.as_object().map(|o| o.keys().collect::<Vec<_>>())
                );
            }
            Err(e) => {
                error!("Failed to fetch simple quotes: {}", e);
            }
        }
        result
    }

    /// Get chart data for a symbol.
    pub async fn get_chart(
        &self,
        symbol: &str,
        interval: &str,
        range: &str,
    ) -> Result<Value, YahooError> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );
        let params = [("interval", interval), ("range", range)];
        self.json(&url, Some(&params)).await
    }

    /// Get chart data using period1 and period2 (Unix timestamps) instead of range.
    pub async fn get_chart_with_periods(
        &self,
        symbol: &str,
        interval: &str,
        period1: i64,
        period2: i64,
    ) -> Result<Value, YahooError> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );
        let params = [
            ("interval", interval),
            ("period1", &period1.to_string()),
            ("period2", &period2.to_string()),
        ];
        self.json(&url, Some(&params)).await
    }

    /// Search for symbols.
    pub async fn search(&self, query: &str, hits: usize) -> Result<Value, YahooError> {
        let url = "https://query1.finance.yahoo.com/v1/finance/search";
        let params = [("q", query), ("quotesCount", &hits.to_string())];
        self.json(url, Some(&params)).await
    }

    /// Get similar/recommended quotes for a symbol.
    pub async fn get_similar_quotes(&self, symbol: &str, limit: usize) -> Result<Value, YahooError> {
        let url = format!(
            "https://query2.finance.yahoo.com/v6/finance/recommendationsbysymbol/{}",
            symbol
        );
        let count_str = limit.to_string();
        let params = [("count", count_str.as_str())];
        self.json(&url, Some(&params)).await
    }


    /// Get fundamentals timeseries data.
    pub async fn get_fundamentals_timeseries(
        &self,
        symbol: &str,
        period1: i64,
        period2: i64,
        types: &[&str],
    ) -> Result<Value, YahooError> {
        let url = format!(
            "https://query1.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries/{}",
            symbol
        );
        let types_str = types.join(",");
        let period1_str = period1.to_string();
        let period2_str = period2.to_string();
        let params = [
            ("merge", "false"),
            ("padTimeSeries", "true"),
            ("period1", period1_str.as_str()),
            ("period2", period2_str.as_str()),
            ("type", types_str.as_str()),
            ("lang", "en-US"),
            ("region", "US"),
        ];
        self.json(&url, Some(&params)).await
    }

    /// Get quote summary with specified modules.
    pub async fn get_quote_summary(
        &self,
        symbol: &str,
        modules: &[&str],
    ) -> Result<Value, YahooError> {
        let url = format!(
            "https://query2.finance.yahoo.com/v10/finance/quoteSummary/{}",
            symbol
        );
        let modules_str = modules.join(",");
        let params = [
            ("modules", modules_str.as_str()),
            ("corsDomain", "finance.yahoo.com"),
            ("formatted", "false"),
        ];
        self.json(&url, Some(&params)).await
    }

    /// Get quote type information.
    pub async fn get_quote_type(&self, symbol: &str) -> Result<Value, YahooError> {
        let url = format!(
            "https://query1.finance.yahoo.com/v1/finance/quoteType/{}",
            symbol
        );
        self.json(&url, None).await
    }

    /// Get earnings transcript.
    pub async fn get_earnings_transcript(
        &self,
        event_id: &str,
        company_id: &str,
    ) -> Result<Value, YahooError> {
        let url = "https://finance.yahoo.com/xhr/transcript";
        let params = [
            ("eventType", "earnings_call"),
            ("quartrId", company_id),
            ("eventId", event_id),
            ("lang", "en-US"),
            ("region", "US"),
        ];
        self.json(url, Some(&params)).await
    }

    /// Make a Yahoo Finance API request and return the raw response.
    /// This is useful for endpoints that need custom handling.
    pub async fn make_request(
        &self,
        url: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<reqwest::Response, YahooError> {
        self.yahoo_request(url, params).await
    }

    /// Get stock actions (dividends, splits, capital gains) for a symbol
    ///
    /// # Arguments
    /// * `symbol` - The stock symbol
    /// * `period` - Time period (e.g., "max", "5y", "1y")
    ///
    /// # Example
    /// ```rust,ignore
    /// let actions = client.get_actions("AAPL", "max").await?;
    /// println!("Total dividends: {}", actions.total_dividends());
    /// ```
    pub async fn get_actions(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<crate::models::ActionsResponse, YahooError> {
        use crate::models::actions::{ActionsResponse, YahooEventsResponse};

        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );

        let params = [
            ("interval", "1d"),
            ("range", period),
            ("events", "div,split,capitalGains"),
        ];

        let response = self.yahoo_request(&url, Some(&params)).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooEventsResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse actions response: {}", e))
        })?;

        ActionsResponse::from_yahoo_response(symbol.to_string(), yahoo_response)
    }

    /// Get only dividends for a symbol
    pub async fn get_dividends(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<crate::models::Dividend>, YahooError> {
        let actions = self.get_actions(symbol, period).await?;
        Ok(actions.dividends)
    }

    /// Get only stock splits for a symbol
    pub async fn get_splits(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<crate::models::StockSplit>, YahooError> {
        let actions = self.get_actions(symbol, period).await?;
        Ok(actions.splits)
    }

    /// Get only capital gains for a symbol
    pub async fn get_capital_gains(
        &self,
        symbol: &str,
        period: &str,
    ) -> Result<Vec<crate::models::CapitalGain>, YahooError> {
        let actions = self.get_actions(symbol, period).await?;
        Ok(actions.capital_gains)
    }

    /// Get option chain for a symbol and expiration date
    ///
    /// # Arguments
    /// * `symbol` - The stock symbol
    /// * `date` - Optional expiration date (YYYY-MM-DD). If None, returns nearest expiration
    ///
    /// # Example
    /// ```rust,ignore
    /// let chain = client.get_option_chain("AAPL", Some("2025-12-19")).await?;
    /// println!("Calls: {}, Puts: {}", chain.calls.len(), chain.puts.len());
    /// ```
    pub async fn get_option_chain(
        &self,
        symbol: &str,
        date: Option<&str>,
    ) -> Result<crate::models::OptionChain, YahooError> {
        use crate::models::options::{date_to_timestamp, OptionChain, YahooOptionsResponse};

        let url = format!(
            "https://query2.finance.yahoo.com/v7/finance/options/{}",
            symbol
        );

        let response = if let Some(exp_date) = date {
            // Convert date to Unix timestamp
            let timestamp = date_to_timestamp(exp_date)?;
            let timestamp_str = timestamp.to_string();
            let params = [("date", timestamp_str.as_str())];
            self.yahoo_request(&url, Some(&params)).await?
        } else {
            self.yahoo_request(&url, None).await?
        };

        let text = response.text().await.map_err(YahooError::NetworkError)?;
        let yahoo_response: YahooOptionsResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse options response: {}", e))
        })?;

        let expiration_date = date.unwrap_or("nearest").to_string();
        OptionChain::from_yahoo_response(symbol.to_string(), expiration_date, yahoo_response)
    }

    /// Get all available option expiration dates for a symbol
    ///
    /// # Example
    /// ```rust,ignore
    /// let expirations = client.get_option_expirations("AAPL").await?;
    /// for exp in &expirations.expirations {
    ///     println!("Expiration: {}", exp);
    /// }
    /// ```
    pub async fn get_option_expirations(
        &self,
        symbol: &str,
    ) -> Result<crate::models::OptionExpirations, YahooError> {
        use crate::models::options::{OptionExpirations, YahooOptionsResponse};

        let url = format!(
            "https://query2.finance.yahoo.com/v7/finance/options/{}",
            symbol
        );

        let response = self.yahoo_request(&url, None).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooOptionsResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse options response: {}", e))
        })?;

        OptionExpirations::from_yahoo_response(symbol.to_string(), yahoo_response)
    }

    /// Get calendar events (earnings dates, dividend dates) for a symbol
    ///
    /// # Example
    /// ```rust,ignore
    /// let calendar = client.get_calendar("AAPL").await?;
    /// if let Some(date) = calendar.earnings_date {
    ///     println!("Next earnings: {}", date);
    /// }
    /// ```
    pub async fn get_calendar(
        &self,
        symbol: &str,
    ) -> Result<crate::models::Calendar, YahooError> {
        use crate::models::calendar::{Calendar, YahooCalendarResponse};

        let url = format!(
            "https://query2.finance.yahoo.com/v10/finance/quoteSummary/{}",
            symbol
        );

        let params = [
            ("modules", "calendarEvents"),
            ("corsDomain", "finance.yahoo.com"),
            ("formatted", "false"),
        ];

        let response = self.yahoo_request(&url, Some(&params)).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooCalendarResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse calendar response: {}", e))
        })?;

        Calendar::from_yahoo_response(symbol.to_string(), yahoo_response)
    }

    /// Get SEC filings for a symbol
    ///
    /// # Example
    /// ```rust,ignore
    /// let filings = client.get_sec_filings("AAPL").await?;
    /// for filing in &filings.filings {
    ///     println!("{}: {} - {}", filing.date, filing.filing_type, filing.title);
    /// }
    /// ```
    pub async fn get_sec_filings(
        &self,
        symbol: &str,
    ) -> Result<crate::models::SecFilingsResponse, YahooError> {
        use crate::models::sec_filings::{SecFilingsResponse, YahooSecFilingsResponse};

        let url = format!(
            "https://query2.finance.yahoo.com/v10/finance/quoteSummary/{}",
            symbol
        );

        let params = [
            ("modules", "secFilings"),
            ("corsDomain", "finance.yahoo.com"),
            ("formatted", "false"),
        ];

        let response = self.yahoo_request(&url, Some(&params)).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooSecFilingsResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse SEC filings response: {}", e))
        })?;

        SecFilingsResponse::from_yahoo_response(symbol.to_string(), yahoo_response)
    }

    /// Get ESG/Sustainability scores for a symbol
    ///
    /// # Example
    /// ```rust,ignore
    /// let esg = client.get_sustainability("AAPL").await?;
    /// if let Some(score) = esg.total_esg {
    ///     println!("ESG Score: {:.1} (Rating: {})", score, esg.rating().unwrap_or("N/A"));
    /// }
    /// ```
    pub async fn get_sustainability(
        &self,
        symbol: &str,
    ) -> Result<crate::models::SustainabilityScores, YahooError> {
        use crate::models::sustainability::{SustainabilityScores, YahooEsgResponse};

        let url = format!(
            "https://query2.finance.yahoo.com/v10/finance/quoteSummary/{}",
            symbol
        );

        let params = [
            ("modules", "esgScores"),
            ("corsDomain", "finance.yahoo.com"),
            ("formatted", "false"),
        ];

        let response = self.yahoo_request(&url, Some(&params)).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooEsgResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse ESG response: {}", e))
        })?;

        SustainabilityScores::from_yahoo_response(symbol.to_string(), yahoo_response)
    }

    /// Get industry data including top performing and growth companies
    ///
    /// # Arguments
    /// * `industry_key` - The industry key (e.g., "technology-hardware")
    ///
    /// # Example
    /// ```rust,ignore
    /// let industry = client.get_industry("technology-hardware").await?;
    /// println!("Industry: {} (Sector: {:?})", industry.name, industry.sector_name);
    /// ```
    pub async fn get_industry(
        &self,
        industry_key: &str,
    ) -> Result<crate::models::Industry, YahooError> {
        use crate::models::industry::{Industry, YahooIndustryResponse};

        let url = format!(
            "https://query2.finance.yahoo.com/v1/finance/industries/{}",
            industry_key
        );

        let response = self.yahoo_request(&url, None).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooIndustryResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse industry response: {}", e))
        })?;

        Industry::from_yahoo_response(yahoo_response)
    }

    /// Get market status (open/closed/pre/post)
    ///
    /// # Arguments
    /// * `market` - Market identifier (e.g., "us_market", "gb_market")
    ///
    /// # Example
    /// ```rust,ignore
    /// let status = client.get_market_status("us_market").await?;
    /// println!("Market is {}", if status.is_open() { "open" } else { "closed" });
    /// ```
    pub async fn get_market_status(
        &self,
        market: &str,
    ) -> Result<crate::models::MarketStatus, YahooError> {
        use crate::models::market::{MarketStatus, YahooMarketTimeResponse};

        let url = "https://query1.finance.yahoo.com/v6/finance/markettime";

        let params = [
            ("formatted", "true"),
            ("key", "finance"),
            ("lang", "en-US"),
            ("market", market),
        ];

        let response = self.yahoo_request(&url, Some(&params)).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooMarketTimeResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse market time response: {}", e))
        })?;

        MarketStatus::from_yahoo_response(market.to_string(), yahoo_response)
    }

    /// Get market summary with major indices
    ///
    /// # Arguments
    /// * `market` - Market identifier (e.g., "us_market")
    ///
    /// # Example
    /// ```rust,ignore
    /// let summary = client.get_market_summary("us_market").await?;
    /// for index in &summary.indices {
    ///     println!("{}: {} ({:+.2}%)", index.short_name, index.price, index.percent_change);
    /// }
    /// ```
    pub async fn get_market_summary(
        &self,
        market: &str,
    ) -> Result<crate::models::MarketSummaryResponse, YahooError> {
        use crate::models::market::{MarketSummaryResponse, YahooMarketSummaryResponse};

        let url = "https://query1.finance.yahoo.com/v6/finance/quote/marketSummary";

        let params = [
            ("fields", "shortName,regularMarketPrice,regularMarketChange,regularMarketChangePercent"),
            ("formatted", "false"),
            ("lang", "en-US"),
            ("market", market),
        ];

        let response = self.yahoo_request(&url, Some(&params)).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;

        let yahoo_response: YahooMarketSummaryResponse = serde_json::from_str(&text).map_err(|e| {
            YahooError::ParseError(format!("Failed to parse market summary response: {}", e))
        })?;

        // Optionally get market status
        let status = self.get_market_status(market).await.ok();

        MarketSummaryResponse::from_yahoo_response(market.to_string(), yahoo_response, status)
    }
}
