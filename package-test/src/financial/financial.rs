//! Financial data fetching functions using finance-query-core
//!
//! This module provides functions to fetch financial statement data from Yahoo Finance:
//! - Income statement data (revenue, net income, EPS, etc.)
//! - Balance sheet data (assets, liabilities, equity, etc.)
//! - Cash flow statement data (operating cash flow, free cash flow, etc.)

use finance_query_core::{FetchClient, YahooAuthManager, YahooError, YahooFinanceClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

// ============================================================================
// Financial Statement Field Constants
// ============================================================================

/// Income statement field names for Yahoo Finance API
pub const INCOME_STATEMENT_FIELDS: &[&str] = &[
    "TotalRevenue",
    "OperatingRevenue",
    "CostOfRevenue",
    "GrossProfit",
    "OperatingExpense",
    "SellingGeneralAndAdministration",
    "ResearchAndDevelopment",
    "OperatingIncome",
    "NetInterestIncome",
    "InterestExpense",
    "InterestIncome",
    "NetNonOperatingInterestIncomeExpense",
    "OtherIncomeExpense",
    "PretaxIncome",
    "TaxProvision",
    "NetIncomeCommonStockholders",
    "NetIncome",
    "DilutedEPS",
    "BasicEPS",
    "DilutedAverageShares",
    "BasicAverageShares",
    "EBIT",
    "EBITDA",
    "ReconciledCostOfRevenue",
    "ReconciledDepreciation",
    "NetIncomeFromContinuingOperationNetMinorityInterest",
    "NormalizedEBITDA",
    "TotalExpenses",
    "TotalOperatingIncomeAsReported",
];

/// Balance sheet field names for Yahoo Finance API
pub const BALANCE_SHEET_FIELDS: &[&str] = &[
    "TotalAssets",
    "CurrentAssets",
    "CashCashEquivalentsAndShortTermInvestments",
    "CashAndCashEquivalents",
    "CashFinancial",
    "Receivables",
    "AccountsReceivable",
    "Inventory",
    "PrepaidAssets",
    "OtherCurrentAssets",
    "TotalNonCurrentAssets",
    "NetPPE",
    "GrossPPE",
    "AccumulatedDepreciation",
    "Goodwill",
    "GoodwillAndOtherIntangibleAssets",
    "OtherIntangibleAssets",
    "InvestmentsAndAdvances",
    "LongTermEquityInvestment",
    "OtherNonCurrentAssets",
    "TotalLiabilitiesNetMinorityInterest",
    "CurrentLiabilities",
    "PayablesAndAccruedExpenses",
    "AccountsPayable",
    "CurrentDebt",
    "CurrentDeferredRevenue",
    "OtherCurrentLiabilities",
    "TotalNonCurrentLiabilitiesNetMinorityInterest",
    "LongTermDebt",
    "LongTermDebtAndCapitalLeaseObligation",
    "NonCurrentDeferredRevenue",
    "NonCurrentDeferredTaxesLiabilities",
    "OtherNonCurrentLiabilities",
    "StockholdersEquity",
    "CommonStockEquity",
    "CommonStock",
    "RetainedEarnings",
    "AdditionalPaidInCapital",
    "TreasuryStock",
    "TotalEquityGrossMinorityInterest",
    "WorkingCapital",
    "InvestedCapital",
    "TangibleBookValue",
    "TotalDebt",
    "NetDebt",
    "ShareIssued",
    "OrdinarySharesNumber",
];


/// Cash flow statement field names for Yahoo Finance API
pub const CASH_FLOW_FIELDS: &[&str] = &[
    "OperatingCashFlow",
    "CashFlowFromContinuingOperatingActivities",
    "NetIncomeFromContinuingOperations",
    "DepreciationAndAmortization",
    "DeferredIncomeTax",
    "ChangeInWorkingCapital",
    "ChangeInReceivables",
    "ChangesInAccountReceivables",
    "ChangeInInventory",
    "ChangeInAccountPayable",
    "ChangeInOtherWorkingCapital",
    "StockBasedCompensation",
    "OtherNonCashItems",
    "InvestingCashFlow",
    "CashFlowFromContinuingInvestingActivities",
    "NetPPEPurchaseAndSale",
    "PurchaseOfPPE",
    "SaleOfPPE",
    "CapitalExpenditure",
    "NetBusinessPurchaseAndSale",
    "PurchaseOfBusiness",
    "SaleOfBusiness",
    "NetInvestmentPurchaseAndSale",
    "PurchaseOfInvestment",
    "SaleOfInvestment",
    "NetOtherInvestingChanges",
    "FinancingCashFlow",
    "CashFlowFromContinuingFinancingActivities",
    "NetIssuancePaymentsOfDebt",
    "NetLongTermDebtIssuance",
    "LongTermDebtIssuance",
    "LongTermDebtPayments",
    "NetShortTermDebtIssuance",
    "NetCommonStockIssuance",
    "CommonStockIssuance",
    "CommonStockPayments",
    "RepurchaseOfCapitalStock",
    "CashDividendsPaid",
    "CommonStockDividendPaid",
    "NetOtherFinancingCharges",
    "EndCashPosition",
    "BeginningCashPosition",
    "ChangesinCash",
    "EffectOfExchangeRateChanges",
    "FreeCashFlow",
    "CapitalExpenditureReported",
];

// ============================================================================
// Frequency enum
// ============================================================================

/// Frequency for financial data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    Annual,
    Quarterly,
}

impl Frequency {
    pub fn as_prefix(&self) -> &'static str {
        match self {
            Frequency::Annual => "annual",
            Frequency::Quarterly => "quarterly",
        }
    }
}

// ============================================================================
// Response Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialsResponse {
    pub timeseries: TimeseriesResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeseriesResult {
    pub error: Option<Value>,
    pub result: Option<Vec<TimeseriesData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeseriesData {
    pub meta: TimeseriesMeta,
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, Option<Vec<TimeseriesValue>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesMeta {
    pub symbol: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub data_type: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesValue {
    pub as_of_date: Option<String>,
    pub period_type: Option<String>,
    pub reported_value: Option<ReportedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportedValue {
    pub raw: Option<f64>,
    pub fmt: Option<String>,
}

// ============================================================================
// Simplified Financial Data Types
// ============================================================================

/// A single financial data point with date and value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialDataPoint {
    pub date: String,
    pub period_type: String,
    pub value: f64,
    pub formatted: Option<String>,
}

/// Parsed financial statement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatement {
    pub symbol: String,
    pub statement_type: String,
    pub frequency: String,
    pub metrics: std::collections::HashMap<String, Vec<FinancialDataPoint>>,
}

impl FinancialStatement {
    /// Get a specific metric's data points
    pub fn get_metric(&self, name: &str) -> Option<&Vec<FinancialDataPoint>> {
        self.metrics.get(name)
    }

    /// Get the latest value for a metric
    pub fn latest(&self, name: &str) -> Option<f64> {
        self.metrics
            .get(name)
            .and_then(|points| points.first())
            .map(|p| p.value)
    }

    /// Get all available metric names
    pub fn metric_names(&self) -> Vec<&String> {
        self.metrics.keys().collect()
    }
}


// ============================================================================
// Helper Functions
// ============================================================================

/// Creates a configured YahooFinanceClient
async fn create_client() -> Result<(Arc<YahooAuthManager>, YahooFinanceClient), YahooError> {
    let fetch_client = Arc::new(FetchClient::new(None)?);
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    let client = YahooFinanceClient::new(auth_manager.clone(), fetch_client);

    // Prime authentication
    auth_manager.refresh().await?;

    Ok((auth_manager, client))
}

/// Build field names with frequency prefix
fn build_fields(fields: &[&str], frequency: Frequency) -> Vec<String> {
    let prefix = frequency.as_prefix();
    fields.iter().map(|f| format!("{}{}", prefix, f)).collect()
}

/// Parse raw JSON response into FinancialStatement
fn parse_financials(
    json: &Value,
    symbol: &str,
    statement_type: &str,
    frequency: Frequency,
) -> Result<FinancialStatement, YahooError> {
    let mut metrics: std::collections::HashMap<String, Vec<FinancialDataPoint>> =
        std::collections::HashMap::new();

    if let Some(result) = json
        .get("timeseries")
        .and_then(|t| t.get("result"))
        .and_then(|r| r.as_array())
    {
        for item in result {
            // Get the metric name from meta.type
            if let Some(types) = item
                .get("meta")
                .and_then(|m| m.get("type"))
                .and_then(|t| t.as_array())
            {
                for type_val in types {
                    if let Some(metric_name) = type_val.as_str() {
                        // Look for the data array with this metric name
                        if let Some(data_array) = item.get(metric_name).and_then(|d| d.as_array()) {
                            let mut points: Vec<FinancialDataPoint> = Vec::new();

                            for data_point in data_array {
                                if let (Some(date), Some(reported)) = (
                                    data_point.get("asOfDate").and_then(|d| d.as_str()),
                                    data_point.get("reportedValue"),
                                ) {
                                    let value = reported
                                        .get("raw")
                                        .and_then(|r| r.as_f64())
                                        .unwrap_or(0.0);
                                    let formatted =
                                        reported.get("fmt").and_then(|f| f.as_str()).map(String::from);
                                    let period_type = data_point
                                        .get("periodType")
                                        .and_then(|p| p.as_str())
                                        .unwrap_or("12M")
                                        .to_string();

                                    points.push(FinancialDataPoint {
                                        date: date.to_string(),
                                        period_type,
                                        value,
                                        formatted,
                                    });
                                }
                            }

                            // Sort by date descending (most recent first)
                            points.sort_by(|a, b| b.date.cmp(&a.date));

                            // Strip the frequency prefix from metric name for cleaner access
                            let clean_name = metric_name
                                .strip_prefix("annual")
                                .or_else(|| metric_name.strip_prefix("quarterly"))
                                .unwrap_or(metric_name)
                                .to_string();

                            if !points.is_empty() {
                                metrics.insert(clean_name, points);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(FinancialStatement {
        symbol: symbol.to_string(),
        statement_type: statement_type.to_string(),
        frequency: frequency.as_prefix().to_string(),
        metrics,
    })
}


// ============================================================================
// Public API Functions
// ============================================================================

/// Get raw financials timeseries data for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `fields` - Slice of field names to fetch (without frequency prefix)
/// * `frequency` - Annual or Quarterly
/// * `years` - Number of years of historical data
///
/// # Returns
/// Raw JSON value containing financials timeseries data
pub async fn get_financials_raw(
    symbol: &str,
    fields: &[&str],
    frequency: Frequency,
    years: i64,
) -> Result<Value, YahooError> {
    let (_, client) = create_client().await?;

    let now = chrono::Utc::now().timestamp();
    let period1 = now - (years * 365 * 24 * 60 * 60);

    let prefixed_fields = build_fields(fields, frequency);
    let field_refs: Vec<&str> = prefixed_fields.iter().map(|s| s.as_str()).collect();

    client
        .get_fundamentals_timeseries(symbol, period1, now, &field_refs)
        .await
}

/// Get income statement data for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `frequency` - Annual or Quarterly
/// * `years` - Number of years of historical data (default 5)
///
/// # Returns
/// FinancialStatement with parsed income statement metrics
///
/// # Example
/// ```rust,ignore
/// let income = get_income_statement("AAPL", Frequency::Annual, 5).await?;
/// if let Some(revenue) = income.latest("TotalRevenue") {
///     println!("Latest revenue: ${:.0}", revenue);
/// }
/// ```
pub async fn get_income_statement(
    symbol: &str,
    frequency: Frequency,
    years: i64,
) -> Result<FinancialStatement, YahooError> {
    let json = get_financials_raw(symbol, INCOME_STATEMENT_FIELDS, frequency, years).await?;
    parse_financials(&json, symbol, "income_statement", frequency)
}

/// Get balance sheet data for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `frequency` - Annual or Quarterly
/// * `years` - Number of years of historical data (default 5)
///
/// # Returns
/// FinancialStatement with parsed balance sheet metrics
///
/// # Example
/// ```rust,ignore
/// let balance = get_balance_sheet("AAPL", Frequency::Annual, 5).await?;
/// if let Some(assets) = balance.latest("TotalAssets") {
///     println!("Total assets: ${:.0}", assets);
/// }
/// ```
pub async fn get_balance_sheet(
    symbol: &str,
    frequency: Frequency,
    years: i64,
) -> Result<FinancialStatement, YahooError> {
    let json = get_financials_raw(symbol, BALANCE_SHEET_FIELDS, frequency, years).await?;
    parse_financials(&json, symbol, "balance_sheet", frequency)
}

/// Get cash flow statement data for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `frequency` - Annual or Quarterly
/// * `years` - Number of years of historical data (default 5)
///
/// # Returns
/// FinancialStatement with parsed cash flow metrics
///
/// # Example
/// ```rust,ignore
/// let cashflow = get_cash_flow("AAPL", Frequency::Annual, 5).await?;
/// if let Some(fcf) = cashflow.latest("FreeCashFlow") {
///     println!("Free cash flow: ${:.0}", fcf);
/// }
/// ```
pub async fn get_cash_flow(
    symbol: &str,
    frequency: Frequency,
    years: i64,
) -> Result<FinancialStatement, YahooError> {
    let json = get_financials_raw(symbol, CASH_FLOW_FIELDS, frequency, years).await?;
    parse_financials(&json, symbol, "cash_flow", frequency)
}

/// Get all financial statements (income, balance sheet, cash flow) for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol (e.g., "AAPL")
/// * `frequency` - Annual or Quarterly
/// * `years` - Number of years of historical data
///
/// # Returns
/// Tuple of (income_statement, balance_sheet, cash_flow)
pub async fn get_all_financials(
    symbol: &str,
    frequency: Frequency,
    years: i64,
) -> Result<(FinancialStatement, FinancialStatement, FinancialStatement), YahooError> {
    let income = get_income_statement(symbol, frequency, years).await?;
    let balance = get_balance_sheet(symbol, frequency, years).await?;
    let cashflow = get_cash_flow(symbol, frequency, years).await?;

    Ok((income, balance, cashflow))
}

/// Get specific financial metrics for a symbol
///
/// # Arguments
/// * `symbol` - The stock symbol
/// * `fields` - Slice of field names (without frequency prefix)
/// * `frequency` - Annual or Quarterly
/// * `years` - Number of years of historical data
///
/// # Returns
/// FinancialStatement with only the requested metrics
///
/// # Example
/// ```rust,ignore
/// let data = get_custom_financials(
///     "AAPL",
///     &["TotalRevenue", "NetIncome", "FreeCashFlow"],
///     Frequency::Annual,
///     5
/// ).await?;
/// ```
pub async fn get_custom_financials(
    symbol: &str,
    fields: &[&str],
    frequency: Frequency,
    years: i64,
) -> Result<FinancialStatement, YahooError> {
    let json = get_financials_raw(symbol, fields, frequency, years).await?;
    parse_financials(&json, symbol, "custom", frequency)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require network access and valid Yahoo Finance authentication
    // Run with: cargo test -p package-test -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_get_income_statement() {
        let result = get_income_statement("AAPL", Frequency::Annual, 5).await;
        assert!(result.is_ok());
        let income = result.unwrap();
        assert!(!income.metrics.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_balance_sheet() {
        let result = get_balance_sheet("AAPL", Frequency::Annual, 5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_cash_flow() {
        let result = get_cash_flow("AAPL", Frequency::Annual, 5).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_quarterly_data() {
        let result = get_income_statement("AAPL", Frequency::Quarterly, 2).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_custom_financials() {
        let result = get_custom_financials(
            "AAPL",
            &["TotalRevenue", "NetIncome", "FreeCashFlow"],
            Frequency::Annual,
            5,
        )
        .await;
        assert!(result.is_ok());
    }
}
