use chrono::{DateTime, Utc};
use finance_query_core::client::{YahooFinanceClient, error::YahooError};
use finance_query_core::models::holders::{
    HolderType, HoldersData, InsiderPurchase, InsiderRosterMember, InsiderTransaction,
    InstitutionalHolder, MajorHoldersBreakdown, MutualFundHolder,
};
use serde_json::Value;
use std::collections::HashMap;
use tracing::info;

// Module mapping for Yahoo Finance quoteSummary API
fn get_modules_for_holder_type(holder_type: HolderType) -> Vec<&'static str> {
    match holder_type {
        HolderType::Major => vec!["majorHoldersBreakdown"],
        HolderType::Institutional => vec!["institutionOwnership"],
        HolderType::MutualFund => vec!["fundOwnership"],
        HolderType::InsiderTransactions => vec!["insiderTransactions"],
        HolderType::InsiderPurchases => vec!["netSharePurchaseActivity"],
        HolderType::InsiderRoster => vec!["insiderHolders"],
    }
}

/// Get holders data for a symbol
pub async fn get_holders_data(
    yahoo_client: &YahooFinanceClient,
    symbol: &str,
    holder_type: HolderType,
) -> Result<HoldersData, YahooError> {
    info!(
        "Fetching {} holders data for {}",
        holder_type.as_str(),
        symbol
    );

    let modules = get_modules_for_holder_type(holder_type);
    let modules_refs: Vec<&str> = modules.to_vec();

    // Fetch data from Yahoo Finance
    let response = yahoo_client
        .get_quote_summary(&symbol.to_uppercase(), &modules_refs)
        .await?;

    // Extract the result
    let result = response
        .get("quoteSummary")
        .and_then(|qs| qs.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .ok_or_else(|| {
            YahooError::ParseError(format!(
                "No {} data found for {}",
                holder_type.as_str(),
                symbol
            ))
        })?;

    // Parse based on holder type
    let holders_data = match holder_type {
        HolderType::Major => {
            let breakdown = parse_major_breakdown(result)?;
            HoldersData {
                symbol: symbol.to_uppercase(),
                holder_type,
                major_breakdown: Some(breakdown),
                institutional_holders: None,
                mutualfund_holders: None,
                insider_transactions: None,
                insider_purchases: None,
                insider_roster: None,
            }
        }
        HolderType::Institutional => {
            let holders = parse_institutional_holders(result)?;
            HoldersData {
                symbol: symbol.to_uppercase(),
                holder_type,
                major_breakdown: None,
                institutional_holders: Some(holders),
                mutualfund_holders: None,
                insider_transactions: None,
                insider_purchases: None,
                insider_roster: None,
            }
        }
        HolderType::MutualFund => {
            let holders = parse_mutualfund_holders(result)?;
            HoldersData {
                symbol: symbol.to_uppercase(),
                holder_type,
                major_breakdown: None,
                institutional_holders: None,
                mutualfund_holders: Some(holders),
                insider_transactions: None,
                insider_purchases: None,
                insider_roster: None,
            }
        }
        HolderType::InsiderTransactions => {
            let transactions = parse_insider_transactions(result)?;
            HoldersData {
                symbol: symbol.to_uppercase(),
                holder_type,
                major_breakdown: None,
                institutional_holders: None,
                mutualfund_holders: None,
                insider_transactions: Some(transactions),
                insider_purchases: None,
                insider_roster: None,
            }
        }
        HolderType::InsiderPurchases => {
            let purchases = parse_insider_purchases(result)?;
            HoldersData {
                symbol: symbol.to_uppercase(),
                holder_type,
                major_breakdown: None,
                institutional_holders: None,
                mutualfund_holders: None,
                insider_transactions: None,
                insider_purchases: Some(purchases),
                insider_roster: None,
            }
        }
        HolderType::InsiderRoster => {
            let roster = parse_insider_roster(result)?;
            HoldersData {
                symbol: symbol.to_uppercase(),
                holder_type,
                major_breakdown: None,
                institutional_holders: None,
                mutualfund_holders: None,
                insider_transactions: None,
                insider_purchases: None,
                insider_roster: Some(roster),
            }
        }
    };

    info!(
        "Successfully parsed {} holders data for {}",
        holder_type.as_str(),
        symbol
    );
    Ok(holders_data)
}

fn parse_major_breakdown(data: &Value) -> Result<MajorHoldersBreakdown, YahooError> {
    let breakdown_data_obj = data
        .get("majorHoldersBreakdown")
        .ok_or_else(|| YahooError::ParseError("No majorHoldersBreakdown data found".to_string()))?;

    let mut breakdown_data = HashMap::new();

    if let Some(val) = breakdown_data_obj.get("insidersPercentHeld")
        && let Some(raw) = val.as_f64()
    {
        breakdown_data.insert("insidersPercentHeld".to_string(), serde_json::json!(raw));
    }

    if let Some(val) = breakdown_data_obj.get("institutionsPercentHeld")
        && let Some(raw) = val.as_f64()
    {
        breakdown_data.insert(
            "institutionsPercentHeld".to_string(),
            serde_json::json!(raw),
        );
    }

    if let Some(val) = breakdown_data_obj.get("institutionsFloatPercentHeld")
        && let Some(raw) = val.as_f64()
    {
        breakdown_data.insert(
            "institutionsFloatPercentHeld".to_string(),
            serde_json::json!(raw),
        );
    }

    if let Some(val) = breakdown_data_obj.get("institutionsCount")
        && let Some(raw) = val.as_i64()
    {
        breakdown_data.insert("institutionsCount".to_string(), serde_json::json!(raw));
    }

    Ok(MajorHoldersBreakdown { breakdown_data })
}

fn parse_institutional_holders(data: &Value) -> Result<Vec<InstitutionalHolder>, YahooError> {
    let empty_vec = Vec::new();
    let holders_list = data
        .get("institutionOwnership")
        .and_then(|io| io.get("ownershipList"))
        .and_then(|ol| ol.as_array())
        .unwrap_or(&empty_vec);

    let mut holders = Vec::new();

    for holder_data in holders_list {
        let date_reported = holder_data
            .get("reportDate")
            .and_then(|rd| rd.get("raw"))
            .and_then(|r| r.as_i64())
            .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_else(Utc::now))
            .unwrap_or_else(Utc::now);

        let holder = InstitutionalHolder {
            holder: holder_data
                .get("organization")
                .and_then(|o| o.as_str())
                .unwrap_or("")
                .to_string(),
            shares: holder_data
                .get("position")
                .and_then(|p| p.get("raw"))
                .and_then(|r| r.as_i64())
                .unwrap_or(0),
            date_reported,
            percent_out: holder_data
                .get("pctHeld")
                .and_then(|p| p.get("raw"))
                .and_then(|r| r.as_f64()),
            value: holder_data
                .get("value")
                .and_then(|v| v.get("raw"))
                .and_then(|r| r.as_i64()),
        };

        holders.push(holder);
    }

    Ok(holders)
}

fn parse_mutualfund_holders(data: &Value) -> Result<Vec<MutualFundHolder>, YahooError> {
    let empty_vec = Vec::new();
    let holders_list = data
        .get("fundOwnership")
        .and_then(|fo| fo.get("ownershipList"))
        .and_then(|ol| ol.as_array())
        .unwrap_or(&empty_vec);

    let mut holders = Vec::new();

    for holder_data in holders_list {
        let date_reported = holder_data
            .get("reportDate")
            .and_then(|rd| rd.get("raw"))
            .and_then(|r| r.as_i64())
            .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_else(Utc::now))
            .unwrap_or_else(Utc::now);

        let holder = MutualFundHolder {
            holder: holder_data
                .get("organization")
                .and_then(|o| o.as_str())
                .unwrap_or("")
                .to_string(),
            shares: holder_data
                .get("position")
                .and_then(|p| p.get("raw"))
                .and_then(|r| r.as_i64())
                .unwrap_or(0),
            date_reported,
            percent_out: holder_data
                .get("pctHeld")
                .and_then(|p| p.get("raw"))
                .and_then(|r| r.as_f64()),
            value: holder_data
                .get("value")
                .and_then(|v| v.get("raw"))
                .and_then(|r| r.as_i64()),
        };

        holders.push(holder);
    }

    Ok(holders)
}

fn parse_insider_transactions(data: &Value) -> Result<Vec<InsiderTransaction>, YahooError> {
    let empty_vec = Vec::new();
    let transactions_list = data
        .get("insiderTransactions")
        .and_then(|it| it.get("transactions"))
        .and_then(|t| t.as_array())
        .unwrap_or(&empty_vec);

    let mut transactions = Vec::new();

    for trans_data in transactions_list {
        let start_date = trans_data
            .get("startDate")
            .and_then(|sd| sd.get("raw"))
            .and_then(|r| r.as_i64())
            .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_else(Utc::now))
            .unwrap_or_else(Utc::now);

        let transaction = InsiderTransaction {
            start_date,
            insider: trans_data
                .get("filerName")
                .and_then(|name| name.as_str())
                .unwrap_or("")
                .to_string(),
            position: trans_data
                .get("filerRelation")
                .and_then(|fr| fr.as_str())
                .unwrap_or("")
                .to_string(),
            transaction: trans_data
                .get("transactionText")
                .and_then(|tt| tt.as_str())
                .unwrap_or("")
                .to_string(),
            shares: trans_data
                .get("shares")
                .and_then(|s| s.get("raw"))
                .and_then(|r| r.as_i64()),
            value: trans_data
                .get("value")
                .and_then(|v| v.get("raw"))
                .and_then(|r| r.as_i64()),
            ownership: trans_data
                .get("ownership")
                .and_then(|o| o.as_str())
                .map(|s| s.to_string()),
        };

        transactions.push(transaction);
    }

    Ok(transactions)
}

fn parse_insider_purchases(data: &Value) -> Result<InsiderPurchase, YahooError> {
    let purchase_data = data.get("netSharePurchaseActivity").ok_or_else(|| {
        YahooError::ParseError("No netSharePurchaseActivity data found".to_string())
    })?;

    Ok(InsiderPurchase {
        period: purchase_data
            .get("period")
            .and_then(|p| p.as_str())
            .unwrap_or("Unknown")
            .to_string(),
        purchases_shares: purchase_data
            .get("buyInfoShares")
            .and_then(|bis| bis.get("raw"))
            .and_then(|r| r.as_i64()),
        purchases_transactions: purchase_data
            .get("buyInfoCount")
            .and_then(|bic| bic.get("raw"))
            .and_then(|r| r.as_i64()),
        sales_shares: purchase_data
            .get("sellInfoShares")
            .and_then(|sis| sis.get("raw"))
            .and_then(|r| r.as_i64()),
        sales_transactions: purchase_data
            .get("sellInfoCount")
            .and_then(|sic| sic.get("raw"))
            .and_then(|r| r.as_i64()),
        net_shares: purchase_data
            .get("netInfoShares")
            .and_then(|nis| nis.get("raw"))
            .and_then(|r| r.as_i64()),
        net_transactions: purchase_data
            .get("netInfoCount")
            .and_then(|nic| nic.get("raw"))
            .and_then(|r| r.as_i64()),
        total_insider_shares: purchase_data
            .get("totalInsiderShares")
            .and_then(|tis| tis.get("raw"))
            .and_then(|r| r.as_i64()),
        net_percent_insider_shares: purchase_data
            .get("netPercentInsiderShares")
            .and_then(|npis| npis.get("raw"))
            .and_then(|r| r.as_f64()),
        buy_percent_insider_shares: purchase_data
            .get("buyPercentInsiderShares")
            .and_then(|bpis| bpis.get("raw"))
            .and_then(|r| r.as_f64()),
        sell_percent_insider_shares: purchase_data
            .get("sellPercentInsiderShares")
            .and_then(|spis| spis.get("raw"))
            .and_then(|r| r.as_f64()),
    })
}

fn parse_insider_roster(data: &Value) -> Result<Vec<InsiderRosterMember>, YahooError> {
    let empty_vec = Vec::new();
    let holders_list = data
        .get("insiderHolders")
        .and_then(|ih| ih.get("holders"))
        .and_then(|h| h.as_array())
        .unwrap_or(&empty_vec);

    let mut roster = Vec::new();

    for holder_data in holders_list {
        let latest_trans_date = holder_data
            .get("latestTransDate")
            .and_then(|ltd| ltd.get("raw"))
            .and_then(|r| r.as_i64())
            .and_then(|ts| DateTime::from_timestamp(ts, 0));

        let position_direct_date = holder_data
            .get("positionDirectDate")
            .and_then(|pdd| pdd.get("raw"))
            .and_then(|r| r.as_i64())
            .and_then(|ts| DateTime::from_timestamp(ts, 0));

        let member = InsiderRosterMember {
            name: holder_data
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string(),
            position: holder_data
                .get("relation")
                .and_then(|r| r.as_str())
                .unwrap_or("")
                .to_string(),
            most_recent_transaction: holder_data
                .get("transactionDescription")
                .and_then(|td| td.as_str())
                .map(|s| s.to_string()),
            latest_transaction_date: latest_trans_date,
            shares_owned_directly: holder_data
                .get("positionDirect")
                .and_then(|pd| pd.get("raw"))
                .and_then(|r| r.as_i64()),
            shares_owned_indirectly: holder_data
                .get("positionIndirect")
                .and_then(|pi| pi.get("raw"))
                .and_then(|r| r.as_i64()),
            position_direct_date,
        };

        roster.push(member);
    }

    Ok(roster)
}
