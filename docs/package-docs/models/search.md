# Search Model

The Search model provides symbol lookup and discovery functionality, allowing you to find stocks, ETFs, indices, and other securities by name or symbol.

## Overview

The search module contains two main structures:
- `SearchResult` - Represents a single search result with symbol and metadata
- `SearchResponse` - Container for multiple search results

These models enable symbol discovery, autocomplete functionality, and security lookup.

## Data Structures

### SearchResult

Represents a single security found in the search.

```rust
pub struct SearchResult {
    pub symbol: String,
    pub name: String,
    pub exchange: Option<String>,
    pub quote_type: Option<String>,
}
```

**Fields:**
- `symbol` - Stock ticker symbol (e.g., "AAPL", "MSFT")
- `name` - Full name of the security (e.g., "Apple Inc.")
- `exchange` - Exchange where the security trades (e.g., "NASDAQ", "NYSE")
- `quote_type` - Type of security (e.g., "EQUITY", "ETF", "INDEX", "CRYPTOCURRENCY")

### SearchResponse

Container for search results.

```rust
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}
```

**Fields:**
- `results` - List of matching securities

## JSON Format

### Single SearchResult Example

```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "exchange": "NASDAQ",
  "quote_type": "EQUITY"
}
```

### SearchResponse Example

```json
{
  "results": [
    {
      "symbol": "AAPL",
      "name": "Apple Inc.",
      "exchange": "NASDAQ",
      "quote_type": "EQUITY"
    },
    {
      "symbol": "AAPL.MX",
      "name": "Apple Inc.",
      "exchange": "MEX",
      "quote_type": "EQUITY"
    },
    {
      "symbol": "APC.F",
      "name": "Apple Inc.",
      "exchange": "FRA",
      "quote_type": "EQUITY"
    }
  ]
}
```

### Multiple Security Types Example

```json
{
  "results": [
    {
      "symbol": "TSLA",
      "name": "Tesla, Inc.",
      "exchange": "NASDAQ",
      "quote_type": "EQUITY"
    },
    {
      "symbol": "TSLA240119C00250000",
      "name": "TSLA Jan 2024 250.000 call",
      "exchange": "OPR",
      "quote_type": "OPTION"
    },
    {
      "symbol": "TSLL",
      "name": "Direxion Daily TSLA Bull 2X Shares",
      "exchange": "NASDAQ",
      "quote_type": "ETF"
    }
  ]
}
```

## Quote Types

Common `quote_type` values you may encounter:

- `EQUITY` - Common stocks
- `ETF` - Exchange-traded funds
- `INDEX` - Market indices
- `MUTUALFUND` - Mutual funds
- `CRYPTOCURRENCY` - Digital currencies
- `CURRENCY` - Foreign exchange pairs
- `FUTURE` - Futures contracts
- `OPTION` - Options contracts

## Usage Examples

### Basic Symbol Search

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let results = client.search("Apple", 10).await?;
    
    println!("Search results for 'Apple':");
    for result in results.results {
        println!("{} - {} ({})",
            result.symbol,
            result.name,
            result.exchange.unwrap_or_else(|| "N/A".to_string())
        );
    }
    
    Ok(())
}
```

### Finding the Correct Symbol

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let query = "Tesla";
    let results = client.search(query, 5).await?;
    
    // Find the main equity listing
    let main_symbol = results.results
        .iter()
        .find(|r| {
            r.quote_type.as_deref() == Some("EQUITY") &&
            r.exchange.as_deref() == Some("NASDAQ")
        });
    
    if let Some(result) = main_symbol {
        println!("Found: {} ({})", result.symbol, result.name);
        
        // Now fetch the quote
        let quote = client.get_quote(&result.symbol).await?;
        println!("Price: ${}", quote.price);
    } else {
        println!("No matching equity found");
    }
    
    Ok(())
}
```

### Building an Autocomplete Feature

```rust
use finance_query_core::YahooClient;

async fn autocomplete_symbols(query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let results = client.search(query, 10).await?;
    
    // Return only equity symbols
    let symbols: Vec<String> = results.results
        .into_iter()
        .filter(|r| r.quote_type.as_deref() == Some("EQUITY"))
        .map(|r| format!("{} - {}", r.symbol, r.name))
        .collect();
    
    Ok(symbols)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let suggestions = autocomplete_symbols("micro").await?;
    
    println!("Autocomplete suggestions:");
    for suggestion in suggestions {
        println!("  {}", suggestion);
    }
    
    Ok(())
}
```

### Filtering by Security Type

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let results = client.search("S&P", 20).await?;
    
    // Separate by type
    let mut equities = Vec::new();
    let mut etfs = Vec::new();
    let mut indices = Vec::new();
    
    for result in results.results {
        match result.quote_type.as_deref() {
            Some("EQUITY") => equities.push(result),
            Some("ETF") => etfs.push(result),
            Some("INDEX") => indices.push(result),
            _ => {}
        }
    }
    
    println!("Equities: {}", equities.len());
    println!("ETFs: {}", etfs.len());
    println!("Indices: {}", indices.len());
    
    println!("\nS&P 500 ETFs:");
    for etf in etfs {
        println!("  {} - {}", etf.symbol, etf.name);
    }
    
    Ok(())
}
```

### Finding International Listings

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let results = client.search("Apple", 20).await?;
    
    println!("Apple Inc. International Listings:");
    println!("{:<15} {:<30} {:<10}", "Symbol", "Name", "Exchange");
    println!("{}", "-".repeat(60));
    
    for result in results.results {
        if result.quote_type.as_deref() == Some("EQUITY") {
            println!("{:<15} {:<30} {:<10}",
                result.symbol,
                result.name,
                result.exchange.unwrap_or_else(|| "N/A".to_string())
            );
        }
    }
    
    Ok(())
}
```

### ETF Discovery

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let keywords = vec!["technology", "dividend", "growth", "value"];
    
    for keyword in keywords {
        let results = client.search(keyword, 5).await?;
        
        let etfs: Vec<_> = results.results
            .into_iter()
            .filter(|r| r.quote_type.as_deref() == Some("ETF"))
            .collect();
        
        if !etfs.is_empty() {
            println!("\n{} ETFs:", keyword.to_uppercase());
            for etf in etfs {
                println!("  {} - {}", etf.symbol, etf.name);
            }
        }
    }
    
    Ok(())
}
```

### Cryptocurrency Search

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let results = client.search("Bitcoin", 10).await?;
    
    println!("Bitcoin-related securities:");
    
    for result in results.results {
        let type_str = result.quote_type.as_deref().unwrap_or("UNKNOWN");
        println!("{} - {} [{}]", result.symbol, result.name, type_str);
    }
    
    Ok(())
}
```

### Symbol Validation

```rust
use finance_query_core::YahooClient;

async fn validate_symbol(symbol: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let results = client.search(symbol, 10).await?;
    
    // Check if exact match exists
    let exact_match = results.results
        .iter()
        .any(|r| r.symbol.eq_ignore_ascii_case(symbol));
    
    Ok(exact_match)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbols = vec!["AAPL", "INVALID123", "MSFT", "FAKE"];
    
    for symbol in symbols {
        let is_valid = validate_symbol(symbol).await?;
        println!("{}: {}", symbol, if is_valid { "✓ Valid" } else { "✗ Invalid" });
    }
    
    Ok(())
}
```

### Building a Symbol Lookup Tool

```rust
use finance_query_core::YahooClient;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    loop {
        print!("Enter search query (or 'quit' to exit): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let query = input.trim();
        
        if query.eq_ignore_ascii_case("quit") {
            break;
        }
        
        if query.is_empty() {
            continue;
        }
        
        let results = client.search(query, 10).await?;
        
        if results.results.is_empty() {
            println!("No results found.\n");
            continue;
        }
        
        println!("\nResults:");
        for (i, result) in results.results.iter().enumerate() {
            println!("{}. {} - {} [{}]",
                i + 1,
                result.symbol,
                result.name,
                result.quote_type.as_deref().unwrap_or("N/A")
            );
        }
        println!();
    }
    
    Ok(())
}
```

### Sector-Specific Search

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Search for sector ETFs
    let sectors = vec![
        "technology",
        "healthcare",
        "financial",
        "energy",
        "consumer"
    ];
    
    println!("Sector ETF Finder:\n");
    
    for sector in sectors {
        let query = format!("{} sector", sector);
        let results = client.search(&query, 5).await?;
        
        let sector_etfs: Vec<_> = results.results
            .into_iter()
            .filter(|r| {
                r.quote_type.as_deref() == Some("ETF") &&
                r.name.to_lowercase().contains("sector")
            })
            .collect();
        
        if !sector_etfs.is_empty() {
            println!("{}:", sector.to_uppercase());
            for etf in sector_etfs {
                println!("  {} - {}", etf.symbol, etf.name);
            }
            println!();
        }
    }
    
    Ok(())
}
```

### Comparing Similar Companies

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Search for companies in the same industry
    let results = client.search("semiconductor", 15).await?;
    
    let equities: Vec<_> = results.results
        .into_iter()
        .filter(|r| r.quote_type.as_deref() == Some("EQUITY"))
        .collect();
    
    println!("Semiconductor Companies:");
    println!("{:<10} {:<40} {:<15}", "Symbol", "Name", "Exchange");
    println!("{}", "-".repeat(70));
    
    for equity in &equities {
        println!("{:<10} {:<40} {:<15}",
            equity.symbol,
            equity.name,
            equity.exchange.as_deref().unwrap_or("N/A")
        );
    }
    
    // Fetch quotes for comparison
    println!("\nPrice Comparison:");
    for equity in equities.iter().take(5) {
        if let Ok(quote) = client.get_quote(&equity.symbol).await {
            println!("{}: ${} ({})",
                equity.symbol,
                quote.price,
                quote.percent_change
            );
        }
    }
    
    Ok(())
}
```

### Exporting Search Results

```rust
use finance_query_core::YahooClient;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let results = client.search("tech", 20).await?;
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write("search_results.json", json)?;
    
    // Export to CSV
    let mut csv = String::from("Symbol,Name,Exchange,Type\n");
    for result in &results.results {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            result.symbol,
            result.name.replace(",", ";"), // Escape commas
            result.exchange.as_deref().unwrap_or("N/A"),
            result.quote_type.as_deref().unwrap_or("N/A")
        ));
    }
    std::fs::write("search_results.csv", csv)?;
    
    println!("Search results exported successfully");
    println!("Found {} results", results.results.len());
    
    Ok(())
}
```

## Search Tips

### Effective Search Queries

1. **Company Name**: "Apple", "Microsoft", "Tesla"
2. **Ticker Symbol**: "AAPL", "MSFT", "TSLA"
3. **Partial Match**: "micro" (finds Microsoft, Micron, etc.)
4. **Industry Terms**: "semiconductor", "biotech", "banking"
5. **Index Names**: "S&P 500", "Dow Jones", "NASDAQ"
6. **ETF Categories**: "dividend ETF", "tech ETF", "bond ETF"

### Best Practices

- Use at least 3 characters for meaningful results
- Limit results to 10-20 for better performance
- Filter by `quote_type` to narrow down results
- Check `exchange` to find primary listings
- Validate symbols before fetching detailed data

## Common Use Cases

1. **Symbol Lookup** - Find the correct ticker for a company
2. **Autocomplete** - Provide search suggestions in UI
3. **Symbol Validation** - Verify if a symbol exists
4. **Discovery** - Find securities in a specific sector or category
5. **International Listings** - Locate ADRs and foreign exchanges
6. **ETF Screening** - Discover funds matching criteria
7. **Competitor Analysis** - Find similar companies

## Notes

- Search is case-insensitive
- Results are ordered by relevance
- The same company may appear multiple times (different exchanges)
- Not all results will have `exchange` or `quote_type` populated
- Search may return related securities (options, ETFs tracking the stock)
- Results are limited by the `limit` parameter (typically 1-25)
- Some symbols may be delisted or inactive
- International symbols may have suffixes (e.g., ".L" for London, ".TO" for Toronto)

## Exchange Codes

Common exchange abbreviations:

- `NASDAQ` - NASDAQ Stock Market
- `NYSE` - New York Stock Exchange
- `AMEX` - American Stock Exchange
- `LSE` - London Stock Exchange
- `TSE` - Tokyo Stock Exchange
- `FRA` - Frankfurt Stock Exchange
- `MEX` - Mexican Stock Exchange
- `OPR` - Options Price Reporting Authority

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.search("Apple", 10).await {
        Ok(results) => {
            if results.results.is_empty() {
                println!("No results found");
            } else {
                println!("Found {} results", results.results.len());
                for result in results.results {
                    println!("{} - {}", result.symbol, result.name);
                }
            }
        }
        Err(YahooError::Network(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(YahooError::ParseError(msg)) => {
            eprintln!("Failed to parse results: {}", msg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## See Also

- [Quote Model](./quote.md) - For fetching detailed quote data after finding a symbol
- [News Model](./news.md) - For news about discovered securities
- [Historical Model](./historical.md) - For historical price data
