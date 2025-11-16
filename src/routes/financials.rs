use actix_web::{web, HttpResponse, Result};
use crate::models::{StatementType, Frequency};
use crate::service;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FinancialsQuery {
    statement: String,
    frequency: String,
}

pub async fn get_financials_handler(
    path: web::Path<String>,
    query: web::Query<FinancialsQuery>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let symbol = path.into_inner();
    
    let statement_type = parse_statement_type(&query.statement)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid statement type"))?;
    let frequency = parse_frequency(&query.frequency)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid frequency"))?;

    let financials = service::get_financial_statement(
        &app_state.yahoo_client,
        &symbol,
        statement_type,
        frequency,
    )
    .await?;

    Ok(HttpResponse::Ok().json(financials))
}

fn parse_statement_type(s: &str) -> Result<StatementType, ()> {
    match s.to_lowercase().as_str() {
        "income" | "income_statement" => Ok(StatementType::IncomeStatement),
        "balance" | "balance_sheet" => Ok(StatementType::BalanceSheet),
        "cashflow" | "cash_flow" => Ok(StatementType::CashFlow),
        _ => Err(()),
    }
}

fn parse_frequency(s: &str) -> Result<Frequency, ()> {
    match s.to_lowercase().as_str() {
        "annual" | "yearly" => Ok(Frequency::Annual),
        "quarterly" | "quarter" => Ok(Frequency::Quarterly),
        _ => Err(()),
    }
}

