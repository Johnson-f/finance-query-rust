# Earnings Transcripts API

The Earnings Transcripts API provides access to earnings call transcripts, including speaker information and full text content from quarterly earnings announcements.

## Overview

This module provides access to:

- **Earnings Call Listings**: List of available earnings calls for a symbol
- **Full Transcripts**: Complete earnings call transcripts with speaker attribution
- **Speaker Information**: Details about executives and analysts participating
- **Structured Content**: Organized paragraphs with speaker identification

## Data Structures

### EarningsCallsList

List of available earnings calls for a symbol.

```rust
pub struct EarningsCallsList {
    pub symbol: String,
    pub earnings_calls: Vec<EarningsCallListing>,
    pub total: usize,
}
```

**Fields:**
- `symbol`: Stock ticker symbol
- `earnings_calls`: List of available earnings calls
- `total`: Total number of earnings calls available

### EarningsCallListing

Individual earnings call metadata.

```rust
pub struct EarningsCallListing {
    pub event_id: String,
    pub quarter: Option<String>,
    pub year: Option<i32>,
    pub title: String,
    pub url: String,
}
```

**Fields:**
- `event_id`: Unique identifier for the earnings call
- `quarter`: Quarter (e.g., "Q1", "Q2", "Q3", "Q4")
- `year`: Year of the earnings call
- `title`: Full title of the earnings call
- `url`: URL to access the transcript


### EarningsTranscript

Complete earnings call transcript with full content.

```rust
pub struct EarningsTranscript {
    pub symbol: String,
    pub quarter: String,
    pub year: i32,
    pub date: DateTime<Utc>,
    pub title: String,
    pub speakers: Vec<TranscriptSpeaker>,
    pub paragraphs: Vec<TranscriptParagraph>,
    pub metadata: HashMap<String, serde_json::Value>,
}
```

**Fields:**
- `symbol`: Stock ticker symbol
- `quarter`: Quarter identifier (e.g., "Q1", "Q2")
- `year`: Year of the earnings call
- `date`: Date and time of the earnings call (UTC)
- `title`: Full title of the earnings call
- `speakers`: List of all speakers in the call
- `paragraphs`: Transcript content organized by speaker
- `metadata`: Additional metadata about the transcript

### TranscriptSpeaker

Information about a speaker in the earnings call.

```rust
pub struct TranscriptSpeaker {
    pub name: String,
    pub role: Option<String>,
    pub company: Option<String>,
}
```

**Fields:**
- `name`: Speaker's full name
- `role`: Speaker's role/title (e.g., "CEO", "CFO", "Analyst")
- `company`: Company affiliation (for analysts)

### TranscriptParagraph

Individual paragraph from the transcript.

```rust
pub struct TranscriptParagraph {
    pub speaker: String,
    pub text: String,
}
```

**Fields:**
- `speaker`: Name of the speaker
- `text`: Content of what was said

## Usage Examples

### List Available Earnings Calls

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let calls = client.get_earnings_calls_list("AAPL").await?;
    
    println!("Earnings Calls for {}", calls.symbol);
    println!("Total available: {}\n", calls.total);
    
    for call in &calls.earnings_calls {
        println!("Event ID: {}", call.event_id);
        
        if let (Some(quarter), Some(year)) = (&call.quarter, call.year) {
            println!("Period: {} {}", quarter, year);
        }
        
        println!("Title: {}", call.title);
        println!("URL: {}", call.url);
        println!();
    }
    
    Ok(())
}
```

### Get Full Transcript

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transcript = client.get_earnings_transcript("AAPL", "event_id_123").await?;
    
    println!("Earnings Call Transcript");
    println!("========================");
    println!("Symbol: {}", transcript.symbol);
    println!("Period: {} {}", transcript.quarter, transcript.year);
    println!("Date: {}", transcript.date.format("%Y-%m-%d %H:%M UTC"));
    println!("Title: {}", transcript.title);
    println!("\nSpeakers: {}", transcript.speakers.len());
    println!("Paragraphs: {}", transcript.paragraphs.len());
    
    Ok(())
}
```

### Display Speakers

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transcript = client.get_earnings_transcript("MSFT", "event_id_456").await?;
    
    println!("Speakers in {} {} {} Earnings Call:\n", 
        transcript.symbol, 
        transcript.quarter, 
        transcript.year
    );
    
    for speaker in &transcript.speakers {
        print!("{}", speaker.name);
        
        if let Some(role) = &speaker.role {
            print!(" - {}", role);
        }
        
        if let Some(company) = &speaker.company {
            print!(" ({})", company);
        }
        
        println!();
    }
    
    Ok(())
}
```

### Read Transcript Content

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transcript = client.get_earnings_transcript("GOOGL", "event_id_789").await?;
    
    println!("Transcript Content:\n");
    println!("═══════════════════════════════════════\n");
    
    for (i, para) in transcript.paragraphs.iter().enumerate() {
        println!("[{}] {}", para.speaker, para.text);
        
        // Add separator every 5 paragraphs for readability
        if (i + 1) % 5 == 0 {
            println!("\n---\n");
        } else {
            println!();
        }
    }
    
    Ok(())
}
```

### Search Transcript for Keywords

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transcript = client.get_earnings_transcript("NVDA", "event_id_101").await?;
    let keywords = vec!["AI", "artificial intelligence", "data center"];
    
    println!("Searching transcript for keywords: {:?}\n", keywords);
    
    for keyword in &keywords {
        let matches: Vec<_> = transcript.paragraphs
            .iter()
            .filter(|p| p.text.to_lowercase().contains(&keyword.to_lowercase()))
            .collect();
        
        println!("Keyword: \"{}\" - {} mentions", keyword, matches.len());
        
        if !matches.is_empty() {
            println!("Sample mentions:");
            for (i, para) in matches.iter().take(3).enumerate() {
                println!("  {}. {} said: \"{}...\"", 
                    i + 1,
                    para.speaker,
                    &para.text.chars().take(100).collect::<String>()
                );
            }
        }
        println!();
    }
    
    Ok(())
}
```

### Analyze Speaker Participation

```rust
use finance_query_core::YahooClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transcript = client.get_earnings_transcript("TSLA", "event_id_202").await?;
    
    // Count paragraphs per speaker
    let mut speaker_counts: HashMap<String, usize> = HashMap::new();
    let mut speaker_words: HashMap<String, usize> = HashMap::new();
    
    for para in &transcript.paragraphs {
        *speaker_counts.entry(para.speaker.clone()).or_insert(0) += 1;
        let word_count = para.text.split_whitespace().count();
        *speaker_words.entry(para.speaker.clone()).or_insert(0) += word_count;
    }
    
    println!("Speaker Participation Analysis:\n");
    
    let mut speakers: Vec<_> = speaker_counts.iter().collect();
    speakers.sort_by(|a, b| b.1.cmp(a.1));
    
    for (speaker, count) in speakers {
        let words = speaker_words.get(speaker).unwrap_or(&0);
        let avg_words = if *count > 0 { words / count } else { 0 };
        
        println!("{}", speaker);
        println!("  Statements: {}", count);
        println!("  Total words: {}", words);
        println!("  Avg words per statement: {}", avg_words);
        println!();
    }
    
    Ok(())
}
```

### Extract Q&A Section

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transcript = client.get_earnings_transcript("META", "event_id_303").await?;
    
    // Find where Q&A starts (typically indicated by "Analyst" speakers)
    let qa_start = transcript.paragraphs
        .iter()
        .position(|p| {
            transcript.speakers
                .iter()
                .find(|s| s.name == p.speaker)
                .and_then(|s| s.role.as_ref())
                .map(|r| r.contains("Analyst"))
                .unwrap_or(false)
        });
    
    if let Some(start_idx) = qa_start {
        println!("Q&A Section (starting at paragraph {}):\n", start_idx + 1);
        
        for para in &transcript.paragraphs[start_idx..] {
            // Find speaker info
            let speaker_info = transcript.speakers
                .iter()
                .find(|s| s.name == para.speaker);
            
            if let Some(info) = speaker_info {
                if let Some(role) = &info.role {
                    if role.contains("Analyst") {
                        println!("Q: [{}] {}", para.speaker, para.text);
                    } else {
                        println!("A: [{}] {}", para.speaker, para.text);
                    }
                    println!();
                }
            }
        }
    } else {
        println!("Q&A section not found in transcript");
    }
    
    Ok(())
}
```

### Compare Multiple Quarters

```rust
use finance_query_core::YahooClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let calls = client.get_earnings_calls_list("AMZN").await?;
    
    println!("Recent Earnings Calls for {}:\n", calls.symbol);
    
    // Get the most recent 4 quarters
    for call in calls.earnings_calls.iter().take(4) {
        if let Ok(transcript) = client.get_earnings_transcript(
            &calls.symbol, 
            &call.event_id
        ).await {
            println!("{} {}:", transcript.quarter, transcript.year);
            println!("  Date: {}", transcript.date.format("%Y-%m-%d"));
            println!("  Speakers: {}", transcript.speakers.len());
            println!("  Length: {} paragraphs", transcript.paragraphs.len());
            
            // Calculate total word count
            let total_words: usize = transcript.paragraphs
                .iter()
                .map(|p| p.text.split_whitespace().count())
                .sum();
            
            println!("  Total words: {}", total_words);
            println!();
        }
    }
    
    Ok(())
}
```

### Sentiment Analysis Helper

```rust
use finance_query_core::YahooClient;

async fn analyze_sentiment_keywords(
    transcript: &finance_query_core::models::EarningsTranscript
) -> (usize, usize) {
    let positive_words = vec![
        "growth", "strong", "increase", "positive", "success",
        "opportunity", "optimistic", "exceed", "record", "momentum"
    ];
    
    let negative_words = vec![
        "decline", "decrease", "challenge", "concern", "weak",
        "difficult", "pressure", "risk", "uncertainty", "headwind"
    ];
    
    let mut positive_count = 0;
    let mut negative_count = 0;
    
    for para in &transcript.paragraphs {
        let text_lower = para.text.to_lowercase();
        
        for word in &positive_words {
            positive_count += text_lower.matches(word).count();
        }
        
        for word in &negative_words {
            negative_count += text_lower.matches(word).count();
        }
    }
    
    (positive_count, negative_count)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    
    let transcript = client.get_earnings_transcript("AAPL", "event_id_404").await?;
    
    let (positive, negative) = analyze_sentiment_keywords(&transcript).await;
    
    println!("Sentiment Analysis for {} {} {}:\n", 
        transcript.symbol, 
        transcript.quarter, 
        transcript.year
    );
    
    println!("Positive keywords: {}", positive);
    println!("Negative keywords: {}", negative);
    
    let total = positive + negative;
    if total > 0 {
        let positive_pct = (positive as f64 / total as f64) * 100.0;
        println!("Positive sentiment: {:.1}%", positive_pct);
        
        if positive_pct > 60.0 {
            println!("Overall tone: POSITIVE ✓");
        } else if positive_pct < 40.0 {
            println!("Overall tone: NEGATIVE ✗");
        } else {
            println!("Overall tone: NEUTRAL");
        }
    }
    
    Ok(())
}
```


## JSON Response Formats

### Earnings Calls List Response

```json
{
  "symbol": "AAPL",
  "earnings_calls": [
    {
      "event_id": "abc123xyz",
      "quarter": "Q4",
      "year": 2024,
      "title": "Apple Inc. Q4 2024 Earnings Call",
      "url": "https://finance.yahoo.com/..."
    },
    {
      "event_id": "def456uvw",
      "quarter": "Q3",
      "year": 2024,
      "title": "Apple Inc. Q3 2024 Earnings Call",
      "url": "https://finance.yahoo.com/..."
    },
    {
      "event_id": "ghi789rst",
      "quarter": "Q2",
      "year": 2024,
      "title": "Apple Inc. Q2 2024 Earnings Call",
      "url": "https://finance.yahoo.com/..."
    }
  ],
  "total": 3
}
```

### Full Transcript Response

```json
{
  "symbol": "AAPL",
  "quarter": "Q4",
  "year": 2024,
  "date": "2024-10-31T21:00:00Z",
  "title": "Apple Inc. Q4 2024 Earnings Call",
  "speakers": [
    {
      "name": "Tim Cook",
      "role": "CEO",
      "company": "Apple Inc."
    },
    {
      "name": "Luca Maestri",
      "role": "CFO",
      "company": "Apple Inc."
    },
    {
      "name": "John Analyst",
      "role": "Analyst",
      "company": "Morgan Stanley"
    }
  ],
  "paragraphs": [
    {
      "speaker": "Tim Cook",
      "text": "Good afternoon and thank you for joining us. Today we are reporting revenue of $89.5 billion, up 6% year over year..."
    },
    {
      "speaker": "Luca Maestri",
      "text": "Thank you, Tim. Let me provide more details on our financial results. Our gross margin was 46.2%, up 130 basis points..."
    },
    {
      "speaker": "John Analyst",
      "text": "Thanks for taking my question. Can you provide more color on iPhone demand in international markets?"
    },
    {
      "speaker": "Tim Cook",
      "text": "Sure, John. We're seeing strong demand across all regions, particularly in emerging markets where we've seen double-digit growth..."
    }
  ],
  "metadata": {
    "duration_minutes": 60,
    "call_type": "earnings",
    "fiscal_period": "Q4 2024"
  }
}
```

### Minimal Listing (Missing Optional Fields)

```json
{
  "symbol": "NEWCO",
  "earnings_calls": [
    {
      "event_id": "xyz789abc",
      "title": "NEWCO Earnings Call",
      "url": "https://finance.yahoo.com/..."
    }
  ],
  "total": 1
}
```

### Speaker Without Company

```json
{
  "name": "Jane Smith",
  "role": "CEO"
}
```

## Field Details

### Event ID

- Unique identifier for each earnings call
- Used to retrieve full transcript
- Format varies by data source
- Required for transcript retrieval

### Quarter Codes

Standard quarter identifiers:
- `"Q1"`: First quarter (Jan-Mar for calendar year)
- `"Q2"`: Second quarter (Apr-Jun)
- `"Q3"`: Third quarter (Jul-Sep)
- `"Q4"`: Fourth quarter (Oct-Dec)

Note: Fiscal quarters may differ from calendar quarters depending on company fiscal year.

### Speaker Roles

Common speaker roles in earnings calls:

**Company Executives:**
- `"CEO"`: Chief Executive Officer
- `"CFO"`: Chief Financial Officer
- `"COO"`: Chief Operating Officer
- `"CTO"`: Chief Technology Officer
- `"Investor Relations"`: IR representative

**Analysts:**
- `"Analyst"`: Financial analyst
- May include firm name in company field

### Date Format

- All dates are ISO 8601 formatted UTC timestamps
- Format: `YYYY-MM-DDTHH:MM:SSZ`
- Earnings calls typically occur after market close (21:00 UTC / 4:00 PM ET)
- Or before market open (13:00 UTC / 8:00 AM ET)

### Metadata

The metadata field can contain various additional information:
- Call duration
- Call type (earnings, special event, etc.)
- Fiscal period details
- Recording information
- Additional context

## Common Use Cases

### 1. Build Transcript Search Engine

```rust
use finance_query_core::YahooClient;

async fn search_transcripts(
    symbol: &str,
    search_term: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let calls = client.get_earnings_calls_list(symbol).await?;
    
    let mut results = Vec::new();
    
    for call in &calls.earnings_calls {
        if let Ok(transcript) = client.get_earnings_transcript(
            symbol, 
            &call.event_id
        ).await {
            let mentions = transcript.paragraphs
                .iter()
                .filter(|p| p.text.to_lowercase().contains(&search_term.to_lowercase()))
                .count();
            
            if mentions > 0 {
                results.push(format!(
                    "{} {}: {} mentions",
                    transcript.quarter,
                    transcript.year,
                    mentions
                ));
            }
        }
    }
    
    Ok(results)
}
```

### 2. Extract Key Metrics Mentioned

```rust
use finance_query_core::YahooClient;
use regex::Regex;

async fn extract_revenue_mentions(
    transcript: &finance_query_core::models::EarningsTranscript
) -> Vec<String> {
    let revenue_regex = Regex::new(r"\$[\d,]+\.?\d*\s*(billion|million)").unwrap();
    let mut mentions = Vec::new();
    
    for para in &transcript.paragraphs {
        for capture in revenue_regex.find_iter(&para.text) {
            mentions.push(format!(
                "{}: {}",
                para.speaker,
                capture.as_str()
            ));
        }
    }
    
    mentions
}
```

### 3. Generate Executive Summary

```rust
use finance_query_core::YahooClient;

async fn generate_summary(
    transcript: &finance_query_core::models::EarningsTranscript
) -> String {
    // Get opening remarks (typically first few paragraphs from CEO/CFO)
    let opening_remarks: Vec<_> = transcript.paragraphs
        .iter()
        .take(5)
        .filter(|p| {
            transcript.speakers
                .iter()
                .find(|s| s.name == p.speaker)
                .and_then(|s| s.role.as_ref())
                .map(|r| r.contains("CEO") || r.contains("CFO"))
                .unwrap_or(false)
        })
        .collect();
    
    let mut summary = format!(
        "Earnings Call Summary - {} {} {}\n\n",
        transcript.symbol,
        transcript.quarter,
        transcript.year
    );
    
    summary.push_str("Key Opening Remarks:\n");
    for remark in opening_remarks {
        let preview = remark.text.chars().take(200).collect::<String>();
        summary.push_str(&format!("- {}: {}...\n", remark.speaker, preview));
    }
    
    summary
}
```

### 4. Track Management Tone Over Time

```rust
use finance_query_core::YahooClient;

async fn track_management_tone(
    symbol: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let calls = client.get_earnings_calls_list(symbol).await?;
    
    println!("Management Tone Analysis for {}:\n", symbol);
    
    for call in calls.earnings_calls.iter().take(4) {
        if let Ok(transcript) = client.get_earnings_transcript(
            symbol,
            &call.event_id
        ).await {
            // Count confidence indicators
            let confidence_words = vec!["confident", "strong", "optimistic", "positive"];
            let caution_words = vec!["cautious", "uncertain", "challenging", "difficult"];
            
            let mut confidence_score = 0;
            let mut caution_score = 0;
            
            for para in &transcript.paragraphs {
                let text_lower = para.text.to_lowercase();
                
                for word in &confidence_words {
                    confidence_score += text_lower.matches(word).count();
                }
                
                for word in &caution_words {
                    caution_score += text_lower.matches(word).count();
                }
            }
            
            println!("{} {}:", transcript.quarter, transcript.year);
            println!("  Confidence indicators: {}", confidence_score);
            println!("  Caution indicators: {}", caution_score);
            
            let tone = if confidence_score > caution_score * 2 {
                "Very Positive"
            } else if confidence_score > caution_score {
                "Positive"
            } else if caution_score > confidence_score {
                "Cautious"
            } else {
                "Neutral"
            };
            
            println!("  Overall tone: {}", tone);
            println!();
        }
    }
    
    Ok(())
}
```

### 5. Identify Most Active Analysts

```rust
use finance_query_core::YahooClient;
use std::collections::HashMap;

async fn identify_active_analysts(
    symbol: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = YahooClient::new();
    let calls = client.get_earnings_calls_list(symbol).await?;
    
    let mut analyst_participation: HashMap<String, usize> = HashMap::new();
    
    for call in &calls.earnings_calls {
        if let Ok(transcript) = client.get_earnings_transcript(
            symbol,
            &call.event_id
        ).await {
            for para in &transcript.paragraphs {
                // Check if speaker is an analyst
                if let Some(speaker) = transcript.speakers
                    .iter()
                    .find(|s| s.name == para.speaker) {
                    
                    if let Some(role) = &speaker.role {
                        if role.contains("Analyst") {
                            *analyst_participation
                                .entry(para.speaker.clone())
                                .or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("Most Active Analysts for {}:\n", symbol);
    
    let mut analysts: Vec<_> = analyst_participation.iter().collect();
    analysts.sort_by(|a, b| b.1.cmp(a.1));
    
    for (analyst, questions) in analysts.iter().take(10) {
        println!("{}: {} questions", analyst, questions);
    }
    
    Ok(())
}
```

## Important Notes

### Data Availability

- Not all companies have transcripts available
- Transcript availability varies by company and date
- Recent calls are more likely to have transcripts
- Some companies may restrict transcript access
- Historical transcripts may be limited

### Content Accuracy

- Transcripts are typically automated and may contain errors
- Speaker attribution is generally accurate but verify for critical use
- Technical terms or product names may be transcribed incorrectly
- Numbers and financial figures should be cross-referenced with official filings

### Usage Considerations

- Transcripts can be very long (10,000+ words)
- Consider pagination or chunking for display
- Full text search may be resource-intensive
- Cache transcripts locally to reduce API calls
- Respect rate limits when fetching multiple transcripts

### Legal and Compliance

- Transcripts are for informational purposes
- Do not use for automated trading without proper compliance
- Verify important information with official sources
- Be aware of fair use and copyright considerations
- Some content may be subject to company restrictions

## Best Practices

1. **Cache Transcripts**: Store locally to avoid repeated API calls
2. **Index Content**: Build search indices for faster keyword lookup
3. **Chunk Large Transcripts**: Break into manageable sections for processing
4. **Verify Speakers**: Cross-reference speaker information with company data
5. **Handle Missing Data**: Not all fields are always available
6. **Rate Limiting**: Implement delays when fetching multiple transcripts
7. **Error Handling**: Gracefully handle missing or unavailable transcripts
8. **Text Processing**: Clean and normalize text before analysis
9. **Context Matters**: Consider full context, not just keyword counts
10. **Combine Sources**: Use with other APIs (financials, analyst data) for complete picture

## Error Handling

```rust
use finance_query_core::{YahooClient, YahooError};

#[tokio::main]
async fn main() {
    let client = YahooClient::new();
    
    match client.get_earnings_calls_list("AAPL").await {
        Ok(calls) => {
            if calls.earnings_calls.is_empty() {
                println!("No earnings calls available");
            } else {
                println!("Found {} earnings calls", calls.total);
                
                // Try to get first transcript
                if let Some(first_call) = calls.earnings_calls.first() {
                    match client.get_earnings_transcript(
                        &calls.symbol,
                        &first_call.event_id
                    ).await {
                        Ok(transcript) => {
                            println!("Transcript loaded: {} paragraphs", 
                                transcript.paragraphs.len());
                        }
                        Err(e) => {
                            println!("Failed to load transcript: {}", e);
                        }
                    }
                }
            }
        }
        Err(YahooError::NotFound) => {
            println!("Symbol not found");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

## Related APIs

- **Calendar API**: Get upcoming earnings dates
- **Earnings History**: Get historical earnings results (Analyst API)
- **Fundamentals API**: Get official financial results
- **News API**: Get earnings-related news articles

## Performance Tips

- Transcripts can be 50KB+ of text data
- Consider streaming or pagination for large transcripts
- Use async/await for concurrent transcript fetching
- Implement caching to reduce API load
- Index transcripts for faster searching
- Use text compression for storage

