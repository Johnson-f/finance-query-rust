# Industry Model

The Industry model provides detailed information about specific industries, including their top performing and fastest growing companies.

## Overview

The `Industry` struct represents a financial industry with its associated companies and performance metrics. It includes information about the industry's sector classification, description, and lists of top performing and high-growth companies.

## Data Structures

### Industry

The main structure representing an industry.

```rust
pub struct Industry {
    pub key: String,
    pub name: String,
    pub sector_key: Option<String>,
    pub sector_name: Option<String>,
    pub description: Option<String>,
    pub top_performing_companies: Vec<IndustryCompany>,
    pub top_growth_companies: Vec<IndustryCompany>,
}
```

**Fields:**
- `key` - Unique identifier for the industry
- `name` - Display name of the industry
- `sector_key` - Optional unique identifier for the parent sector
- `sector_name` - Optional display name of the parent sector
- `description` - Optional detailed description of the industry
- `top_performing_companies` - List of companies with the best YTD returns
- `top_growth_companies` - List of companies with the highest growth estimates

### IndustryCompany

Represents a company within an industry context.

```rust
pub struct IndustryCompany {
    pub symbol: String,
    pub name: String,
    pub ytd_return: Option<f64>,
    pub last_price: Option<f64>,
    pub target_price: Option<f64>,
    pub growth_estimate: Option<f64>,
}
```

**Fields:**
- `symbol` - Stock ticker symbol
- `name` - Company name
- `ytd_return` - Year-to-date return percentage (present in top performing companies)
- `last_price` - Current stock price (present in top performing companies)
- `target_price` - Analyst target price (present in top performing companies)
- `growth_estimate` - Growth estimate percentage (present in top growth companies)

## JSON Format

### Example Response

```json
{
  "key": "ms_technology",
  "name": "Technology",
  "sectorKey": "ms_technology_sector",
  "sectorName": "Technology Sector",
  "description": "Companies engaged in the design, development, and support of computer operating systems and applications.",
  "topPerformingCompanies": [
    {
      "symbol": "AAPL",
      "name": "Apple Inc.",
      "ytdReturn": 45.23,
      "lastPrice": 178.50,
      "targetPrice": 195.00
    },
    {
      "symbol": "MSFT",
      "name": "Microsoft Corporation",
      "ytdReturn": 42.18,
      "lastPrice": 378.91,
      "targetPrice": 405.00
    },
    {
      "symbol": "NVDA",
      "name": "NVIDIA Corporation",
      "ytdReturn": 38.67,
      "lastPrice": 495.22,
      "targetPrice": 550.00
    }
  ],
  "topGrowthCompanies": [
    {
      "symbol": "PLTR",
      "name": "Palantir Technologies Inc.",
      "ytdReturn": 125.45,
      "growthEstimate": 28.5
    },
    {
      "symbol": "SNOW",
      "name": "Snowflake Inc.",
      "ytdReturn": 89.32,
      "growthEstimate": 25.3
    },
    {
      "symbol": "CRWD",
      "name": "CrowdStrike Holdings Inc.",
      "ytdReturn": 76.21,
      "growthEstimate": 22.8
    }
  ]
}
```

### Field Descriptions

**Top-Level Fields:**
- All string fields are required except those marked as `Option`
- `sectorKey` and `sectorName` may be `null` if the industry is not associated with a sector
- `description` may be `null` if no description is available
- Company arrays are always present but may be empty

**Company Fields:**
- Fields are conditionally included based on availability
- `ytdReturn` is typically present in both lists
- `lastPrice` and `targetPrice` are typically only in `topPerformingCompanies`
- `growthEstimate` is typically only in `topGrowthCompanies`
- Numeric values may be `null` if data is unavailable

## Usage Examples

### Basic Usage

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Fetch industry data
    let industry = client.get_industry("ms_technology").await?;
    
    println!("Industry: {}", industry.name);
    println!("Sector: {}", industry.sector_name.unwrap_or_default());
    println!("Description: {}", industry.description.unwrap_or_default());
    
    Ok(())
}
```

### Analyzing Top Performers

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let industry = client.get_industry("ms_software").await?;
    
    println!("Top Performing Companies in {}:", industry.name);
    println!("{:<10} {:<30} {:>12} {:>12}", "Symbol", "Name", "YTD Return", "Last Price");
    println!("{}", "-".repeat(70));
    
    for company in &industry.top_performing_companies {
        let ytd = company.ytd_return
            .map(|v| format!("{:.2}%", v))
            .unwrap_or_else(|| "N/A".to_string());
        let price = company.last_price
            .map(|v| format!("${:.2}", v))
            .unwrap_or_else(|| "N/A".to_string());
            
        println!("{:<10} {:<30} {:>12} {:>12}", 
            company.symbol, 
            company.name, 
            ytd, 
            price
        );
    }
    
    Ok(())
}
```

### Analyzing Growth Companies

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let industry = client.get_industry("ms_biotechnology").await?;
    
    println!("High Growth Companies in {}:", industry.name);
    println!("{:<10} {:<30} {:>15} {:>15}", "Symbol", "Name", "YTD Return", "Growth Est.");
    println!("{}", "-".repeat(75));
    
    for company in &industry.top_growth_companies {
        let ytd = company.ytd_return
            .map(|v| format!("{:.2}%", v))
            .unwrap_or_else(|| "N/A".to_string());
        let growth = company.growth_estimate
            .map(|v| format!("{:.2}%", v))
            .unwrap_or_else(|| "N/A".to_string());
            
        println!("{:<10} {:<30} {:>15} {:>15}", 
            company.symbol, 
            company.name, 
            ytd, 
            growth
        );
    }
    
    Ok(())
}
```

### Finding Investment Opportunities

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let industry = client.get_industry("ms_semiconductors").await?;
    
    // Find companies with high growth and positive YTD returns
    let opportunities: Vec<_> = industry.top_growth_companies
        .iter()
        .filter(|c| {
            c.ytd_return.unwrap_or(0.0) > 20.0 && 
            c.growth_estimate.unwrap_or(0.0) > 15.0
        })
        .collect();
    
    println!("Investment Opportunities in {}:", industry.name);
    for company in opportunities {
        println!("  {} ({})", company.name, company.symbol);
        println!("    YTD Return: {:.2}%", company.ytd_return.unwrap());
        println!("    Growth Estimate: {:.2}%", company.growth_estimate.unwrap());
    }
    
    Ok(())
}
```

### Comparing Price to Target

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let industry = client.get_industry("ms_banks").await?;
    
    println!("Price vs Target Analysis for {}:", industry.name);
    
    for company in &industry.top_performing_companies {
        if let (Some(price), Some(target)) = (company.last_price, company.target_price) {
            let upside = ((target - price) / price) * 100.0;
            
            println!("{} ({})", company.name, company.symbol);
            println!("  Current: ${:.2}", price);
            println!("  Target: ${:.2}", target);
            println!("  Upside: {:.2}%", upside);
            println!();
        }
    }
    
    Ok(())
}
```

### Serializing to JSON

```rust
use finance_query_core::YahooClient;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let industry = client.get_industry("ms_aerospace_defense").await?;
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&industry)?;
    println!("{}", json);
    
    // Save to file
    std::fs::write("industry_data.json", json)?;
    
    Ok(())
}
```

## Common Industry Keys

Here are some common industry keys you can use:

- `ms_technology` - Technology
- `ms_software` - Software
- `ms_semiconductors` - Semiconductors
- `ms_biotechnology` - Biotechnology
- `ms_pharmaceuticals` - Pharmaceuticals
- `ms_banks` - Banks
- `ms_insurance` - Insurance
- `ms_aerospace_defense` - Aerospace & Defense
- `ms_automotive` - Automotive
- `ms_retail` - Retail
- `ms_energy` - Energy
- `ms_utilities` - Utilities
- `ms_real_estate` - Real Estate
- `ms_telecommunications` - Telecommunications

## Notes

- Industry keys typically follow the format `ms_<industry_name>` where `ms` stands for "market screener"
- The number of companies in each list varies by industry and data availability
- YTD returns are expressed as percentages (e.g., 45.23 means 45.23%)
- Growth estimates represent expected annual growth rates
- Price data is in USD
- Data is updated regularly but may have slight delays
- Not all companies will have all fields populated

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_industry("invalid_industry_key").await {
        Ok(industry) => {
            println!("Industry: {}", industry.name);
        }
        Err(YahooError::NotFound) => {
            eprintln!("Industry not found");
        }
        Err(YahooError::Network(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## See Also

- [Sectors Model](./sectors.md) - For broader sector-level data
- [Market Model](./market.md) - For overall market trends
- [Quote Model](./quote.md) - For individual stock quotes
