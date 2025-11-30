use finance_query_core::client::YahooFinanceClient;
use finance_query_core::client::error::YahooError;
use finance_query_core::models::{SearchResult, SearchResponse};
use serde_json::Value;

pub async fn search(
    yahoo_client: &YahooFinanceClient,
    query: &str,
    hits: usize,
) -> Result<SearchResponse, YahooError> {
    let data = yahoo_client.search(query, hits).await?;
    parse_search_results(data)
}

fn parse_search_results(data: Value) -> Result<SearchResponse, YahooError> {
    let mut results = Vec::new();

    if let Some(quotes) = data.get("quotes").and_then(|q| q.as_array()) {
        for quote in quotes {
            results.push(SearchResult {
                symbol: quote.get("symbol")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string(),
                name: quote.get("longname")
                    .or_else(|| quote.get("shortname"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string(),
                exchange: quote.get("exchange")
                    .and_then(|e| e.as_str())
                    .map(|s| s.to_string()),
                quote_type: quote.get("quoteType")
                    .and_then(|q| q.as_str())
                    .map(|s| s.to_string()),
            });
        }
    }

    Ok(SearchResponse { results })
}