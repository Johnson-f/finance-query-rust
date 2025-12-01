# Financial Statements API

The Financial Statements API provides access to company financial data including income statements, balance sheets, and cash flow statements in both annual and quarterly formats.

## Overview

This module provides access to three core financial statements:

- **Income Statement**: Revenue, expenses, and profitability metrics
- **Balance Sheet**: Assets, liabilities, and shareholders' equity
- **Cash Flow Statement**: Operating, investing, and financing cash flows

Each statement is available in:
- **Annual**: Yearly financial data
- **Quarterly**: Quarterly financial data

## Data Structures

### FinancialStatement

Main structure containing financial statement data.

```rust
pub struct FinancialStatement {
    pub symbol: String,
    pub statement_type: String,
    pub frequency: String,
    pub statement: HashMap<String, HashMap<String, serde_json::Value>>,
}
```

**Fields:**
- `symbol`: Stock ticker symbol
- `statement_type`: Type of statement ("income", "balance", "cashflow")
- `frequency`: Data frequency ("annual" or "quarterly")
- `statement`: Nested map of financial line items by period

### StatementType

Enum for financial statement types.

```rust
pub enum StatementType {
    IncomeStatement,  // "income"
    BalanceSheet,     // "balance"
    CashFlow,         // "cashflow"
}
```

### Frequency

Enum for reporting frequency.

```rust
pub enum Frequency {
    Annual,     // "annual"
    Quarterly,  // "quarterly"
}
```


## Usage Examples

### Get Annual Income Statement

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let income = client.get_income_statement("AAPL", "annual").await?;
    
    println!("Income Statement for {}", income.symbol);
    println!("Type: {}", income.statement_type);
    println!("Frequency: {}", income.frequency);
    println!("Periods available: {}", income.statement.len());
    
    // List all available periods
    for period in income.statement.keys() {
        println!("  - {}", period);
    }
    
    Ok(())
}
```

### Get Quarterly Balance Sheet

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let balance = client.get_balance_sheet("MSFT", "quarterly").await?;
    
    println!("Balance Sheet for {}", balance.symbol);
    println!("Frequency: {}\n", balance.frequency);
    
    // Get most recent quarter
    if let Some((period, data)) = balance.statement.iter().next() {
        println!("Most Recent Period: {}", period);
        println!("Line items: {}", data.len());
        
        // Display some key metrics
        if let Some(total_assets) = data.get("TotalAssets") {
            println!("Total Assets: {:?}", total_assets);
        }
        
        if let Some(total_liabilities) = data.get("TotalLiabilities") {
            println!("Total Liabilities: {:?}", total_liabilities);
        }
        
        if let Some(equity) = data.get("StockholdersEquity") {
            println!("Stockholders' Equity: {:?}", equity);
        }
    }
    
    Ok(())
}
```

### Get Cash Flow Statement

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let cashflow = client.get_cash_flow("GOOGL", "annual").await?;
    
    println!("Cash Flow Statement for {}\n", cashflow.symbol);
    
    for (period, data) in &cashflow.statement {
        println!("Period: {}", period);
        
        if let Some(operating_cf) = data.get("OperatingCashFlow") {
            println!("  Operating Cash Flow: {:?}", operating_cf);
        }
        
        if let Some(investing_cf) = data.get("InvestingCashFlow") {
            println!("  Investing Cash Flow: {:?}", investing_cf);
        }
        
        if let Some(financing_cf) = data.get("FinancingCashFlow") {
            println!("  Financing Cash Flow: {:?}", financing_cf);
        }
        
        if let Some(free_cf) = data.get("FreeCashFlow") {
            println!("  Free Cash Flow: {:?}", free_cf);
        }
        
        println!();
    }
    
    Ok(())
}
```

### Extract Specific Metric Over Time

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let income = client.get_income_statement("NVDA", "annual").await?;
    
    println!("Revenue Trend for {}:\n", income.symbol);
    
    // Collect revenue data
    let mut revenues: Vec<(String, f64)> = Vec::new();
    
    for (period, data) in &income.statement {
        if let Some(revenue) = data.get("TotalRevenue") {
            if let Some(value) = revenue.as_f64() {
                revenues.push((period.clone(), value));
            }
        }
    }
    
    // Sort by period (assuming YYYY-MM-DD format)
    revenues.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Display trend
    for (i, (period, revenue)) in revenues.iter().enumerate() {
        print!("{}: ${:.2}B", period, revenue / 1_000_000_000.0);
        
        // Calculate growth rate
        if i > 0 {
            let prev_revenue = revenues[i - 1].1;
            let growth = ((revenue - prev_revenue) / prev_revenue) * 100.0;
            print!(" ({:+.1}% YoY)", growth);
        }
        
        println!();
    }
    
    Ok(())
}
```

### Calculate Financial Ratios

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let income = client.get_income_statement("AAPL", "annual").await?;
    let balance = client.get_balance_sheet("AAPL", "annual").await?;
    
    println!("Financial Ratios for {}:\n", income.symbol);
    
    // Get most recent period (assuming both have same periods)
    if let Some((period, income_data)) = income.statement.iter().next() {
        if let Some(balance_data) = balance.statement.get(period) {
            println!("Period: {}\n", period);
            
            // Profit Margin
            if let (Some(net_income), Some(revenue)) = (
                income_data.get("NetIncome").and_then(|v| v.as_f64()),
                income_data.get("TotalRevenue").and_then(|v| v.as_f64())
            ) {
                let profit_margin = (net_income / revenue) * 100.0;
                println!("Profit Margin: {:.2}%", profit_margin);
            }
            
            // Return on Assets (ROA)
            if let (Some(net_income), Some(total_assets)) = (
                income_data.get("NetIncome").and_then(|v| v.as_f64()),
                balance_data.get("TotalAssets").and_then(|v| v.as_f64())
            ) {
                let roa = (net_income / total_assets) * 100.0;
                println!("Return on Assets: {:.2}%", roa);
            }
            
            // Return on Equity (ROE)
            if let (Some(net_income), Some(equity)) = (
                income_data.get("NetIncome").and_then(|v| v.as_f64()),
                balance_data.get("StockholdersEquity").and_then(|v| v.as_f64())
            ) {
                let roe = (net_income / equity) * 100.0;
                println!("Return on Equity: {:.2}%", roe);
            }
            
            // Debt to Equity Ratio
            if let (Some(total_debt), Some(equity)) = (
                balance_data.get("TotalDebt").and_then(|v| v.as_f64()),
                balance_data.get("StockholdersEquity").and_then(|v| v.as_f64())
            ) {
                let debt_to_equity = total_debt / equity;
                println!("Debt to Equity: {:.2}", debt_to_equity);
            }
            
            // Current Ratio
            if let (Some(current_assets), Some(current_liabilities)) = (
                balance_data.get("CurrentAssets").and_then(|v| v.as_f64()),
                balance_data.get("CurrentLiabilities").and_then(|v| v.as_f64())
            ) {
                let current_ratio = current_assets / current_liabilities;
                println!("Current Ratio: {:.2}", current_ratio);
            }
        }
    }
    
    Ok(())
}
```

### Compare Multiple Companies

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "MSFT", "GOOGL"];
    
    println!("Revenue Comparison (Most Recent Annual):\n");
    
    for symbol in symbols {
        match client.get_income_statement(symbol, "annual").await {
            Ok(income) => {
                if let Some((period, data)) = income.statement.iter().next() {
                    if let Some(revenue) = data.get("TotalRevenue").and_then(|v| v.as_f64()) {
                        println!("{}: ${:.2}B ({})", 
                            symbol, 
                            revenue / 1_000_000_000.0,
                            period
                        );
                    }
                }
            }
            Err(e) => {
                println!("{}: Error - {}", symbol, e);
            }
        }
    }
    
    Ok(())
}
```

### Analyze Quarterly Trends

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let income = client.get_income_statement("TSLA", "quarterly").await?;
    
    println!("Quarterly Earnings Trend for {}:\n", income.symbol);
    
    // Collect EPS data
    let mut eps_data: Vec<(String, f64)> = Vec::new();
    
    for (period, data) in &income.statement {
        if let Some(eps) = data.get("BasicEPS").and_then(|v| v.as_f64()) {
            eps_data.push((period.clone(), eps));
        }
    }
    
    // Sort by period
    eps_data.sort_by(|a, b| a.0.cmp(&b.0));
    
    // Display with quarter-over-quarter change
    for (i, (period, eps)) in eps_data.iter().enumerate() {
        print!("{}: ${:.2}", period, eps);
        
        if i > 0 {
            let prev_eps = eps_data[i - 1].1;
            let change = eps - prev_eps;
            let change_pct = if prev_eps != 0.0 {
                (change / prev_eps.abs()) * 100.0
            } else {
                0.0
            };
            
            print!(" ({:+.2}, {:+.1}% QoQ)", change, change_pct);
        }
        
        println!();
    }
    
    Ok(())
}
```

### Generate Financial Summary

```rust
use finance_query_core::YahooClient;

async fn generate_financial_summary(
    symbol: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let income = client.get_income_statement(symbol, "annual").await?;
    let balance = client.get_balance_sheet(symbol, "annual").await?;
    let cashflow = client.get_cash_flow(symbol, "annual").await?;
    
    println!("═══════════════════════════════════════");
    println!("  Financial Summary: {}", symbol);
    println!("═══════════════════════════════════════\n");
    
    // Get most recent period
    if let Some((period, income_data)) = income.statement.iter().next() {
        println!("Period: {}\n", period);
        
        // Income Statement Highlights
        println!("📊 INCOME STATEMENT");
        if let Some(revenue) = income_data.get("TotalRevenue").and_then(|v| v.as_f64()) {
            println!("  Revenue: ${:.2}B", revenue / 1_000_000_000.0);
        }
        if let Some(gross_profit) = income_data.get("GrossProfit").and_then(|v| v.as_f64()) {
            println!("  Gross Profit: ${:.2}B", gross_profit / 1_000_000_000.0);
        }
        if let Some(operating_income) = income_data.get("OperatingIncome").and_then(|v| v.as_f64()) {
            println!("  Operating Income: ${:.2}B", operating_income / 1_000_000_000.0);
        }
        if let Some(net_income) = income_data.get("NetIncome").and_then(|v| v.as_f64()) {
            println!("  Net Income: ${:.2}B", net_income / 1_000_000_000.0);
        }
        
        // Balance Sheet Highlights
        if let Some(balance_data) = balance.statement.get(period) {
            println!("\n💰 BALANCE SHEET");
            if let Some(assets) = balance_data.get("TotalAssets").and_then(|v| v.as_f64()) {
                println!("  Total Assets: ${:.2}B", assets / 1_000_000_000.0);
            }
            if let Some(liabilities) = balance_data.get("TotalLiabilities").and_then(|v| v.as_f64()) {
                println!("  Total Liabilities: ${:.2}B", liabilities / 1_000_000_000.0);
            }
            if let Some(equity) = balance_data.get("StockholdersEquity").and_then(|v| v.as_f64()) {
                println!("  Stockholders' Equity: ${:.2}B", equity / 1_000_000_000.0);
            }
        }
        
        // Cash Flow Highlights
        if let Some(cf_data) = cashflow.statement.get(period) {
            println!("\n💵 CASH FLOW");
            if let Some(operating) = cf_data.get("OperatingCashFlow").and_then(|v| v.as_f64()) {
                println!("  Operating Cash Flow: ${:.2}B", operating / 1_000_000_000.0);
            }
            if let Some(capex) = cf_data.get("CapitalExpenditures").and_then(|v| v.as_f64()) {
                println!("  Capital Expenditures: ${:.2}B", capex / 1_000_000_000.0);
            }
            if let Some(free_cf) = cf_data.get("FreeCashFlow").and_then(|v| v.as_f64()) {
                println!("  Free Cash Flow: ${:.2}B", free_cf / 1_000_000_000.0);
            }
        }
    }
    
    println!("\n═══════════════════════════════════════\n");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_financial_summary("AAPL").await?;
    Ok(())
}
```

### Export to CSV

```rust
use finance_query_core::YahooClient;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let income = client.get_income_statement("AAPL", "annual").await?;
    
    let mut file = File::create("income_statement.csv")?;
    
    // Get all periods
    let mut periods: Vec<_> = income.statement.keys().collect();
    periods.sort();
    
    // Get all line items (from first period)
    let line_items: Vec<String> = if let Some(first_period) = periods.first() {
        if let Some(data) = income.statement.get(*first_period) {
            data.keys().cloned().collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    
    // Write header
    write!(file, "Line Item")?;
    for period in &periods {
        write!(file, ",{}", period)?;
    }
    writeln!(file)?;
    
    // Write data rows
    for item in &line_items {
        write!(file, "{}", item)?;
        
        for period in &periods {
            if let Some(data) = income.statement.get(*period) {
                if let Some(value) = data.get(item) {
                    write!(file, ",{}", value)?;
                } else {
                    write!(file, ",")?;
                }
            } else {
                write!(file, ",")?;
            }
        }
        
        writeln!(file)?;
    }
    
    println!("Exported to income_statement.csv");
    
    Ok(())
}
```


## JSON Response Formats

### Income Statement Response

```json
{
  "symbol": "AAPL",
  "statement_type": "income",
  "frequency": "annual",
  "statement": {
    "2023-09-30": {
      "TotalRevenue": 383285000000,
      "CostOfRevenue": 214137000000,
      "GrossProfit": 169148000000,
      "OperatingExpense": 55013000000,
      "OperatingIncome": 114135000000,
      "InterestExpense": 3933000000,
      "TaxProvision": 16741000000,
      "NetIncome": 96995000000,
      "BasicEPS": 6.16,
      "DilutedEPS": 6.13,
      "BasicAverageShares": 15744231000,
      "DilutedAverageShares": 15812547000
    },
    "2022-09-24": {
      "TotalRevenue": 394328000000,
      "CostOfRevenue": 223546000000,
      "GrossProfit": 170782000000,
      "OperatingExpense": 51345000000,
      "OperatingIncome": 119437000000,
      "InterestExpense": 2931000000,
      "TaxProvision": 19300000000,
      "NetIncome": 99803000000,
      "BasicEPS": 6.15,
      "DilutedEPS": 6.11,
      "BasicAverageShares": 16215963000,
      "DilutedAverageShares": 16325819000
    }
  }
}
```

### Balance Sheet Response

```json
{
  "symbol": "MSFT",
  "statement_type": "balance",
  "frequency": "quarterly",
  "statement": {
    "2024-09-30": {
      "TotalAssets": 512163000000,
      "CurrentAssets": 184257000000,
      "CashAndCashEquivalents": 78446000000,
      "AccountsReceivable": 56924000000,
      "Inventory": 1234000000,
      "TotalLiabilities": 238562000000,
      "CurrentLiabilities": 115485000000,
      "AccountsPayable": 23456000000,
      "ShortTermDebt": 12000000000,
      "LongTermDebt": 78925000000,
      "StockholdersEquity": 273601000000,
      "RetainedEarnings": 118848000000,
      "CommonStock": 93718000000
    },
    "2024-06-30": {
      "TotalAssets": 498366000000,
      "CurrentAssets": 179632000000,
      "CashAndCashEquivalents": 75531000000,
      "AccountsReceivable": 54321000000,
      "Inventory": 1198000000,
      "TotalLiabilities": 232706000000,
      "CurrentLiabilities": 112345000000,
      "AccountsPayable": 22134000000,
      "ShortTermDebt": 11500000000,
      "LongTermDebt": 76234000000,
      "StockholdersEquity": 265660000000,
      "RetainedEarnings": 115234000000,
      "CommonStock": 91567000000
    }
  }
}
```

### Cash Flow Statement Response

```json
{
  "symbol": "GOOGL",
  "statement_type": "cashflow",
  "frequency": "annual",
  "statement": {
    "2023-12-31": {
      "OperatingCashFlow": 101736000000,
      "InvestingCashFlow": -31485000000,
      "FinancingCashFlow": -58234000000,
      "FreeCashFlow": 69251000000,
      "CapitalExpenditures": -32485000000,
      "DividendsPaid": -0,
      "RepurchaseOfStock": -61521000000,
      "ChangeInCash": 12017000000,
      "BeginningCashPosition": 24048000000,
      "EndCashPosition": 30236000000
    },
    "2022-12-31": {
      "OperatingCashFlow": 91495000000,
      "InvestingCashFlow": -28745000000,
      "FinancingCashFlow": -52134000000,
      "FreeCashFlow": 60010000000,
      "CapitalExpenditures": -31485000000,
      "DividendsPaid": -0,
      "RepurchaseOfStock": -59296000000,
      "ChangeInCash": 10616000000,
      "BeginningCashPosition": 20945000000,
      "EndCashPosition": 24048000000
    }
  }
}
```

### Empty Statement

```json
{
  "symbol": "NEWCO",
  "statement_type": "income",
  "frequency": "annual",
  "statement": {}
}
```

## Common Line Items

### Income Statement

**Revenue:**
- `TotalRevenue` - Total revenue/sales
- `CostOfRevenue` - Cost of goods sold (COGS)
- `GrossProfit` - Revenue minus COGS

**Operating:**
- `OperatingExpense` - Total operating expenses
- `ResearchAndDevelopment` - R&D expenses
- `SellingGeneralAndAdministrative` - SG&A expenses
- `OperatingIncome` - Operating profit (EBIT)

**Non-Operating:**
- `InterestIncome` - Interest earned
- `InterestExpense` - Interest paid
- `OtherIncomeExpense` - Other gains/losses

**Bottom Line:**
- `PretaxIncome` - Income before taxes
- `TaxProvision` - Income tax expense
- `NetIncome` - Net profit/loss
- `NetIncomeCommonStockholders` - Net income available to common shareholders

**Per Share:**
- `BasicEPS` - Basic earnings per share
- `DilutedEPS` - Diluted earnings per share
- `BasicAverageShares` - Weighted average shares (basic)
- `DilutedAverageShares` - Weighted average shares (diluted)

### Balance Sheet

**Assets:**
- `TotalAssets` - Total assets
- `CurrentAssets` - Current assets (< 1 year)
- `CashAndCashEquivalents` - Cash and equivalents
- `ShortTermInvestments` - Marketable securities
- `AccountsReceivable` - Money owed by customers
- `Inventory` - Inventory value
- `PrepaidAssets` - Prepaid expenses
- `OtherCurrentAssets` - Other current assets

**Non-Current Assets:**
- `PropertyPlantEquipment` - PP&E (net)
- `GoodwillAndIntangibleAssets` - Goodwill and intangibles
- `LongTermInvestments` - Long-term investments
- `OtherNonCurrentAssets` - Other non-current assets

**Liabilities:**
- `TotalLiabilities` - Total liabilities
- `CurrentLiabilities` - Current liabilities (< 1 year)
- `AccountsPayable` - Money owed to suppliers
- `ShortTermDebt` - Short-term debt
- `CurrentDebtAndCapitalLeaseObligation` - Current debt obligations
- `OtherCurrentLiabilities` - Other current liabilities

**Non-Current Liabilities:**
- `LongTermDebt` - Long-term debt
- `LongTermDebtAndCapitalLeaseObligation` - Long-term debt obligations
- `DeferredTaxLiabilities` - Deferred taxes
- `OtherNonCurrentLiabilities` - Other non-current liabilities

**Equity:**
- `StockholdersEquity` - Total shareholders' equity
- `CommonStock` - Common stock value
- `RetainedEarnings` - Accumulated earnings
- `TreasuryStock` - Treasury stock (negative)
- `AccumulatedOtherComprehensiveIncome` - AOCI

### Cash Flow Statement

**Operating Activities:**
- `OperatingCashFlow` - Cash from operations
- `NetIncome` - Starting net income
- `DepreciationAndAmortization` - D&A add-back
- `DeferredIncomeTax` - Deferred tax changes
- `ChangeInWorkingCapital` - Working capital changes
- `ChangeInAccountsReceivable` - AR changes
- `ChangeInInventory` - Inventory changes
- `ChangeInAccountsPayable` - AP changes

**Investing Activities:**
- `InvestingCashFlow` - Cash from investing
- `CapitalExpenditures` - Capital expenditures (negative)
- `PurchaseOfInvestment` - Investment purchases
- `SaleOfInvestment` - Investment sales
- `PurchaseOfBusiness` - Acquisitions

**Financing Activities:**
- `FinancingCashFlow` - Cash from financing
- `RepurchaseOfStock` - Stock buybacks (negative)
- `DividendsPaid` - Dividends paid (negative)
- `IssuanceOfDebt` - Debt issued
- `RepaymentOfDebt` - Debt repaid (negative)
- `IssuanceOfStock` - Stock issued

**Summary:**
- `FreeCashFlow` - Operating CF minus CapEx
- `ChangeInCash` - Net change in cash
- `BeginningCashPosition` - Starting cash
- `EndCashPosition` - Ending cash

## Data Format Notes

### Values

- All monetary values are in the reporting currency (typically USD for US companies)
- Values are absolute numbers, not in thousands or millions
- Example: `383285000000` = $383.285 billion
- Negative values may be represented as negative numbers
- Some items (like expenses) may be positive even though they reduce income

### Periods

- Period keys are typically in `YYYY-MM-DD` format
- Represents the end date of the fiscal period
- Annual statements: Fiscal year end date
- Quarterly statements: Quarter end date
- Periods are sorted in reverse chronological order (most recent first)

### Missing Data

- Not all line items are available for all companies
- Some fields may be `null` or missing
- Different industries may have different line items
- Historical data availability varies by company

## Financial Ratios Reference

### Profitability Ratios

```rust
// Gross Margin
let gross_margin = (gross_profit / revenue) * 100.0;

// Operating Margin
let operating_margin = (operating_income / revenue) * 100.0;

// Net Profit Margin
let net_margin = (net_income / revenue) * 100.0;

// Return on Assets (ROA)
let roa = (net_income / total_assets) * 100.0;

// Return on Equity (ROE)
let roe = (net_income / stockholders_equity) * 100.0;
```

### Liquidity Ratios

```rust
// Current Ratio
let current_ratio = current_assets / current_liabilities;

// Quick Ratio (Acid Test)
let quick_ratio = (current_assets - inventory) / current_liabilities;

// Cash Ratio
let cash_ratio = cash_and_equivalents / current_liabilities;
```

### Leverage Ratios

```rust
// Debt to Equity
let debt_to_equity = total_debt / stockholders_equity;

// Debt to Assets
let debt_to_assets = total_debt / total_assets;

// Equity Multiplier
let equity_multiplier = total_assets / stockholders_equity;

// Interest Coverage
let interest_coverage = operating_income / interest_expense;
```

### Efficiency Ratios

```rust
// Asset Turnover
let asset_turnover = revenue / total_assets;

// Inventory Turnover
let inventory_turnover = cost_of_revenue / inventory;

// Receivables Turnover
let receivables_turnover = revenue / accounts_receivable;
```

### Cash Flow Ratios

```rust
// Operating Cash Flow Ratio
let ocf_ratio = operating_cash_flow / current_liabilities;

// Free Cash Flow to Net Income
let fcf_to_ni = free_cash_flow / net_income;

// Cash Flow Margin
let cf_margin = (operating_cash_flow / revenue) * 100.0;
```

## Best Practices

1. **Handle Missing Data**: Always check for `None` values before calculations
2. **Validate Periods**: Ensure periods match when comparing across statements
3. **Currency Awareness**: Be aware of reporting currency (usually in metadata)
4. **Fiscal vs Calendar**: Companies may have different fiscal year ends
5. **Restatements**: Historical data may be restated; use most recent data
6. **Industry Context**: Compare ratios within the same industry
7. **Trend Analysis**: Look at multiple periods for meaningful insights
8. **Cross-Validation**: Verify key metrics across different statements
9. **Cache Data**: Financial statements don't change frequently
10. **Error Handling**: Gracefully handle missing or incomplete data

## Important Notes

### Data Source

- Data is sourced from Yahoo Finance
- Based on company SEC filings (10-K, 10-Q)
- May have slight delays from official filing dates
- Historical data subject to restatements

### Limitations

- Line item names may vary between companies
- Not all companies report all line items
- Some metrics may be calculated differently
- International companies may report in local currency
- GAAP vs IFRS differences may exist

### Update Frequency

- Annual statements: Updated after fiscal year end
- Quarterly statements: Updated after each quarter
- Typically available 4-6 weeks after period end
- Check calendar API for earnings dates

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_income_statement("AAPL", "annual").await {
        Ok(statement) => {
            if statement.statement.is_empty() {
                println!("No financial data available");
            } else {
                println!("Loaded {} periods", statement.statement.len());
            }
        }
        Err(YahooError::NotFound) => {
            println!("Symbol not found");
        }
        Err(YahooError::ParseError(msg)) => {
            println!("Failed to parse financial data: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## Related APIs

- **Quote API**: Get current market price for valuation ratios
- **Analyst API**: Get earnings estimates and forecasts
- **Calendar API**: Get upcoming earnings dates
- **Actions API**: Get dividend payment history

## Performance Tips

- Financial statements can be large (100KB+)
- Cache statements locally to reduce API calls
- Process data asynchronously when fetching multiple symbols
- Consider using quarterly data for more granular analysis
- Filter to specific line items if you don't need all data

