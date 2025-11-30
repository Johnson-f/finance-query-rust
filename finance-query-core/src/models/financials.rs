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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn statement_type_roundtrip(st in prop_oneof![
            Just(StatementType::IncomeStatement),
            Just(StatementType::BalanceSheet),
            Just(StatementType::CashFlow),
        ]) {
            let json = serde_json::to_string(&st).unwrap();
            let parsed: StatementType = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(st.as_str(), parsed.as_str());
        }

        #[test]
        fn frequency_roundtrip(freq in prop_oneof![
            Just(Frequency::Annual),
            Just(Frequency::Quarterly),
        ]) {
            let json = serde_json::to_string(&freq).unwrap();
            let parsed: Frequency = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(freq.as_str(), parsed.as_str());
        }

        #[test]
        fn financial_statement_roundtrip(
            symbol in "[A-Z]{1,5}",
            statement_type in "income|balance|cashflow",
            frequency in "annual|quarterly",
        ) {
            let statement = FinancialStatement {
                symbol: symbol.clone(),
                statement_type: statement_type.clone(),
                frequency: frequency.clone(),
                statement: HashMap::new(),
            };

            let json = serde_json::to_string(&statement).unwrap();
            let parsed: FinancialStatement = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(statement.symbol, parsed.symbol);
            prop_assert_eq!(statement.statement_type, parsed.statement_type);
            prop_assert_eq!(statement.frequency, parsed.frequency);
        }
    }
}
