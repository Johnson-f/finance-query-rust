# SEC Filings Model

The SEC Filings model provides access to regulatory filings submitted to the U.S. Securities and Exchange Commission (SEC), including 10-K, 10-Q, 8-K, and other important corporate disclosures.

## Overview

The SEC filings module contains three main structures:
- `SecFiling` - Represents a single SEC filing with metadata and exhibits
- `SecExhibit` - Represents an exhibit or attachment within a filing
- `SecFilingsResponse` - Container for all filings for a symbol

These models enable regulatory compliance tracking, fundamental research, and corporate event monitoring.

## Data Structures

### SecFiling

Represents a single SEC filing.

```rust
pub struct SecFiling {
    pub date: DateTime<Utc>,
    pub filing_type: String,
    pub title: String,
    pub url: String,
    pub exhibits: Vec<SecExhibit>,
}
```

**Fields:**
- `date` - Filing date as UTC timestamp
- `filing_type` - Type of filing (e.g., "10-K", "10-Q", "8-K", "DEF 14A")
- `title` - Full title of the filing
- `url` - Direct link to the filing on SEC EDGAR
- `exhibits` - List of exhibits/attachments included with the filing

### SecExhibit

Represents an exhibit within a filing.

```rust
pub struct SecExhibit {
    pub exhibit_type: String,
    pub url: String,
}
```

**Fields:**
- `exhibit_type` - Type of exhibit (e.g., "EX-31.1", "EX-32.1", "EX-99.1")
- `url` - Direct link to the exhibit document

### SecFilingsResponse

Container for all filings for a symbol.

```rust
pub struct SecFilingsResponse {
    pub symbol: String,
    pub filings: Vec<SecFiling>,
}
```

**Fields:**
- `symbol` - Stock ticker symbol
- `filings` - List of SEC filings, typically in reverse chronological order

## JSON Format

### SecFilingsResponse Example

```json
{
  "symbol": "AAPL",
  "filings": [
    {
      "date": "2024-11-01T00:00:00Z",
      "filingType": "10-K",
      "title": "Annual Report for fiscal year ending September 30, 2024",
      "url": "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000320193&type=10-K&dateb=&owner=exclude&count=40",
      "exhibits": [
        {
          "exhibitType": "EX-31.1",
          "url": "https://www.sec.gov/Archives/edgar/data/320193/000032019324000123/aapl-20240930ex311.htm"
        },
        {
          "exhibitType": "EX-31.2",
          "url": "https://www.sec.gov/Archives/edgar/data/320193/000032019324000123/aapl-20240930ex312.htm"
        },
        {
          "exhibitType": "EX-32.1",
          "url": "https://www.sec.gov/Archives/edgar/data/320193/000032019324000123/aapl-20240930ex321.htm"
        }
      ]
    },
    {
      "date": "2024-08-01T00:00:00Z",
      "filingType": "10-Q",
      "title": "Quarterly Report for quarter ending June 30, 2024",
      "url": "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000320193&type=10-Q&dateb=&owner=exclude&count=40",
      "exhibits": [
        {
          "exhibitType": "EX-31.1",
          "url": "https://www.sec.gov/Archives/edgar/data/320193/000032019324000089/aapl-20240630ex311.htm"
        },
        {
          "exhibitType": "EX-32.1",
          "url": "https://www.sec.gov/Archives/edgar/data/320193/000032019324000089/aapl-20240630ex321.htm"
        }
      ]
    },
    {
      "date": "2024-07-15T00:00:00Z",
      "filingType": "8-K",
      "title": "Current Report - Results of Operations and Financial Condition",
      "url": "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0000320193&type=8-K&dateb=&owner=exclude&count=40",
      "exhibits": [
        {
          "exhibitType": "EX-99.1",
          "url": "https://www.sec.gov/Archives/edgar/data/320193/000032019324000078/aapl-20240715ex991.htm"
        }
      ]
    }
  ]
}
```

### Filing Without Exhibits Example

```json
{
  "symbol": "TSLA",
  "filings": [
    {
      "date": "2024-10-15T00:00:00Z",
      "filingType": "4",
      "title": "Statement of Changes in Beneficial Ownership",
      "url": "https://www.sec.gov/cgi-bin/browse-edgar?action=getcompany&CIK=0001318605&type=4&dateb=&owner=exclude&count=40",
      "exhibits": []
    }
  ]
}
```

## Filing Types

### Common SEC Filing Types

**Periodic Reports:**
- `10-K` - Annual report with comprehensive financial information
- `10-Q` - Quarterly report with unaudited financial statements
- `8-K` - Current report for material events

**Proxy Statements:**
- `DEF 14A` - Definitive proxy statement (annual meeting materials)
- `DEFA14A` - Additional proxy soliciting materials

**Registration Statements:**
- `S-1` - Initial registration for new securities
- `S-3` - Simplified registration for seasoned issuers
- `S-4` - Registration for business combinations
- `S-8` - Registration for employee benefit plans

**Ownership Reports:**
- `3` - Initial statement of beneficial ownership
- `4` - Statement of changes in beneficial ownership
- `5` - Annual statement of beneficial ownership
- `13D` - Schedule filed when acquiring 5%+ of a company
- `13G` - Simplified version of 13D for passive investors

**Other Important Filings:**
- `SC 13D/G` - Beneficial ownership reports
- `144` - Notice of proposed sale of securities
- `6-K` - Report of foreign private issuer

## Exhibit Types

### Common Exhibit Types

**Certifications:**
- `EX-31.1` - Section 302 certification by CEO
- `EX-31.2` - Section 302 certification by CFO
- `EX-32.1` - Section 906 certification by CEO
- `EX-32.2` - Section 906 certification by CFO

**Financial Information:**
- `EX-99.1` - Additional exhibits (often earnings releases)
- `EX-99.2` - Additional financial information

**Contracts and Agreements:**
- `EX-10.1` - Material contracts
- `EX-4.1` - Instruments defining rights of security holders

## Usage Examples

### Fetching SEC Filings

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let filings = client.get_sec_filings("AAPL").await?;
    
    println!("SEC Filings for {}:", filings.symbol);
    println!();
    
    for filing in filings.filings.iter().take(10) {
        println!("{} - {} ({})",
            filing.date.format("%Y-%m-%d"),
            filing.filing_type,
            filing.title
        );
    }
    
    Ok(())
}
```

### Finding Specific Filing Types

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let filings = client.get_sec_filings("TSLA").await?;
    
    // Find all 10-K filings (annual reports)
    let annual_reports: Vec<_> = filings.filings
        .iter()
        .filter(|f| f.filing_type == "10-K")
        .collect();
    
    println!("Annual Reports (10-K) for {}:", filings.symbol);
    for report in annual_reports {
        println!("{} - {}", 
            report.date.format("%Y-%m-%d"),
            report.title
        );
        println!("  URL: {}", report.url);
        println!();
    }
    
    Ok(())
}
```

### Monitoring Recent 8-K Filings

```rust
use finance_query_core::YahooClient;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let filings = client.get_sec_filings("NVDA").await?;
    
    // Find 8-K filings from the last 30 days
    let thirty_days_ago = Utc::now() - Duration::days(30);
    
    let recent_8k: Vec<_> = filings.filings
        .iter()
        .filter(|f| {
            f.filing_type == "8-K" && f.date > thirty_days_ago
        })
        .collect();
    
    if recent_8k.is_empty() {
        println!("No recent 8-K filings in the last 30 days");
    } else {
        println!("Recent 8-K Filings (Material Events):");
        for filing in recent_8k {
            println!("\n{}", filing.date.format("%Y-%m-%d"));
            println!("{}", filing.title);
            println!("{}", filing.url);
        }
    }
    
    Ok(())
}
```

### Analyzing Filing Exhibits

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let filings = client.get_sec_filings("MSFT").await?;
    
    // Find the most recent 10-K
    let latest_10k = filings.filings
        .iter()
        .find(|f| f.filing_type == "10-K");
    
    if let Some(filing) = latest_10k {
        println!("Latest 10-K: {}", filing.date.format("%Y-%m-%d"));
        println!("Title: {}", filing.title);
        println!("\nExhibits:");
        
        for exhibit in &filing.exhibits {
            println!("  {} - {}", exhibit.exhibit_type, exhibit.url);
        }
        
        // Find CEO certification
        let ceo_cert = filing.exhibits
            .iter()
            .find(|e| e.exhibit_type == "EX-31.1");
        
        if let Some(cert) = ceo_cert {
            println!("\nCEO Certification: {}", cert.url);
        }
    }
    
    Ok(())
}
```

### Tracking Insider Trading (Form 4)

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let filings = client.get_sec_filings("AAPL").await?;
    
    // Find Form 4 filings (insider transactions)
    let form_4_filings: Vec<_> = filings.filings
        .iter()
        .filter(|f| f.filing_type == "4")
        .take(10)
        .collect();
    
    println!("Recent Insider Trading Activity (Form 4):");
    println!("{:<12} {:<50}", "Date", "Title");
    println!("{}", "-".repeat(65));
    
    for filing in form_4_filings {
        println!("{:<12} {:<50}",
            filing.date.format("%Y-%m-%d"),
            filing.title
        );
    }
    
    Ok(())
}
```

### Building a Filing Calendar

```rust
use finance_query_core::YahooClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN"];
    
    let mut filing_calendar: HashMap<String, Vec<String>> = HashMap::new();
    
    for symbol in symbols {
        let filings = client.get_sec_filings(symbol).await?;
        
        // Get recent 10-Q and 10-K filings
        let reports: Vec<String> = filings.filings
            .iter()
            .filter(|f| f.filing_type == "10-Q" || f.filing_type == "10-K")
            .take(2)
            .map(|f| format!("{} - {}",
                f.date.format("%Y-%m-%d"),
                f.filing_type
            ))
            .collect();
        
        filing_calendar.insert(symbol.to_string(), reports);
    }
    
    println!("Recent Financial Reports:");
    for (symbol, reports) in filing_calendar {
        println!("\n{}:", symbol);
        for report in reports {
            println!("  {}", report);
        }
    }
    
    Ok(())
}
```

### Downloading Filing URLs

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let filings = client.get_sec_filings("TSLA").await?;
    
    // Create a list of URLs for all 10-K and 10-Q filings
    let mut urls = Vec::new();
    
    for filing in &filings.filings {
        if filing.filing_type == "10-K" || filing.filing_type == "10-Q" {
            urls.push(format!("{},{},{},{}",
                filing.date.format("%Y-%m-%d"),
                filing.filing_type,
                filing.title.replace(",", ";"),
                filing.url
            ));
        }
    }
    
    // Save to CSV
    let mut csv = String::from("Date,Type,Title,URL\n");
    csv.push_str(&urls.join("\n"));
    
    std::fs::write("tsla_filings.csv", csv)?;
    println!("Saved {} filing URLs to tsla_filings.csv", urls.len());
    
    Ok(())
}
```

### Comparing Filing Frequency

```rust
use finance_query_core::YahooClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "TSLA", "NVDA"];
    
    println!("Filing Frequency Analysis:\n");
    
    for symbol in symbols {
        let filings = client.get_sec_filings(symbol).await?;
        
        // Count filings by type
        let mut counts: HashMap<String, usize> = HashMap::new();
        
        for filing in &filings.filings {
            *counts.entry(filing.filing_type.clone()).or_insert(0) += 1;
        }
        
        println!("{}:", symbol);
        let mut sorted: Vec<_> = counts.iter().collect();
        sorted.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        
        for (filing_type, count) in sorted.iter().take(5) {
            println!("  {}: {}", filing_type, count);
        }
        println!();
    }
    
    Ok(())
}
```

### Finding Proxy Statements

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let filings = client.get_sec_filings("AAPL").await?;
    
    // Find proxy statements (DEF 14A)
    let proxy_statements: Vec<_> = filings.filings
        .iter()
        .filter(|f| f.filing_type.contains("14A"))
        .collect();
    
    println!("Proxy Statements:");
    for proxy in proxy_statements {
        println!("\n{}", proxy.date.format("%Y-%m-%d"));
        println!("Type: {}", proxy.filing_type);
        println!("Title: {}", proxy.title);
        println!("URL: {}", proxy.url);
    }
    
    Ok(())
}
```

### Exporting Complete Filing Data

```rust
use finance_query_core::YahooClient;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let filings = client.get_sec_filings("AAPL").await?;
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&filings)?;
    std::fs::write("aapl_sec_filings.json", json)?;
    
    println!("Exported {} filings to aapl_sec_filings.json", filings.filings.len());
    
    // Print summary
    println!("\nFiling Summary:");
    println!("  Total Filings: {}", filings.filings.len());
    
    if let Some(latest) = filings.filings.first() {
        println!("  Latest Filing: {} ({})",
            latest.filing_type,
            latest.date.format("%Y-%m-%d")
        );
    }
    
    Ok(())
}
```

### Building a Compliance Dashboard

```rust
use finance_query_core::YahooClient;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbol = "MSFT";
    let filings = client.get_sec_filings(symbol).await?;
    
    let ninety_days_ago = Utc::now() - Duration::days(90);
    
    println!("Compliance Dashboard for {}", symbol);
    println!("Last 90 Days\n");
    
    // Check for required filings
    let has_10q = filings.filings.iter().any(|f| {
        f.filing_type == "10-Q" && f.date > ninety_days_ago
    });
    
    let has_8k = filings.filings.iter().any(|f| {
        f.filing_type == "8-K" && f.date > ninety_days_ago
    });
    
    println!("10-Q Filed: {}", if has_10q { "✓" } else { "✗" });
    println!("8-K Filed: {}", if has_8k { "✓" } else { "✗" });
    
    // List all recent filings
    println!("\nRecent Filings:");
    for filing in filings.filings.iter().filter(|f| f.date > ninety_days_ago) {
        println!("  {} - {}", 
            filing.date.format("%Y-%m-%d"),
            filing.filing_type
        );
    }
    
    Ok(())
}
```

## Important Notes

### Data Availability
- Only available for U.S. public companies
- Foreign companies may have limited filings (6-K forms)
- Private companies do not file with the SEC
- Historical filings may be limited

### Filing Timing
- 10-K: Due 60-90 days after fiscal year end (depending on company size)
- 10-Q: Due 40-45 days after quarter end
- 8-K: Due 4 business days after material event
- Form 4: Due 2 business days after insider transaction

### URL Structure
- URLs point to SEC EDGAR database
- Links may be to search pages or direct document links
- Some exhibits may require additional navigation
- Documents are typically in HTML or XBRL format

### Best Practices
- Check filing dates to ensure data is current
- Review exhibits for detailed information
- Compare filings across periods for trends
- Use 8-K filings to track material events
- Monitor Form 4 for insider trading patterns

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_sec_filings("AAPL").await {
        Ok(filings) => {
            if filings.filings.is_empty() {
                println!("No SEC filings available");
            } else {
                println!("Found {} filings", filings.filings.len());
                for filing in filings.filings.iter().take(5) {
                    println!("{} - {}",
                        filing.date.format("%Y-%m-%d"),
                        filing.filing_type
                    );
                }
            }
        }
        Err(YahooError::NotFound) => {
            eprintln!("Symbol not found or no filings available");
        }
        Err(YahooError::ParseError(msg)) => {
            eprintln!("Failed to parse filings: {}", msg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

## See Also

- [Calendar Model](./calendar.md) - For earnings dates that often coincide with filings
- [News Model](./news.md) - For news about filing-related events
- [Quote Model](./quote.md) - For company information and fundamentals
- [Financials Model](./financials.md) - For parsed financial statement data

## External Resources

- [SEC EDGAR Database](https://www.sec.gov/edgar/searchedgar/companysearch.html) - Official SEC filing search
- [SEC Filing Types](https://www.sec.gov/forms) - Complete list of SEC forms
- [EDGAR Filing Manual](https://www.sec.gov/info/edgar/edmanuals.htm) - Technical documentation
