use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementType {
    #[serde(rename = "income")]
    IncomeStatement,
    #[serde(rename = "balance")]
    BalanceSheet,
    #[serde(rename = "cashflow")]
    CashFlow,
}

impl StatementType {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            StatementType::IncomeStatement => "income",
            StatementType::BalanceSheet => "balance",
            StatementType::CashFlow => "cashflow",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    #[serde(rename = "annual")]
    Annual,
    #[serde(rename = "quarterly")]
    Quarterly,
}

impl Frequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Frequency::Annual => "annual",
            Frequency::Quarterly => "quarterly",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FinancialStatement {
    pub symbol: String,
    pub statement_type: String,
    pub frequency: String,
    #[serde(rename = "statement")]
    pub statement: HashMap<String, HashMap<String, serde_json::Value>>,
}