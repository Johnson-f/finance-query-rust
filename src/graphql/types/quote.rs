use async_graphql::*;
use finance_query_core::models::quote::{Quote as QuoteModel, SimpleQuote as SimpleQuoteModel, DetailedQuote as DetailedQuoteModel};

#[derive(SimpleObject, Clone)]
pub struct Quote {
    pub symbol: String,
    pub name: String,
    pub price: String,
    pub pre_market_price: Option<String>,
    pub after_hours_price: Option<String>,
    pub change: String,
    pub percent_change: String,
    pub open: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    pub year_high: Option<String>,
    pub year_low: Option<String>,
    pub volume: Option<i64>,
    pub avg_volume: Option<i64>,
    pub market_cap: Option<String>,
    pub beta: Option<String>,
    pub pe: Option<String>,
    pub eps: Option<String>,
    pub dividend: Option<String>,
    pub dividend_yield: Option<String>,
    pub ex_dividend: Option<String>,
    pub net_assets: Option<String>,
    pub nav: Option<String>,
    pub expense_ratio: Option<String>,
    pub category: Option<String>,
    pub last_capital_gain: Option<String>,
    pub morningstar_rating: Option<String>,
    pub morningstar_risk_rating: Option<String>,
    pub holdings_turnover: Option<String>,
    pub earnings_date: Option<String>,
    pub last_dividend: Option<String>,
    pub inception_date: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub about: Option<String>,
    pub employees: Option<String>,
    pub five_days_return: Option<String>,
    pub one_month_return: Option<String>,
    pub three_month_return: Option<String>,
    pub six_month_return: Option<String>,
    pub ytd_return: Option<String>,
    pub year_return: Option<String>,
    pub three_year_return: Option<String>,
    pub five_year_return: Option<String>,
    pub ten_year_return: Option<String>,
    pub max_return: Option<String>,
    pub logo: Option<String>,
}

impl From<QuoteModel> for Quote {
    fn from(quote: QuoteModel) -> Self {
        Quote {
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

#[derive(SimpleObject, Clone)]
pub struct SimpleQuote {
    pub symbol: String,
    pub name: String,
    pub price: String,
    pub pre_market_price: Option<String>,
    pub after_hours_price: Option<String>,
    pub change: String,
    pub percent_change: String,
    pub logo: Option<String>,
}

impl From<SimpleQuoteModel> for SimpleQuote {
    fn from(quote: SimpleQuoteModel) -> Self {
        SimpleQuote {
            symbol: quote.symbol,
            name: quote.name,
            price: quote.price,
            pre_market_price: quote.pre_market_price,
            after_hours_price: quote.after_hours_price,
            change: quote.change,
            percent_change: quote.percent_change,
            logo: quote.logo,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct DetailedQuote {
    pub symbol: String,
    pub name: String,
    pub price: String,
    #[graphql(name = "preMarketPrice")]
    pub pre_market_price: Option<String>,
    #[graphql(name = "afterHoursPrice")]
    pub after_hours_price: Option<String>,
    pub change: String,
    #[graphql(name = "percentChange")]
    pub percent_change: String,
    pub open: Option<String>,
    pub high: Option<String>,
    pub low: Option<String>,
    #[graphql(name = "yearHigh")]
    pub year_high: Option<String>,
    #[graphql(name = "yearLow")]
    pub year_low: Option<String>,
    pub volume: Option<i64>,
    #[graphql(name = "avgVolume")]
    pub avg_volume: Option<i64>,
    #[graphql(name = "marketCap")]
    pub market_cap: Option<String>,
    pub beta: Option<String>,
    pub pe: Option<String>,
    pub eps: Option<String>,
    pub dividend: Option<String>,
    #[graphql(name = "yield")]
    pub dividend_yield: Option<String>,
    #[graphql(name = "exDividend")]
    pub ex_dividend: Option<String>,
    #[graphql(name = "netAssets")]
    pub net_assets: Option<String>,
    pub nav: Option<String>,
    #[graphql(name = "expenseRatio")]
    pub expense_ratio: Option<String>,
    pub category: Option<String>,
    #[graphql(name = "lastCapitalGain")]
    pub last_capital_gain: Option<String>,
    #[graphql(name = "morningstarRating")]
    pub morningstar_rating: Option<String>,
    #[graphql(name = "morningstarRiskRating")]
    pub morningstar_risk_rating: Option<String>,
    #[graphql(name = "holdingsTurnover")]
    pub holdings_turnover: Option<String>,
    #[graphql(name = "earningsDate")]
    pub earnings_date: Option<String>,
    #[graphql(name = "lastDividend")]
    pub last_dividend: Option<String>,
    #[graphql(name = "inceptionDate")]
    pub inception_date: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub about: Option<String>,
    pub employees: Option<String>,
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
    pub logo: Option<String>,
}

impl From<DetailedQuoteModel> for DetailedQuote {
    fn from(quote: DetailedQuoteModel) -> Self {
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