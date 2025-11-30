use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a single option contract
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    pub contract_symbol: String,
    pub last_trade_date: DateTime<Utc>,
    pub strike: f64,
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub change: f64,
    pub percent_change: f64,
    pub volume: Option<u64>,
    pub open_interest: Option<u64>,
    pub implied_volatility: f64,
    pub in_the_money: bool,
    pub contract_size: String,
    pub currency: String,
}

/// Complete option chain for a symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub expiration_date: String,
    pub calls: Vec<OptionContract>,
    pub puts: Vec<OptionContract>,
    pub underlying_price: Option<f64>,
}

impl OptionChain {
    pub(crate) fn from_yahoo_response(
        symbol: String,
        expiration_date: String,
        response: YahooOptionsResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let result = response.option_chain.result.first().ok_or_else(|| {
            crate::client::YahooError::ParseError("No option chain data in response".to_string())
        })?;

        let options_data = result.options.first().ok_or_else(|| {
            crate::client::YahooError::ParseError(
                "No options data for expiration".to_string(),
            )
        })?;

        let calls = options_data
            .calls
            .iter()
            .map(parse_option_contract)
            .collect::<Result<Vec<_>, _>>()?;

        let puts = options_data
            .puts
            .iter()
            .map(parse_option_contract)
            .collect::<Result<Vec<_>, _>>()?;

        let underlying_price = result
            .quote
            .as_ref()
            .and_then(|q| q.get("regularMarketPrice"))
            .and_then(|p| p.as_f64());

        Ok(Self {
            symbol,
            expiration_date,
            calls,
            puts,
            underlying_price,
        })
    }
}

/// List of available expiration dates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionExpirations {
    pub symbol: String,
    pub expirations: Vec<String>, // YYYY-MM-DD format
}

impl OptionExpirations {
    pub(crate) fn from_yahoo_response(
        symbol: String,
        response: YahooOptionsResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let result = response.option_chain.result.first().ok_or_else(|| {
            crate::client::YahooError::ParseError("No option chain data".to_string())
        })?;

        let expirations = result
            .expiration_dates
            .iter()
            .map(|&ts| {
                Utc.timestamp_opt(ts, 0)
                    .single()
                    .map(|dt| dt.format("%Y-%m-%d").to_string())
                    .ok_or_else(|| {
                        crate::client::YahooError::ParseError(
                            "Invalid expiration timestamp".to_string(),
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            symbol,
            expirations,
        })
    }
}

fn parse_option_contract(
    contract: &YahooOptionContract,
) -> Result<OptionContract, crate::client::YahooError> {
    Ok(OptionContract {
        contract_symbol: contract.contract_symbol.clone(),
        last_trade_date: Utc
            .timestamp_opt(contract.last_trade_date, 0)
            .single()
            .ok_or_else(|| {
                crate::client::YahooError::ParseError("Invalid last trade date".to_string())
            })?,
        strike: contract.strike,
        last_price: contract.last_price,
        bid: contract.bid,
        ask: contract.ask,
        change: contract.change,
        percent_change: contract.percent_change,
        volume: contract.volume,
        open_interest: contract.open_interest,
        implied_volatility: contract.implied_volatility,
        in_the_money: contract.in_the_money,
        contract_size: contract.contract_size.clone(),
        currency: contract.currency.clone(),
    })
}

// Internal Yahoo response structures
#[derive(Debug, Deserialize)]
pub(crate) struct YahooOptionsResponse {
    #[serde(rename = "optionChain")]
    pub option_chain: OptionChainData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OptionChainData {
    pub result: Vec<OptionChainResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OptionChainResult {
    #[serde(rename = "expirationDates")]
    pub expiration_dates: Vec<i64>,
    pub options: Vec<OptionsData>,
    pub quote: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OptionsData {
    pub calls: Vec<YahooOptionContract>,
    pub puts: Vec<YahooOptionContract>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooOptionContract {
    #[serde(rename = "contractSymbol")]
    pub contract_symbol: String,
    #[serde(rename = "lastTradeDate")]
    pub last_trade_date: i64,
    pub strike: f64,
    #[serde(rename = "lastPrice")]
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub change: f64,
    #[serde(rename = "percentChange")]
    pub percent_change: f64,
    pub volume: Option<u64>,
    #[serde(rename = "openInterest")]
    pub open_interest: Option<u64>,
    #[serde(rename = "impliedVolatility")]
    pub implied_volatility: f64,
    #[serde(rename = "inTheMoney")]
    pub in_the_money: bool,
    #[serde(rename = "contractSize")]
    pub contract_size: String,
    pub currency: String,
}

/// Helper to convert date string to Unix timestamp
pub(crate) fn date_to_timestamp(date_str: &str) -> Result<i64, crate::client::YahooError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| crate::client::YahooError::ParseError(format!("Invalid date format: {}", e)))?
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| crate::client::YahooError::ParseError("Invalid time".to_string()))?
        .and_local_timezone(Utc)
        .single()
        .ok_or_else(|| crate::client::YahooError::ParseError("Invalid timezone".to_string()))
        .map(|dt| dt.timestamp())
}
