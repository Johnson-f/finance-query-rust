use async_graphql::*;
use crate::models::indices::MarketIndex as MarketIndexModel;

#[derive(SimpleObject, Clone)]
pub struct MarketIndex {
    pub name: String,
    pub value: f64,
    pub change: String,
    #[graphql(name = "percentChange")]
    pub percent_change: String,
    #[graphql(name = "fiveDaysReturn")]
    pub five_days_return: Option<String>,
    #[graphql(name = "oneMonthReturn")]
    pub one_month_return: Option<String>,
    #[graphql(name = "threeMonthReturn")]
    pub three_month_return: Option<String>,
    #[graphql(name = "sixMonthReturn")]
    pub six_month_return: Option<String>,
    #[graphql(name = "ytdReturn")]
    pub ytd_return: Option<String>,
    #[graphql(name = "yearReturn")]
    pub year_return: Option<String>,
    #[graphql(name = "threeYearReturn")]
    pub three_year_return: Option<String>,
    #[graphql(name = "fiveYearReturn")]
    pub five_year_return: Option<String>,
    #[graphql(name = "tenYearReturn")]
    pub ten_year_return: Option<String>,
    #[graphql(name = "maxReturn")]
    pub max_return: Option<String>,
}

impl From<MarketIndexModel> for MarketIndex {
    fn from(index: MarketIndexModel) -> Self {
        MarketIndex {
            name: index.name,
            value: index.value,
            change: index.change,
            percent_change: index.percent_change,
            five_days_return: index.five_days_return,
            one_month_return: index.one_month_return,
            three_month_return: index.three_month_return,
            six_month_return: index.six_month_return,
            ytd_return: index.ytd_return,
            year_return: index.year_return,
            three_year_return: index.three_year_return,
            five_year_return: index.five_year_return,
            ten_year_return: index.ten_year_return,
            max_return: index.max_return,
        }
    }
}