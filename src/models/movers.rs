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