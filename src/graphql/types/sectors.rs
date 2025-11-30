use async_graphql::*;
use finance_query_core::models::sectors::{MarketSector as MarketSectorModel, MarketSectorDetails as MarketSectorDetailsModel};

#[derive(SimpleObject, Clone)]
pub struct MarketSector {
    pub sector: String,
    #[graphql(name = "dayReturn")]
    pub day_return: String,
    #[graphql(name = "ytdReturn")]
    pub ytd_return: String,
    #[graphql(name = "yearReturn")]
    pub year_return: String,
    #[graphql(name = "threeYearReturn")]
    pub three_year_return: String,
    #[graphql(name = "fiveYearReturn")]
    pub five_year_return: String,
}

impl From<MarketSectorModel> for MarketSector {
    fn from(sector: MarketSectorModel) -> Self {
        MarketSector {
            sector: sector.sector,
            day_return: sector.day_return,
            ytd_return: sector.ytd_return,
            year_return: sector.year_return,
            three_year_return: sector.three_year_return,
            five_year_return: sector.five_year_return,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct MarketSectorDetails {
    pub sector: String,
    #[graphql(name = "dayReturn")]
    pub day_return: String,
    #[graphql(name = "ytdReturn")]
    pub ytd_return: String,
    #[graphql(name = "yearReturn")]
    pub year_return: String,
    #[graphql(name = "threeYearReturn")]
    pub three_year_return: String,
    #[graphql(name = "fiveYearReturn")]
    pub five_year_return: String,
    #[graphql(name = "marketCap")]
    pub market_cap: String,
    #[graphql(name = "marketWeight")]
    pub market_weight: String,
    pub industries: i32,
    pub companies: i32,
    #[graphql(name = "topIndustries")]
    pub top_industries: Vec<String>,
    #[graphql(name = "topCompanies")]
    pub top_companies: Vec<String>,
}

impl From<MarketSectorDetailsModel> for MarketSectorDetails {
    fn from(details: MarketSectorDetailsModel) -> Self {
        MarketSectorDetails {
            sector: details.sector,
            day_return: details.day_return,
            ytd_return: details.ytd_return,
            year_return: details.year_return,
            three_year_return: details.three_year_return,
            five_year_return: details.five_year_return,
            market_cap: details.market_cap,
            market_weight: details.market_weight,
            industries: details.industries,
            companies: details.companies,
            top_industries: details.top_industries,
            top_companies: details.top_companies,
        }
    }
}