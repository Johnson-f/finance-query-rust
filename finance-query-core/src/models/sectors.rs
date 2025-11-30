use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Sector {
    #[serde(rename = "Basic Materials")]
    BasicMaterials,
    #[serde(rename = "Communication Services")]
    Communication,
    #[serde(rename = "Consumer Cyclical")]
    ConsumerCyclical,
    #[serde(rename = "Consumer Defensive")]
    ConsumerDefensive,
    Energy,
    #[serde(rename = "Financial Services")]
    FinancialServices,
    Healthcare,
    Industrials,
    #[serde(rename = "Real Estate")]
    RealEstate,
    Technology,
    Utilities,
}

impl Sector {
    pub fn as_str(&self) -> &'static str {
        match self {
            Sector::BasicMaterials => "Basic Materials",
            Sector::Communication => "Communication Services",
            Sector::ConsumerCyclical => "Consumer Cyclical",
            Sector::ConsumerDefensive => "Consumer Defensive",
            Sector::Energy => "Energy",
            Sector::FinancialServices => "Financial Services",
            Sector::Healthcare => "Healthcare",
            Sector::Industrials => "Industrials",
            Sector::RealEstate => "Real Estate",
            Sector::Technology => "Technology",
            Sector::Utilities => "Utilities",
        }
    }

    #[allow(dead_code)]
    pub fn url_path(&self) -> &'static str {
        match self {
            Sector::BasicMaterials => "basic-materials",
            Sector::Communication => "communication-services",
            Sector::ConsumerCyclical => "consumer-cyclical",
            Sector::ConsumerDefensive => "consumer-defensive",
            Sector::Energy => "energy",
            Sector::FinancialServices => "financial-services",
            Sector::Healthcare => "healthcare",
            Sector::Industrials => "industrials",
            Sector::RealEstate => "real-estate",
            Sector::Technology => "technology",
            Sector::Utilities => "utilities",
        }
    }


    #[allow(dead_code)]
    pub fn all() -> Vec<Sector> {
        vec![
            Sector::Technology,
            Sector::Healthcare,
            Sector::FinancialServices,
            Sector::ConsumerCyclical,
            Sector::Industrials,
            Sector::ConsumerDefensive,
            Sector::Energy,
            Sector::RealEstate,
            Sector::Utilities,
            Sector::BasicMaterials,
            Sector::Communication,
        ]
    }
}

impl FromStr for Sector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Basic Materials" => Ok(Sector::BasicMaterials),
            "Communication Services" => Ok(Sector::Communication),
            "Consumer Cyclical" => Ok(Sector::ConsumerCyclical),
            "Consumer Defensive" => Ok(Sector::ConsumerDefensive),
            "Energy" => Ok(Sector::Energy),
            "Financial Services" => Ok(Sector::FinancialServices),
            "Healthcare" => Ok(Sector::Healthcare),
            "Industrials" => Ok(Sector::Industrials),
            "Real Estate" => Ok(Sector::RealEstate),
            "Technology" => Ok(Sector::Technology),
            "Utilities" => Ok(Sector::Utilities),
            _ => Err(format!("Invalid sector: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSector {
    pub sector: String,
    #[serde(rename = "dayReturn")]
    pub day_return: String,
    #[serde(rename = "ytdReturn")]
    pub ytd_return: String,
    #[serde(rename = "yearReturn")]
    pub year_return: String,
    #[serde(rename = "threeYearReturn")]
    pub three_year_return: String,
    #[serde(rename = "fiveYearReturn")]
    pub five_year_return: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSectorDetails {
    pub sector: String,
    #[serde(rename = "dayReturn")]
    pub day_return: String,
    #[serde(rename = "ytdReturn")]
    pub ytd_return: String,
    #[serde(rename = "yearReturn")]
    pub year_return: String,
    #[serde(rename = "threeYearReturn")]
    pub three_year_return: String,
    #[serde(rename = "fiveYearReturn")]
    pub five_year_return: String,
    #[serde(rename = "marketCap")]
    pub market_cap: String,
    #[serde(rename = "marketWeight")]
    pub market_weight: String,
    pub industries: i32,
    pub companies: i32,
    #[serde(rename = "topIndustries")]
    pub top_industries: Vec<String>,
    #[serde(rename = "topCompanies")]
    pub top_companies: Vec<String>,
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
        fn market_sector_roundtrip(
            sector in "[A-Za-z ]{1,30}",
            day_return in "-?[0-9]{1,3}\\.[0-9]{2}%",
            ytd_return in "-?[0-9]{1,3}\\.[0-9]{2}%",
            year_return in "-?[0-9]{1,3}\\.[0-9]{2}%",
            three_year_return in "-?[0-9]{1,3}\\.[0-9]{2}%",
            five_year_return in "-?[0-9]{1,3}\\.[0-9]{2}%",
        ) {
            let ms = MarketSector {
                sector: sector.clone(),
                day_return: day_return.clone(),
                ytd_return: ytd_return.clone(),
                year_return: year_return.clone(),
                three_year_return: three_year_return.clone(),
                five_year_return: five_year_return.clone(),
            };

            let json = serde_json::to_string(&ms).unwrap();
            let parsed: MarketSector = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(ms.sector, parsed.sector);
            prop_assert_eq!(ms.day_return, parsed.day_return);
            prop_assert_eq!(ms.ytd_return, parsed.ytd_return);
            prop_assert_eq!(ms.year_return, parsed.year_return);
        }

        #[test]
        fn sector_roundtrip(sector in prop_oneof![
            Just(Sector::BasicMaterials),
            Just(Sector::Communication),
            Just(Sector::ConsumerCyclical),
            Just(Sector::ConsumerDefensive),
            Just(Sector::Energy),
            Just(Sector::FinancialServices),
            Just(Sector::Healthcare),
            Just(Sector::Industrials),
            Just(Sector::RealEstate),
            Just(Sector::Technology),
            Just(Sector::Utilities),
        ]) {
            let json = serde_json::to_string(&sector).unwrap();
            let parsed: Sector = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(sector.as_str(), parsed.as_str());
        }
    }
}
