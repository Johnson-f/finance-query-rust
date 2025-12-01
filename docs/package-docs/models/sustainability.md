# Sustainability (ESG) Scores

The sustainability module provides access to ESG (Environmental, Social, and Governance) scores and related sustainability metrics for publicly traded companies.

## Overview

ESG scores help investors evaluate a company's performance on environmental, social, and governance factors. This module fetches and parses ESG data from Yahoo Finance, providing comprehensive sustainability metrics.

## Data Structure

### `SustainabilityScores`

The main struct containing all ESG-related information:

```rust
pub struct SustainabilityScores {
    pub symbol: String,
    pub total_esg: Option<f64>,
    pub environment_score: Option<f64>,
    pub social_score: Option<f64>,
    pub governance_score: Option<f64>,
    pub controversy_level: Option<u8>,
    pub percentile: Option<f64>,
    pub peer_group: Option<String>,
    pub peer_esg_score_performance: Option<String>,
    pub related_controversy: Option<Vec<String>>,
}
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `symbol` | `String` | Stock ticker symbol |
| `total_esg` | `Option<f64>` | Overall ESG score (0-100, higher is better) |
| `environment_score` | `Option<f64>` | Environmental pillar score |
| `social_score` | `Option<f64>` | Social pillar score |
| `governance_score` | `Option<f64>` | Governance pillar score |
| `controversy_level` | `Option<u8>` | Controversy level (0-5, where 5 is most controversial) |
| `percentile` | `Option<f64>` | ESG percentile rank compared to all companies |
| `peer_group` | `Option<String>` | Industry peer group classification |
| `peer_esg_score_performance` | `Option<String>` | Performance relative to peers (e.g., "AVG", "OUTPERFORM") |
| `related_controversy` | `Option<Vec<String>>` | List of controversy topics |

## Usage

### Basic Example

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    // Fetch ESG scores for a company
    let esg = client.get_sustainability("AAPL").await?;
    
    println!("ESG Score for {}: {:?}", esg.symbol, esg.total_esg);
    println!("Environment: {:?}", esg.environment_score);
    println!("Social: {:?}", esg.social_score);
    println!("Governance: {:?}", esg.governance_score);
    
    Ok(())
}
```

### Checking Data Availability

```rust
let esg = client.get_sustainability("TSLA").await?;

if esg.has_data() {
    println!("ESG data is available");
} else {
    println!("No ESG data available for this symbol");
}
```

### Getting ESG Rating

The module provides a convenience method to get a letter grade (A-F) based on the total ESG score:

```rust
let esg = client.get_sustainability("MSFT").await?;

if let Some(rating) = esg.rating() {
    println!("ESG Rating: {}", rating);
    // Output: ESG Rating: A
}
```

Rating scale:
- **A**: Score ≥ 70 (Excellent)
- **B**: Score ≥ 50 (Good)
- **C**: Score ≥ 30 (Average)
- **D**: Score ≥ 20 (Below Average)
- **F**: Score < 20 (Poor)

### Analyzing Controversies

```rust
let esg = client.get_sustainability("XOM").await?;

if let Some(level) = esg.controversy_level {
    println!("Controversy Level: {}/5", level);
}

if let Some(controversies) = &esg.related_controversy {
    println!("Related Controversies:");
    for controversy in controversies {
        println!("  - {}", controversy);
    }
}
```

### Comparing to Peers

```rust
let esg = client.get_sustainability("GOOGL").await?;

if let Some(peer_group) = &esg.peer_group {
    println!("Peer Group: {}", peer_group);
}

if let Some(performance) = &esg.peer_esg_score_performance {
    println!("Performance vs Peers: {}", performance);
}

if let Some(percentile) = esg.percentile {
    println!("Percentile Rank: {:.1}%", percentile);
}
```

## JSON Response Format

When serialized to JSON, the `SustainabilityScores` struct produces the following format:

### Example Response

```json
{
  "symbol": "AAPL",
  "totalEsg": 18.12,
  "environmentScore": 0.58,
  "socialScore": 10.75,
  "governanceScore": 6.79,
  "controversyLevel": 3,
  "percentile": 35.42,
  "peerGroup": "Technology Hardware",
  "peerEsgScorePerformance": "AVG",
  "relatedControversy": [
    "Business Ethics",
    "Product Quality & Safety",
    "Customer Incidents"
  ]
}
```

### Field Descriptions in JSON

- **`totalEsg`**: Aggregate ESG risk score (lower is better in Yahoo's scoring)
- **`environmentScore`**: Environmental risk score
- **`socialScore`**: Social risk score  
- **`governanceScore`**: Governance risk score
- **`controversyLevel`**: Integer from 0-5 indicating severity of controversies
- **`percentile`**: Percentile ranking (higher percentile = better performance)
- **`peerGroup`**: Industry classification for peer comparison
- **`peerEsgScorePerformance`**: Relative performance ("OUTPERFORM", "AVG", "UNDERPERFORM")
- **`relatedControversy`**: Array of controversy categories

### Null Values

Fields may be `null` if data is not available:

```json
{
  "symbol": "PRIVATE",
  "totalEsg": null,
  "environmentScore": null,
  "socialScore": null,
  "governanceScore": null,
  "controversyLevel": null,
  "percentile": null,
  "peerGroup": null,
  "peerEsgScorePerformance": null,
  "relatedControversy": null
}
```

## Complete Example

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let symbols = vec!["AAPL", "TSLA", "MSFT", "GOOGL"];
    
    for symbol in symbols {
        match client.get_sustainability(symbol).await {
            Ok(esg) => {
                if esg.has_data() {
                    println!("\n{} ESG Analysis:", symbol);
                    println!("  Total Score: {:.2}", esg.total_esg.unwrap_or(0.0));
                    println!("  Rating: {}", esg.rating().unwrap_or("N/A"));
                    
                    if let Some(env) = esg.environment_score {
                        println!("  Environment: {:.2}", env);
                    }
                    if let Some(soc) = esg.social_score {
                        println!("  Social: {:.2}", soc);
                    }
                    if let Some(gov) = esg.governance_score {
                        println!("  Governance: {:.2}", gov);
                    }
                    
                    if let Some(level) = esg.controversy_level {
                        println!("  Controversy Level: {}/5", level);
                    }
                    
                    if let Some(percentile) = esg.percentile {
                        println!("  Percentile: {:.1}%", percentile);
                    }
                } else {
                    println!("{}: No ESG data available", symbol);
                }
            }
            Err(e) => eprintln!("Error fetching {} ESG data: {}", symbol, e),
        }
    }
    
    Ok(())
}
```

## Important Notes

1. **Data Availability**: Not all companies have ESG scores. Use `has_data()` to check before accessing scores.

2. **Scoring System**: Yahoo Finance uses a risk-based scoring system where lower scores generally indicate better ESG performance (less risk).

3. **Controversy Levels**: 
   - 0-1: No or minor controversies
   - 2-3: Moderate controversies
   - 4-5: Severe controversies

4. **Update Frequency**: ESG scores are typically updated quarterly or when significant events occur.

5. **Peer Comparisons**: Peer group classifications help contextualize scores within the same industry.

## Error Handling

```rust
match client.get_sustainability("INVALID").await {
    Ok(esg) => {
        if !esg.has_data() {
            println!("Symbol exists but has no ESG data");
        }
    }
    Err(e) => {
        eprintln!("Failed to fetch ESG data: {}", e);
    }
}
```

## See Also

- [Calendar Events](./calendar.md) - Includes ESG-related events
- [News](./news.md) - ESG-related news articles
- [Quote](./quote.md) - Basic company information
