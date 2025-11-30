use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MoverCount {
    #[serde(rename = "25")]
    TwentyFive,
    #[serde(rename = "50")]
    #[default]
    Fifty,
    #[serde(rename = "100")]
    Hundred,
}

impl MoverCount {
    pub fn as_str(&self) -> &'static str {
        match self {
            MoverCount::TwentyFive => "25",
            MoverCount::Fifty => "50",
            MoverCount::Hundred => "100",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "25" => Some(MoverCount::TwentyFive),
            "50" => Some(MoverCount::Fifty),
            "100" => Some(MoverCount::Hundred),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarketMover {
    pub symbol: String,
    pub name: String,
    pub price: String,
    pub change: String,
    #[serde(rename = "percentChange")]
    pub percent_change: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn market_mover_roundtrip(
            symbol in "[A-Z]{1,5}",
            name in "[A-Za-z ]{1,50}",
            price in "[0-9]{1,4}\\.[0-9]{2}",
            change in "-?[0-9]{1,3}\\.[0-9]{2}",
            percent_change in "-?[0-9]{1,3}\\.[0-9]{2}%",
        ) {
            let mover = MarketMover {
                symbol: symbol.clone(),
                name: name.clone(),
                price: price.clone(),
                change: change.clone(),
                percent_change: percent_change.clone(),
            };

            let json = serde_json::to_string(&mover).unwrap();
            let parsed: MarketMover = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(mover.symbol, parsed.symbol);
            prop_assert_eq!(mover.name, parsed.name);
            prop_assert_eq!(mover.price, parsed.price);
            prop_assert_eq!(mover.change, parsed.change);
            prop_assert_eq!(mover.percent_change, parsed.percent_change);
        }

        #[test]
        fn mover_count_roundtrip(count in prop_oneof![
            Just(MoverCount::TwentyFive),
            Just(MoverCount::Fifty),
            Just(MoverCount::Hundred),
        ]) {
            let json = serde_json::to_string(&count).unwrap();
            let parsed: MoverCount = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(count.as_str(), parsed.as_str());
        }
    }
}
