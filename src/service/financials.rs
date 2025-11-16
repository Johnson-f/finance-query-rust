use crate::client::YahooFinanceClient;
use crate::client::error::YahooError;
use crate::models::{FinancialStatement, StatementType, Frequency};
use crate::utils::financials_constants::get_statement_fields;
use serde_json::Value;
use std::collections::HashMap;
use tracing::debug;

/// Map statement type to internal key string
fn map_statement_type_to_key(statement_type: StatementType) -> &'static str {
    match statement_type {
        StatementType::IncomeStatement => "income",
        StatementType::BalanceSheet => "balance",
        StatementType::CashFlow => "cashflow",
    }
}

/// Parse Yahoo Finance timeseries response into financial statement format.
/// 
/// Transforms the API response into a format where each metric is a key,
/// and the value is a dictionary of date->value mappings.
fn parse_timeseries_data(timeseries_result: &[Value]) -> HashMap<String, HashMap<String, Value>> {
    let mut parsed_data: HashMap<String, HashMap<String, Value>> = HashMap::new();

    for item in timeseries_result {
        // Get the metric name (e.g., 'annualTotalRevenue')
        let metric_name_with_prefix = item
            .get("meta")
            .and_then(|m| m.get("type"))
            .and_then(|t| t.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if metric_name_with_prefix.is_empty() {
            continue;
        }

        // Remove frequency prefix (annual/quarterly/trailing) for storage
        let mut metric_name = metric_name_with_prefix.to_string();
        for prefix in ["annual", "quarterly", "trailing"] {
            if metric_name.starts_with(prefix) {
                metric_name = metric_name[prefix.len()..].to_string();
                break;
            }
        }

        // Extract timestamp data using the original name with prefix
        let mut timestamp_data: HashMap<String, Value> = HashMap::new();
        
        if let Some(datapoints) = item.get(metric_name_with_prefix).and_then(|v| v.as_array()) {
            for datapoint in datapoints {
                // Skip null entries
                if datapoint.is_null() {
                    continue;
                }

                let as_of_date = datapoint.get("asOfDate");
                let reported_value = datapoint.get("reportedValue");

                if let Some(date) = as_of_date {
                    let date_key = if let Some(date_str) = date.as_str() {
                        date_str.to_string()
                    } else if let Some(date_num) = date.as_i64() {
                        date_num.to_string()
                    } else if let Some(date_f64) = date.as_f64() {
                        date_f64.to_string()
                    } else {
                        continue;
                    };

                    // Handle raw value
                    if let Some(reported) = reported_value {
                        let value = if let Some(raw_obj) = reported.as_object() {
                            // Check if raw is nested with parsedValue
                            if let Some(raw) = raw_obj.get("raw") {
                                if let Some(raw_obj_inner) = raw.as_object() {
                                    if raw_obj_inner.contains_key("parsedValue") {
                                        raw_obj_inner.get("parsedValue").cloned()
                                    } else {
                                        Some(raw.clone())
                                    }
                                } else {
                                    Some(raw.clone())
                                }
                            } else {
                                Some(reported.clone())
                            }
                        } else {
                            Some(reported.clone())
                        };

                        if let Some(val) = value {
                            timestamp_data.insert(date_key, val);
                        }
                    }
                }
            }
        }

        if !timestamp_data.is_empty() {
            parsed_data.insert(metric_name, timestamp_data);
        }
    }

    parsed_data
}

pub async fn get_financial_statement(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
    statement_type: StatementType,
    frequency: Frequency,
) -> Result<FinancialStatement, YahooError> {
    // Get appropriate fields for statement type
    let statement_key = map_statement_type_to_key(statement_type);
    let fields = get_statement_fields(statement_key, frequency.as_str());
    
    // Convert to slice of string references for the API call
    let field_refs: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();

    // Define time range (go back ~10 years, matching Python implementation)
    let period2 = chrono::Utc::now().timestamp();
    let period1 = period2 - (10 * 365 * 24 * 60 * 60); // 10 years ago

    debug!("Fetching financial statement for {}: type={}, frequency={}, fields={}", 
           symbol, statement_key, frequency.as_str(), fields.len());

    // Fetch data from Yahoo Finance
    let response = yahoo_client
        .get_fundamentals_timeseries(symbol, period1, period2, &field_refs)
        .await?;

    // Parse the response
    let timeseries_data = response
        .get("timeseries")
        .and_then(|ts| ts.get("result"))
        .and_then(|r| r.as_array())
        .ok_or_else(|| {
            YahooError::ParseError(format!(
                "No timeseries data found for {} {} statement",
                symbol, statement_key
            ))
        })?;

    if timeseries_data.is_empty() {
        return Err(YahooError::ParseError(format!(
            "No {} data found for {}",
            statement_key, symbol
        )));
    }

    // Transform to our format
    let parsed_statement = parse_timeseries_data(timeseries_data);

    if parsed_statement.is_empty() {
        return Err(YahooError::ParseError(format!(
            "No {} data found for {}",
            statement_key, symbol
        )));
    }

    debug!("Successfully parsed {} metrics for {} {} statement", 
           parsed_statement.len(), symbol, statement_key);

    Ok(FinancialStatement {
        symbol: symbol.to_uppercase(),
        statement_type: statement_key.to_string(),
        frequency: frequency.as_str().to_string(),
        statement: parsed_statement,
    })
}