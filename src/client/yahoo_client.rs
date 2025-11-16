use crate::client::error::YahooError;
use crate::client::yahoo_auth::YahooAuthManager;
use crate::client::FetchClient;
use reqwest::cookie::Jar;
use serde_json::Value;
use std::sync::Arc;

pub struct YahooFinanceClient {
    auth_manager: Arc<YahooAuthManager>,
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
        let (cookie_jar, crumb) = self.auth_manager.get_or_refresh().await?;

        let mut request = self
            .fetch_client
            .client()
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36",
            )
            .query(&[("crumb", crumb.as_str())]);

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await.map_err(YahooError::NetworkError)?;

        let status = response.status();
        if status == 401 {
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
        let response = self.yahoo_request(url, params).await?;
        let text = response.text().await.map_err(YahooError::NetworkError)?;
        serde_json::from_str(&text).map_err(|e| {
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
        let url = "https://query1.finance.yahoo.com/v7/finance/quote";
        let symbols_str = symbols.join(",");
        let params = [
            ("symbols", symbols_str.as_str()),
            (
                "modules",
                "assetProfile,price,summaryDetail,defaultKeyStatistics,calendarEvents,quoteUnadjustedPerformanceOverview",
            ),
        ];
        self.json(url, Some(&params)).await
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
        let params = [("count", &limit.to_string())];
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
        let params = [
            ("merge", "false"),
            ("padTimeSeries", "true"),
            ("period1", &period1.to_string()),
            ("period2", &period2.to_string()),
            ("type", &types_str),
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
            ("modules", &modules_str),
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
}

