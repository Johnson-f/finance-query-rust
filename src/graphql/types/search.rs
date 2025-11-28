use async_graphql::*;
use crate::models::search::{SearchResult as SearchResultModel, SearchResponse as SearchResponseModel};

#[derive(SimpleObject, Clone)]
pub struct SearchResult {
    pub symbol: String,
    pub name: String,
    pub exchange: Option<String>,
    pub quote_type: Option<String>,
}

impl From<SearchResultModel> for SearchResult {
    fn from(result: SearchResultModel) -> Self {
        SearchResult {
            symbol: result.symbol,
            name: result.name,
            exchange: result.exchange,
            quote_type: result.quote_type,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

impl From<SearchResponseModel> for SearchResponse {
    fn from(response: SearchResponseModel) -> Self {
        SearchResponse {
            results: response.results.into_iter().map(SearchResult::from).collect(),
        }
    }
}