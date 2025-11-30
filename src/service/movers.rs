use finance_query_core::client::{error::YahooError, YahooFinanceClient};
use finance_query_core::models::movers::{MarketMover, MoverCount};
use serde_json::Value;
use tracing::{debug, error, info};

/// Fetch movers from Yahoo Finance screener API
async fn fetch_movers(
    yahoo_client: &YahooFinanceClient,
    url: &str,
) -> Result<Vec<MarketMover>, YahooError> {
    info!("Fetching movers from: {}", url);
    
    // Make request with fields parameter
    let params = [(
        "fields",
        "symbol,longName,shortName,regularMarketPrice,regularMarketChange,regularMarketChangePercent",
    )];
    
    let response = yahoo_client
        .make_request(url, Some(&params))
        .await?;
    
    let status = response.status();
    if !status.is_success() {
        return Err(YahooError::HttpError(
            status.as_u16(),
            format!("HTTP {}: {}", status, status.canonical_reason().unwrap_or("Unknown")),
        ));
    }
    
    let text = response.text().await.map_err(YahooError::NetworkError)?;
    let data: Value = serde_json::from_str(&text).map_err(|e| {
        error!("Failed to parse JSON response: {}. Response text: {}", e, &text.chars().take(200).collect::<String>());
        YahooError::ParseError(format!("Failed to parse JSON response: {}", e))
    })?;
    
    debug!("Parsed movers response, extracting quotes");
    
    // Extract quotes from response
    let quotes = data
        .get("finance")
        .and_then(|f| f.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .and_then(|first| first.get("quotes"))
        .and_then(|q| q.as_array())
        .ok_or_else(|| {
            YahooError::ParseError("No quotes found in movers response".to_string())
        })?;
    
    let mut movers = Vec::new();
    
    for item in quotes {
        let symbol = item
            .get("symbol")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| YahooError::ParseError("Missing symbol in mover data".to_string()))?;
        
        let name = item
            .get("longName")
            .or_else(|| item.get("shortName"))
            .and_then(|n| n.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| symbol.clone());
        
        let price = get_fmt(item, "regularMarketPrice")
            .unwrap_or_else(|| "0.0".to_string());
        
        let change = get_fmt(item, "regularMarketChange")
            .unwrap_or_else(|| "0.0".to_string());
        
        let percent_change = get_fmt(item, "regularMarketChangePercent")
            .unwrap_or_else(|| "0.0%".to_string());
        
        movers.push(MarketMover {
            symbol,
            name,
            price,
            change,
            percent_change,
        });
    }
    
    info!("Successfully parsed {} movers", movers.len());
    Ok(movers)
}

/// Helper function to extract formatted value from Yahoo API response
fn get_fmt(data: &Value, key: &str) -> Option<String> {
    data.get(key)
        .and_then(|v| {
            if let Some(obj) = v.as_object() {
                obj.get("fmt")
                    .or_else(|| obj.get("raw"))
                    .and_then(|val| {
                        val.as_str().map(|s| s.to_string())
                            .or_else(|| val.as_f64().map(|f| f.to_string()))
                    })
            } else if let Some(num) = v.as_f64() {
                Some(num.to_string())
            } else {
                v.as_str().map(|str_val| str_val.to_string())
            }
        })
}

/// Get the most active stocks
pub async fn get_actives(
    yahoo_client: &YahooFinanceClient,
    count: MoverCount,
) -> Result<Vec<MarketMover>, YahooError> {
    info!("Fetching most active stocks (count: {})", count.as_str());
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/screener/predefined/saved?count={}&formatted=true&scrIds=MOST_ACTIVES",
        count.as_str()
    );
    fetch_movers(yahoo_client, &url).await
}

/// Get the top gaining stocks
pub async fn get_gainers(
    yahoo_client: &YahooFinanceClient,
    count: MoverCount,
) -> Result<Vec<MarketMover>, YahooError> {
    info!("Fetching top gaining stocks (count: {})", count.as_str());
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/screener/predefined/saved?count={}&formatted=true&scrIds=DAY_GAINERS",
        count.as_str()
    );
    fetch_movers(yahoo_client, &url).await
}

/// Get the top losing stocks
pub async fn get_losers(
    yahoo_client: &YahooFinanceClient,
    count: MoverCount,
) -> Result<Vec<MarketMover>, YahooError> {
    info!("Fetching top losing stocks (count: {})", count.as_str());
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/screener/predefined/saved?count={}&formatted=true&scrIds=DAY_LOSERS",
        count.as_str()
    );
    fetch_movers(yahoo_client, &url).await
}