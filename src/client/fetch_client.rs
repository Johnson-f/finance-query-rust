use crate::client::error::YahooError;
use reqwest::{cookie::Jar, Client, ClientBuilder, Url};
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct FetchClient {
    client: Client,
    cookie_jar: Arc<Jar>,
    proxy: Option<String>,
}

impl FetchClient {
    pub fn new(proxy: Option<String>) -> Result<Self, YahooError> {
        let cookie_jar = Arc::new(Jar::default());

        let mut builder = ClientBuilder::new()
            .timeout(DEFAULT_TIMEOUT)
            .cookie_store(true)
            .cookie_provider(cookie_jar.clone())
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36");

        if let Some(proxy_url) = &proxy {
            builder = builder.proxy(reqwest::Proxy::all(proxy_url).map_err(|e| {
                YahooError::NetworkError(e)
            })?);
        }

        let client = builder.build().map_err(YahooError::NetworkError)?;

        Ok(Self {
            client,
            cookie_jar,
            proxy,
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn cookie_jar(&self) -> &Arc<Jar> {
        &self.cookie_jar
    }

    pub async fn fetch(&self, url: &str) -> Result<String, YahooError> {
        let response = self
            .client
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("sec-ch-ua", r#""Chromium";v="122", "Google Chrome";v="122""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", r#""Windows""#)
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        let status = response.status();
        if !status.is_success() {
            return Err(YahooError::HttpError(
                status.as_u16(),
                format!("HTTP {}: {}", status, response.status().canonical_reason().unwrap_or("Unknown")),
            ));
        }

        response.text().await.map_err(YahooError::NetworkError)
    }

    pub async fn fetch_response(&self, url: &str) -> Result<reqwest::Response, YahooError> {
        let response = self
            .client
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("sec-ch-ua", r#""Chromium";v="122", "Google Chrome";v="122""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", r#""Windows""#)
            .send()
            .await
            .map_err(YahooError::NetworkError)?;

        Ok(response)
    }
}

