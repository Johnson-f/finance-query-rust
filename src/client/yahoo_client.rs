use crate::client::error::YahooError;
use crate::client::yahoo_auth::YahooAuthManager;
use crate::client::FetchClient;
use reqwest::cookie::CookieStore;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

pub struct YahooFinanceClient {
    auth_manager: Arc<YahooAuthManager>,
    #[allow(dead_code)]
    fetch_client: Arc<FetchClient>,
}

impl YahooFinanceClient {
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
                warn!("Got 401 Unauthorized: {}. Forcing auth refresh and retrying once", msg);
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
                    debug!("Using {} cookies for request to {}", cookie_count, url_parsed.host_str().unwrap_or("unknown"));
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
                warn!("No cookies found in jar for {}", url_parsed.host_str().unwrap_or("unknown"));
            }
        }

        // Create a new client with the cookie jar from auth manager
        let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .cookie_provider(cookie_jar.clone())
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .map_err(YahooError::NetworkError)?;

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
                debug!("401 response body (first 200 chars): {}", 
                    body.chars().take(200).collect::<String>());
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
                format!("HTTP {}: {}", status, response.status().canonical_reason().unwrap_or("Unknown")),
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
        debug!("Response preview (first 500 chars): {}", &text.chars().take(500).collect::<String>());
        serde_json::from_str(&text).map_err(|e| {
            error!("Failed to parse JSON response from {}: {}. Response text: {}", url, e, &text.chars().take(200).collect::<String>());
            YahooError::ParseError(format!("Failed to parse JSON response from {}: {}", url, e))
        })
    }

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

    pub async fn get_simple_quotes(&self, symbols: &[&str]) -> Result<Value, YahooError> {
        info!("Fetching simple quotes for symbols: {:?}", symbols);
        let url = "https://query1.finance.yahoo.com/v7/finance/quote";
        let symbols_str = symbols.join(",");
        let params = [("symbols", symbols_str.as_str())];
        let result = self.json(url, Some(&params)).await;
        match &result {
            Ok(data) => {
                info!("Successfully received quote data");
                debug!("Quote data keys: {:?}", data.as_object().map(|o| o.keys().collect::<Vec<_>>()));
            }
            Err(e) => {
                error!("Failed to fetch simple quotes: {}", e);
            }
        }
        result
    }

    pub async fn get_chart(
        &self,
        symbol: &str,
        interval: &str,
        range: &str,
    ) -> Result<Value, YahooError> {
        let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{}", symbol);
        let params = [("interval", interval), ("range", range)];
        self.json(&url, Some(&params)).await
    }

    pub async fn search(&self, query: &str, hits: usize) -> Result<Value, YahooError> {
        let url = "https://query1.finance.yahoo.com/v1/finance/search";
        let params = [
            ("q", query),
            ("quotesCount", &hits.to_string()),
        ];
        self.json(url, Some(&params)).await
    }

    pub async fn get_similar_quotes(&self, symbol: &str, limit: usize) -> Result<Value, YahooError> {
        let url = format!(
            "https://query2.finance.yahoo.com/v6/finance/recommendationsbysymbol/{}",
            symbol
        );
        let count_str = limit.to_string();
        let params = [("count", count_str.as_str())];
        self.json(&url, Some(&params)).await
    }

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

    pub async fn get_quote_type(&self, symbol: &str) -> Result<Value, YahooError> {
        let url = format!("https://query1.finance.yahoo.com/v1/finance/quoteType/{}", symbol);
        self.json(&url, None).await
    }

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

    /// Make a Yahoo Finance API request and return the raw response
    /// This is useful for endpoints that need custom handling
    pub async fn make_request(
        &self,
        url: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<reqwest::Response, YahooError> {
        self.yahoo_request(url, params).await
    }
}