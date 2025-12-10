use async_graphql::*;
use chrono::{DateTime, Utc};
use finance_query_core::models::holders::{
    InsiderPurchase as InsiderPurchaseModel,
    InsiderPurchasesResponse as InsiderPurchasesResponseModel,
    InsiderRosterMember as InsiderRosterMemberModel,
    InsiderRosterResponse as InsiderRosterResponseModel,
    InsiderTransaction as InsiderTransactionModel,
    InsiderTransactionsResponse as InsiderTransactionsResponseModel,
    InstitutionalHolder as InstitutionalHolderModel,
    InstitutionalHoldersResponse as InstitutionalHoldersResponseModel,
    MajorHoldersBreakdown as MajorHoldersBreakdownModel,
    MajorHoldersResponse as MajorHoldersResponseModel, MutualFundHolder as MutualFundHolderModel,
    MutualFundHoldersResponse as MutualFundHoldersResponseModel,
};
use std::collections::HashMap;

#[derive(SimpleObject, Clone)]
pub struct MajorHoldersBreakdown {
    #[graphql(name = "breakdownData")]
    pub breakdown_data: HashMap<String, async_graphql::Json<serde_json::Value>>,
}

impl From<MajorHoldersBreakdownModel> for MajorHoldersBreakdown {
    fn from(breakdown: MajorHoldersBreakdownModel) -> Self {
        MajorHoldersBreakdown {
            breakdown_data: breakdown
                .breakdown_data
                .into_iter()
                .map(|(k, v)| (k, async_graphql::Json(v)))
                .collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InstitutionalHolder {
    pub holder: String,
    pub shares: i64,
    #[graphql(name = "dateReported")]
    pub date_reported: DateTime<Utc>,
    #[graphql(name = "percentOut")]
    pub percent_out: Option<f64>,
    pub value: Option<i64>,
}

impl From<InstitutionalHolderModel> for InstitutionalHolder {
    fn from(holder: InstitutionalHolderModel) -> Self {
        InstitutionalHolder {
            holder: holder.holder,
            shares: holder.shares,
            date_reported: holder.date_reported,
            percent_out: holder.percent_out,
            value: holder.value,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct MutualFundHolder {
    pub holder: String,
    pub shares: i64,
    #[graphql(name = "dateReported")]
    pub date_reported: DateTime<Utc>,
    #[graphql(name = "percentOut")]
    pub percent_out: Option<f64>,
    pub value: Option<i64>,
}

impl From<MutualFundHolderModel> for MutualFundHolder {
    fn from(holder: MutualFundHolderModel) -> Self {
        MutualFundHolder {
            holder: holder.holder,
            shares: holder.shares,
            date_reported: holder.date_reported,
            percent_out: holder.percent_out,
            value: holder.value,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InsiderTransaction {
    #[graphql(name = "startDate")]
    pub start_date: DateTime<Utc>,
    pub insider: String,
    pub position: String,
    pub transaction: String,
    pub shares: Option<i64>,
    pub value: Option<i64>,
    pub ownership: Option<String>,
}

impl From<InsiderTransactionModel> for InsiderTransaction {
    fn from(transaction: InsiderTransactionModel) -> Self {
        InsiderTransaction {
            start_date: transaction.start_date,
            insider: transaction.insider,
            position: transaction.position,
            transaction: transaction.transaction,
            shares: transaction.shares,
            value: transaction.value,
            ownership: transaction.ownership,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InsiderPurchase {
    pub period: String,
    #[graphql(name = "purchasesShares")]
    pub purchases_shares: Option<i64>,
    #[graphql(name = "purchasesTransactions")]
    pub purchases_transactions: Option<i64>,
    #[graphql(name = "salesShares")]
    pub sales_shares: Option<i64>,
    #[graphql(name = "salesTransactions")]
    pub sales_transactions: Option<i64>,
    #[graphql(name = "netShares")]
    pub net_shares: Option<i64>,
    #[graphql(name = "netTransactions")]
    pub net_transactions: Option<i64>,
    #[graphql(name = "totalInsiderShares")]
    pub total_insider_shares: Option<i64>,
    #[graphql(name = "netPercentInsiderShares")]
    pub net_percent_insider_shares: Option<f64>,
    #[graphql(name = "buyPercentInsiderShares")]
    pub buy_percent_insider_shares: Option<f64>,
    #[graphql(name = "sellPercentInsiderShares")]
    pub sell_percent_insider_shares: Option<f64>,
}

impl From<InsiderPurchaseModel> for InsiderPurchase {
    fn from(purchase: InsiderPurchaseModel) -> Self {
        InsiderPurchase {
            period: purchase.period,
            purchases_shares: purchase.purchases_shares,
            purchases_transactions: purchase.purchases_transactions,
            sales_shares: purchase.sales_shares,
            sales_transactions: purchase.sales_transactions,
            net_shares: purchase.net_shares,
            net_transactions: purchase.net_transactions,
            total_insider_shares: purchase.total_insider_shares,
            net_percent_insider_shares: purchase.net_percent_insider_shares,
            buy_percent_insider_shares: purchase.buy_percent_insider_shares,
            sell_percent_insider_shares: purchase.sell_percent_insider_shares,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InsiderRosterMember {
    pub name: String,
    pub position: String,
    #[graphql(name = "mostRecentTransaction")]
    pub most_recent_transaction: Option<String>,
    #[graphql(name = "latestTransactionDate")]
    pub latest_transaction_date: Option<DateTime<Utc>>,
    #[graphql(name = "sharesOwnedDirectly")]
    pub shares_owned_directly: Option<i64>,
    #[graphql(name = "sharesOwnedIndirectly")]
    pub shares_owned_indirectly: Option<i64>,
    #[graphql(name = "positionDirectDate")]
    pub position_direct_date: Option<DateTime<Utc>>,
}

impl From<InsiderRosterMemberModel> for InsiderRosterMember {
    fn from(member: InsiderRosterMemberModel) -> Self {
        InsiderRosterMember {
            name: member.name,
            position: member.position,
            most_recent_transaction: member.most_recent_transaction,
            latest_transaction_date: member.latest_transaction_date,
            shares_owned_directly: member.shares_owned_directly,
            shares_owned_indirectly: member.shares_owned_indirectly,
            position_direct_date: member.position_direct_date,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct MajorHoldersResponse {
    pub symbol: String,
    pub breakdown: MajorHoldersBreakdown,
}

impl From<MajorHoldersResponseModel> for MajorHoldersResponse {
    fn from(response: MajorHoldersResponseModel) -> Self {
        MajorHoldersResponse {
            symbol: response.symbol,
            breakdown: MajorHoldersBreakdown::from(response.breakdown),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InstitutionalHoldersResponse {
    pub symbol: String,
    pub holders: Vec<InstitutionalHolder>,
}

impl From<InstitutionalHoldersResponseModel> for InstitutionalHoldersResponse {
    fn from(response: InstitutionalHoldersResponseModel) -> Self {
        InstitutionalHoldersResponse {
            symbol: response.symbol,
            holders: response
                .holders
                .into_iter()
                .map(InstitutionalHolder::from)
                .collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct MutualFundHoldersResponse {
    pub symbol: String,
    pub holders: Vec<MutualFundHolder>,
}

impl From<MutualFundHoldersResponseModel> for MutualFundHoldersResponse {
    fn from(response: MutualFundHoldersResponseModel) -> Self {
        MutualFundHoldersResponse {
            symbol: response.symbol,
            holders: response
                .holders
                .into_iter()
                .map(MutualFundHolder::from)
                .collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InsiderTransactionsResponse {
    pub symbol: String,
    pub transactions: Vec<InsiderTransaction>,
}

impl From<InsiderTransactionsResponseModel> for InsiderTransactionsResponse {
    fn from(response: InsiderTransactionsResponseModel) -> Self {
        InsiderTransactionsResponse {
            symbol: response.symbol,
            transactions: response
                .transactions
                .into_iter()
                .map(InsiderTransaction::from)
                .collect(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InsiderPurchasesResponse {
    pub symbol: String,
    pub summary: InsiderPurchase,
}

impl From<InsiderPurchasesResponseModel> for InsiderPurchasesResponse {
    fn from(response: InsiderPurchasesResponseModel) -> Self {
        InsiderPurchasesResponse {
            symbol: response.symbol,
            summary: InsiderPurchase::from(response.summary),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct InsiderRosterResponse {
    pub symbol: String,
    pub roster: Vec<InsiderRosterMember>,
}

impl From<InsiderRosterResponseModel> for InsiderRosterResponse {
    fn from(response: InsiderRosterResponseModel) -> Self {
        InsiderRosterResponse {
            symbol: response.symbol,
            roster: response
                .roster
                .into_iter()
                .map(InsiderRosterMember::from)
                .collect(),
        }
    }
}
