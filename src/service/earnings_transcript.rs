use crate::client::{scraper, YahooFinanceClient};
use crate::client::error::YahooError;
use crate::client::FetchClient;
use crate::models::{
    EarningsCallListing, EarningsCallsList, EarningsTranscript, TranscriptParagraph, TranscriptSpeaker,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

pub async fn get_earnings_calls_list(
    _yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
) -> Result<EarningsCallsList, YahooError> {
    // Scrape earnings calls page to get list
    debug!("Fetching earnings calls list for symbol: {}", symbol);
    let calls_data = match scraper::scrape_earnings_calls_list(fetch_client, symbol).await {
        Ok(data) => {
            debug!("Scraper returned {} earnings calls for {}", data.len(), symbol);
            data
        }
        Err(e) => {
            warn!("Failed to scrape earnings calls for {}: {}", symbol, e);
            return Err(e);
        }
    };

    if calls_data.is_empty() {
        warn!("No earnings calls found for {} - scraper returned empty list", symbol);
        return Err(YahooError::NotFound(format!(
            "No earnings calls found for {}. The symbol may not have earnings transcripts available, or the page structure may have changed.",
            symbol
        )));
    }

    // Convert to EarningsCallListing structs
    let mut earnings_calls: Vec<EarningsCallListing> = calls_data
        .into_iter()
        .filter_map(|call_value| {
            let obj = call_value.as_object()?;
            Some(EarningsCallListing {
                event_id: obj.get("eventId")?.as_str()?.to_string(),
                quarter: obj.get("quarter")?.as_str().map(|s| s.to_string()),
                year: obj.get("year")?.as_i64().map(|y| y as i32),
                title: obj.get("title")?.as_str()?.to_string(),
                url: obj.get("url")?.as_str()?.to_string(),
            })
        })
        .collect();

    // Sort by year and quarter (most recent first)
    earnings_calls.sort_by(|a, b| {
        let a_quarter_num = a
            .quarter
            .as_ref()
            .and_then(|q| q.chars().nth(1))
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);
        let b_quarter_num = b
            .quarter
            .as_ref()
            .and_then(|q| q.chars().nth(1))
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);
        let a_year = a.year.unwrap_or(0);
        let b_year = b.year.unwrap_or(0);
        (b_year, b_quarter_num).cmp(&(a_year, a_quarter_num))
    });

    Ok(EarningsCallsList {
        symbol: symbol.to_uppercase(),
        total: earnings_calls.len(),
        earnings_calls,
    })
}

pub async fn get_earnings_transcript(
    _yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    symbol: &str,
    quarter: Option<String>,
    year: Option<i32>,
) -> Result<EarningsTranscript, YahooError> {
    // Get list of available earnings calls
    let calls_list = get_earnings_calls_list(_yahoo_client, fetch_client, symbol).await?;

    if calls_list.earnings_calls.is_empty() {
        return Err(YahooError::NotFound(format!(
            "No earnings calls found for {}",
            symbol
        )));
    }

    // Filter by quarter/year if specified, otherwise use most recent call
    let target_call = if let (Some(ref q), Some(y)) = (quarter.as_ref(), year) {
        // Normalize quarter format
        let normalized_quarter = if q.starts_with("Q") || q.starts_with("q") {
            q.to_uppercase()
        } else {
            format!("Q{}", q.to_uppercase())
        };

        // Find matching call
        calls_list
            .earnings_calls
            .iter()
            .find(|call| {
                let quarter_match = call.quarter.as_ref().map(|cq| cq == &normalized_quarter).unwrap_or(false);
                let year_match = call.year == Some(y);
                quarter_match && year_match
            })
            .ok_or_else(|| {
                YahooError::ParseError(format!(
                    "No earnings call found for {} {} {}",
                    symbol,
                    normalized_quarter,
                    y
                ))
            })?
    } else {
        // Get the most recent call (first in sorted list)
        &calls_list.earnings_calls[0]
    };

    // Scrape the transcript from the URL
    debug!("Scraping transcript from URL: {}", target_call.url);
    let transcript_data = scraper::scrape_earnings_transcript_from_url(fetch_client, &target_call.url).await?;

    // Parse the transcript
    parse_transcript(symbol, &transcript_data, target_call)
}

fn parse_transcript(
    symbol: &str,
    transcript_data: &Value,
    call_info: &EarningsCallListing,
) -> Result<EarningsTranscript, YahooError> {
    let content = transcript_data
        .get("transcriptContent")
        .ok_or_else(|| YahooError::ParseError("Missing transcriptContent in response".to_string()))?;
    let metadata = transcript_data
        .get("transcriptMetadata")
        .ok_or_else(|| YahooError::ParseError("Missing transcriptMetadata in response".to_string()))?;

    // Parse speaker mapping
    let mut speaker_mapping: HashMap<String, String> = HashMap::new();
    let mut speakers_list: Vec<TranscriptSpeaker> = Vec::new();

    if let Some(speaker_mapping_array) = content.get("speaker_mapping").and_then(|sm| sm.as_array()) {
        for speaker_data in speaker_mapping_array {
            if let Some(speaker_id) = speaker_data.get("speaker").and_then(|s| s.as_str()) {
                let speaker_info = speaker_data.get("speaker_data").and_then(|sd| sd.as_object());
                let name = speaker_info
                    .and_then(|si| si.get("name"))
                    .and_then(|n| n.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("Speaker {}", speaker_id));
                let role = speaker_info
                    .and_then(|si| si.get("role"))
                    .and_then(|r| r.as_str())
                    .map(|s| s.to_string());
                let company = speaker_info
                    .and_then(|si| si.get("company"))
                    .and_then(|c| c.as_str())
                    .map(|s| s.to_string());

                speaker_mapping.insert(speaker_id.to_string(), name.clone());
                speakers_list.push(TranscriptSpeaker { name, role, company });
            }
        }
    }

    // Parse paragraphs
    let transcript_obj = content
        .get("transcript")
        .and_then(|t| t.as_object())
        .ok_or_else(|| YahooError::ParseError("Missing transcript in response".to_string()))?;
    let paragraphs_data = transcript_obj
        .get("paragraphs")
        .and_then(|p| p.as_array())
        .ok_or_else(|| YahooError::ParseError("Missing paragraphs in response".to_string()))?;

    let mut paragraphs: Vec<TranscriptParagraph> = Vec::new();
    for para in paragraphs_data {
        if let Some(para_obj) = para.as_object() {
            let speaker_id = para_obj
                .get("speaker")
                .and_then(|s| s.as_str())
                .unwrap_or("Unknown");
            let speaker_name = speaker_mapping
                .get(speaker_id)
                .cloned()
                .unwrap_or_else(|| format!("Speaker {}", speaker_id));
            let text = para_obj
                .get("text")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();

            if !text.is_empty() {
                paragraphs.push(TranscriptParagraph {
                    speaker: speaker_name,
                    text,
                });
            }
        }
    }

    // Extract metadata
    let fiscal_year = metadata
        .get("fiscalYear")
        .and_then(|fy| fy.as_i64())
        .map(|y| y as i32)
        .or(call_info.year)
        .ok_or_else(|| YahooError::ParseError("Missing fiscal year".to_string()))?;
    let fiscal_period = metadata
        .get("fiscalPeriod")
        .and_then(|fp| fp.as_str())
        .map(|s| s.to_string())
        .or_else(|| call_info.quarter.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    let title = metadata
        .get("title")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| call_info.title.clone());

    // Parse date
    let date = if let Some(date_timestamp) = metadata.get("date").and_then(|d| d.as_i64()) {
        DateTime::from_timestamp(date_timestamp, 0)
            .ok_or_else(|| YahooError::ParseError("Invalid date timestamp".to_string()))?
    } else {
        Utc::now()
    };

    // Build metadata dict
    let mut meta_dict = HashMap::new();
    meta_dict.insert("eventId".to_string(), Value::String(call_info.event_id.clone()));
    meta_dict.insert("fiscalYear".to_string(), Value::Number(fiscal_year.into()));
    meta_dict.insert("fiscalPeriod".to_string(), Value::String(fiscal_period.clone()));
    if let Some(transcript_id) = metadata.get("transcriptId") {
        meta_dict.insert("transcriptId".to_string(), transcript_id.clone());
    }
    meta_dict.insert(
        "eventType".to_string(),
        Value::String(
            metadata
                .get("eventType")
                .and_then(|et| et.as_str())
                .unwrap_or("Earnings Call")
                .to_string(),
        ),
    );
    meta_dict.insert(
        "isLatest".to_string(),
        Value::Bool(metadata.get("isLatest").and_then(|il| il.as_bool()).unwrap_or(false)),
    );
    meta_dict.insert(
        "retrieved_at".to_string(),
        Value::String(Utc::now().to_rfc3339()),
    );

    Ok(EarningsTranscript {
        symbol: symbol.to_uppercase(),
        quarter: fiscal_period,
        year: fiscal_year,
        date,
        title,
        speakers: speakers_list,
        paragraphs,
        metadata: meta_dict,
    })
}