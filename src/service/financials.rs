use crate::client::YahooFinanceClient;
use crate::client::error::YahooError;
use crate::models::{FinancialStatement, StatementType, Frequency};
use serde_json::Value;

pub async fn get_financial_statement(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
    statement_type: StatementType,
    frequency: Frequency,
) -> Result<FinancialStatement, YahooError> {
    // Map statement type to Yahoo Finance types
    let types = match statement_type {
        StatementType::IncomeStatement => {
            if matches!(frequency, Frequency::Annual) {
                vec!["annualTotalRevenue", "annualNetIncome"]
            } else {
                vec!["quarterlyTotalRevenue", "quarterlyNetIncome"]
            }
        }
        StatementType::BalanceSheet => {
            if matches!(frequency, Frequency::Annual) {
                vec!["annualTotalAssets", "annualTotalLiabilities"]
            } else {
                vec!["quarterlyTotalAssets", "quarterlyTotalLiabilities"]
            }
        }
        StatementType::CashFlow => {
            if matches!(frequency, Frequency::Annual) {
                vec!["annualOperatingCashFlow", "annualFreeCashFlow"]
            } else {
                vec!["quarterlyOperatingCashFlow", "quarterlyFreeCashFlow"]
            }
        }
    };

    // Get current time and 5 years ago
    let now = chrono::Utc::now().timestamp();
    let five_years_ago = now - (5 * 365 * 24 * 60 * 60);

    let data = yahoo_client
        .get_fundamentals_timeseries(symbol, five_years_ago, now, &types.iter().map(|s| *s).collect::<Vec<_>>())
        .await?;

    Ok(FinancialStatement {
        symbol: symbol.to_string(),
        statement_type: statement_type.as_str().to_string(),
        frequency: frequency.as_str().to_string(),
        data,
    })
}

