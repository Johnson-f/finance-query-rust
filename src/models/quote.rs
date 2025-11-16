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

