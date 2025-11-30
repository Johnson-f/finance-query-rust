use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchResult {
    pub symbol: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn optional_string() -> impl Strategy<Value = Option<String>> {
        proptest::option::of("[A-Za-z0-9 ]{1,20}")
    }

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn search_result_roundtrip(
            symbol in "[A-Z]{1,5}",
            name in "[A-Za-z ]{1,50}",
            exchange in optional_string(),
            quote_type in optional_string(),
        ) {
            let result = SearchResult {
                symbol: symbol.clone(),
                name: name.clone(),
                exchange: exchange.clone(),
                quote_type: quote_type.clone(),
            };

            let json = serde_json::to_string(&result).unwrap();
            let parsed: SearchResult = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(result.symbol, parsed.symbol);
            prop_assert_eq!(result.name, parsed.name);
            prop_assert_eq!(result.exchange, parsed.exchange);
            prop_assert_eq!(result.quote_type, parsed.quote_type);
        }
    }
}
