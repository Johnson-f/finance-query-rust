use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Quote {
    pub symbol: String,
    pub name: String,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_market_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_hours_price: Option<String>,
    pub change: String,
    pub percent_change: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dividend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dividend_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex_dividend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_assets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_capital_gain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morningstar_rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morningstar_risk_rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdings_turnover: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earnings_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_dividend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inception_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employees: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub five_days_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub six_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub five_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ten_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SimpleQuote {
    pub symbol: String,
    pub name: String,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_market_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_hours_price: Option<String>,
    pub change: String,
    pub percent_change: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedQuote {
    pub symbol: String,
    pub name: String,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "preMarketPrice")]
    pub pre_market_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "afterHoursPrice")]
    pub after_hours_price: Option<String>,
    pub change: String,
    #[serde(rename = "percentChange")]
    pub percent_change: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yearHigh")]
    pub year_high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yearLow")]
    pub year_low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "avgVolume")]
    pub avg_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "marketCap")]
    pub market_cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dividend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yield")]
    pub dividend_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "exDividend")]
    pub ex_dividend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "netAssets")]
    pub net_assets: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "expenseRatio")]
    pub expense_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lastCapitalGain")]
    pub last_capital_gain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "morningstarRating")]
    pub morningstar_rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "morningstarRiskRating")]
    pub morningstar_risk_rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "holdingsTurnover")]
    pub holdings_turnover: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "earningsDate")]
    pub earnings_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lastDividend")]
    pub last_dividend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "inceptionDate")]
    pub inception_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employees: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fiveDaysReturn")]
    pub five_days_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "oneMonthReturn")]
    pub one_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "threeMonthReturn")]
    pub three_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sixMonthReturn")]
    pub six_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ytdReturn")]
    pub ytd_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yearReturn")]
    pub year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "threeYearReturn")]
    pub three_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fiveYearReturn")]
    pub five_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tenYearReturn")]
    pub ten_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxReturn")]
    pub max_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
}

impl From<Quote> for DetailedQuote {
    fn from(quote: Quote) -> Self {
        DetailedQuote {
            symbol: quote.symbol,
            name: quote.name,
            price: quote.price,
            pre_market_price: quote.pre_market_price,
            after_hours_price: quote.after_hours_price,
            change: quote.change,
            percent_change: quote.percent_change,
            open: quote.open,
            high: quote.high,
            low: quote.low,
            year_high: quote.year_high,
            year_low: quote.year_low,
            volume: quote.volume,
            avg_volume: quote.avg_volume,
            market_cap: quote.market_cap,
            beta: quote.beta,
            pe: quote.pe,
            eps: quote.eps,
            dividend: quote.dividend,
            dividend_yield: quote.dividend_yield,
            ex_dividend: quote.ex_dividend,
            net_assets: quote.net_assets,
            nav: quote.nav,
            expense_ratio: quote.expense_ratio,
            category: quote.category,
            last_capital_gain: quote.last_capital_gain,
            morningstar_rating: quote.morningstar_rating,
            morningstar_risk_rating: quote.morningstar_risk_rating,
            holdings_turnover: quote.holdings_turnover,
            earnings_date: quote.earnings_date,
            last_dividend: quote.last_dividend,
            inception_date: quote.inception_date,
            sector: quote.sector,
            industry: quote.industry,
            about: quote.about,
            employees: quote.employees,
            five_days_return: quote.five_days_return,
            one_month_return: quote.one_month_return,
            three_month_return: quote.three_month_return,
            six_month_return: quote.six_month_return,
            ytd_return: quote.ytd_return,
            year_return: quote.year_return,
            three_year_return: quote.three_year_return,
            five_year_return: quote.five_year_return,
            ten_year_return: quote.ten_year_return,
            max_return: quote.max_return,
            logo: quote.logo,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Helper strategy for generating optional strings
    fn optional_string() -> impl Strategy<Value = Option<String>> {
        proptest::option::of("[a-zA-Z0-9 .-]{0,20}")
    }

    // Helper strategy for generating optional i64
    fn optional_i64() -> impl Strategy<Value = Option<i64>> {
        proptest::option::of(0i64..1_000_000_000i64)
    }

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn simple_quote_roundtrip(
            symbol in "[A-Z]{1,5}",
            name in "[A-Za-z ]{1,50}",
            price in "[0-9]{1,4}\\.[0-9]{2}",
            pre_market_price in optional_string(),
            after_hours_price in optional_string(),
            change in "-?[0-9]{1,3}\\.[0-9]{2}",
            percent_change in "-?[0-9]{1,3}\\.[0-9]{2}%",
            logo in optional_string(),
        ) {
            let quote = SimpleQuote {
                symbol: symbol.clone(),
                name: name.clone(),
                price: price.clone(),
                pre_market_price: pre_market_price.clone(),
                after_hours_price: after_hours_price.clone(),
                change: change.clone(),
                percent_change: percent_change.clone(),
                logo: logo.clone(),
            };

            let json = serde_json::to_string(&quote).unwrap();
            let parsed: SimpleQuote = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(quote.symbol, parsed.symbol);
            prop_assert_eq!(quote.name, parsed.name);
            prop_assert_eq!(quote.price, parsed.price);
            prop_assert_eq!(quote.pre_market_price, parsed.pre_market_price);
            prop_assert_eq!(quote.after_hours_price, parsed.after_hours_price);
            prop_assert_eq!(quote.change, parsed.change);
            prop_assert_eq!(quote.percent_change, parsed.percent_change);
            prop_assert_eq!(quote.logo, parsed.logo);
        }

        #[test]
        fn quote_roundtrip(
            symbol in "[A-Z]{1,5}",
            name in "[A-Za-z ]{1,50}",
            price in "[0-9]{1,4}\\.[0-9]{2}",
            pre_market_price in optional_string(),
            after_hours_price in optional_string(),
            change in "-?[0-9]{1,3}\\.[0-9]{2}",
            percent_change in "-?[0-9]{1,3}\\.[0-9]{2}%",
            open in optional_string(),
            high in optional_string(),
            low in optional_string(),
            year_high in optional_string(),
            year_low in optional_string(),
            volume in optional_i64(),
            avg_volume in optional_i64(),
            market_cap in optional_string(),
            logo in optional_string(),
        ) {
            let quote = Quote {
                symbol: symbol.clone(),
                name: name.clone(),
                price: price.clone(),
                pre_market_price: pre_market_price.clone(),
                after_hours_price: after_hours_price.clone(),
                change: change.clone(),
                percent_change: percent_change.clone(),
                open: open.clone(),
                high: high.clone(),
                low: low.clone(),
                year_high: year_high.clone(),
                year_low: year_low.clone(),
                volume,
                avg_volume,
                market_cap: market_cap.clone(),
                beta: None,
                pe: None,
                eps: None,
                dividend: None,
                dividend_yield: None,
                ex_dividend: None,
                net_assets: None,
                nav: None,
                expense_ratio: None,
                category: None,
                last_capital_gain: None,
                morningstar_rating: None,
                morningstar_risk_rating: None,
                holdings_turnover: None,
                earnings_date: None,
                last_dividend: None,
                inception_date: None,
                sector: None,
                industry: None,
                about: None,
                employees: None,
                five_days_return: None,
                one_month_return: None,
                three_month_return: None,
                six_month_return: None,
                ytd_return: None,
                year_return: None,
                three_year_return: None,
                five_year_return: None,
                ten_year_return: None,
                max_return: None,
                logo: logo.clone(),
            };

            let json = serde_json::to_string(&quote).unwrap();
            let parsed: Quote = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(quote.symbol, parsed.symbol);
            prop_assert_eq!(quote.name, parsed.name);
            prop_assert_eq!(quote.price, parsed.price);
            prop_assert_eq!(quote.pre_market_price, parsed.pre_market_price);
            prop_assert_eq!(quote.after_hours_price, parsed.after_hours_price);
            prop_assert_eq!(quote.change, parsed.change);
            prop_assert_eq!(quote.percent_change, parsed.percent_change);
            prop_assert_eq!(quote.open, parsed.open);
            prop_assert_eq!(quote.high, parsed.high);
            prop_assert_eq!(quote.low, parsed.low);
            prop_assert_eq!(quote.year_high, parsed.year_high);
            prop_assert_eq!(quote.year_low, parsed.year_low);
            prop_assert_eq!(quote.volume, parsed.volume);
            prop_assert_eq!(quote.avg_volume, parsed.avg_volume);
            prop_assert_eq!(quote.market_cap, parsed.market_cap);
            prop_assert_eq!(quote.logo, parsed.logo);
        }
    }
}
