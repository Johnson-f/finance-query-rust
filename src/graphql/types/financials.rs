use async_graphql::*;
use std::collections::HashMap;
use crate::models::financials::{
    FinancialStatement as FinancialStatementModel,
    StatementType as StatementTypeModel,
    Frequency as FrequencyModel,
};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum StatementType {
    #[graphql(name = "income")]
    IncomeStatement,
    #[graphql(name = "balance")]
    BalanceSheet,
    #[graphql(name = "cashflow")]
    CashFlow,
}

impl From<StatementTypeModel> for StatementType {
    fn from(stmt: StatementTypeModel) -> Self {
        match stmt {
            StatementTypeModel::IncomeStatement => StatementType::IncomeStatement,
            StatementTypeModel::BalanceSheet => StatementType::BalanceSheet,
            StatementTypeModel::CashFlow => StatementType::CashFlow,
        }
    }
}

impl From<StatementType> for StatementTypeModel {
    fn from(stmt: StatementType) -> Self {
        match stmt {
            StatementType::IncomeStatement => StatementTypeModel::IncomeStatement,
            StatementType::BalanceSheet => StatementTypeModel::BalanceSheet,
            StatementType::CashFlow => StatementTypeModel::CashFlow,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Frequency {
    #[graphql(name = "annual")]
    Annual,
    #[graphql(name = "quarterly")]
    Quarterly,
}

impl From<FrequencyModel> for Frequency {
    fn from(freq: FrequencyModel) -> Self {
        match freq {
            FrequencyModel::Annual => Frequency::Annual,
            FrequencyModel::Quarterly => Frequency::Quarterly,
        }
    }
}

impl From<Frequency> for FrequencyModel {
    fn from(freq: Frequency) -> Self {
        match freq {
            Frequency::Annual => FrequencyModel::Annual,
            Frequency::Quarterly => FrequencyModel::Quarterly,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct FinancialStatement {
    pub symbol: String,
    pub statement_type: String,
    pub frequency: String,
    pub statement: HashMap<String, HashMap<String, async_graphql::Json<serde_json::Value>>>,
}

impl From<FinancialStatementModel> for FinancialStatement {
    fn from(stmt: FinancialStatementModel) -> Self {
        FinancialStatement {
            symbol: stmt.symbol,
            statement_type: stmt.statement_type,
            frequency: stmt.frequency,
            statement: stmt.statement.into_iter()
                .map(|(k, v)| {
                    let inner_map: HashMap<String, async_graphql::Json<serde_json::Value>> = v
                        .into_iter()
                        .map(|(k2, v2)| (k2, async_graphql::Json(v2)))
                        .collect();
                    (k, inner_map)
                })
                .collect(),
        }
    }
}