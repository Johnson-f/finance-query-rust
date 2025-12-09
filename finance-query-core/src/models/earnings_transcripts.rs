use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(dead_code)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EarningsCallListing {
    pub event_id: String,
    pub quarter: Option<String>,
    pub year: Option<i32>,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EarningsCallsList {
    pub symbol: String,
    pub earnings_calls: Vec<EarningsCallListing>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranscriptSpeaker {
    pub name: String,
    pub role: Option<String>,
    pub company: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranscriptParagraph {
    pub speaker: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn optional_string() -> impl Strategy<Value = Option<String>> {
        proptest::option::of("[A-Za-z ]{1,30}")
    }

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn transcript_speaker_roundtrip(
            name in "[A-Za-z ]{1,30}",
            role in optional_string(),
            company in optional_string(),
        ) {
            let speaker = TranscriptSpeaker {
                name: name.clone(),
                role: role.clone(),
                company: company.clone(),
            };

            let json = serde_json::to_string(&speaker).unwrap();
            let parsed: TranscriptSpeaker = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(speaker.name, parsed.name);
            prop_assert_eq!(speaker.role, parsed.role);
            prop_assert_eq!(speaker.company, parsed.company);
        }

        #[test]
        fn transcript_paragraph_roundtrip(
            speaker in "[A-Za-z ]{1,30}",
            text in "[A-Za-z0-9 .,!?]{1,100}",
        ) {
            let para = TranscriptParagraph {
                speaker: speaker.clone(),
                text: text.clone(),
            };

            let json = serde_json::to_string(&para).unwrap();
            let parsed: TranscriptParagraph = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(para.speaker, parsed.speaker);
            prop_assert_eq!(para.text, parsed.text);
        }

        #[test]
        fn earnings_call_listing_roundtrip(
            event_id in "[a-z0-9]{8}",
            quarter in optional_string(),
            year in proptest::option::of(2000i32..2030i32),
            title in "[A-Za-z0-9 ]{1,50}",
            url in "https://[a-z]{5,10}\\.com/[a-z]{5,10}",
        ) {
            let listing = EarningsCallListing {
                event_id: event_id.clone(),
                quarter: quarter.clone(),
                year,
                title: title.clone(),
                url: url.clone(),
            };

            let json = serde_json::to_string(&listing).unwrap();
            let parsed: EarningsCallListing = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(listing.event_id, parsed.event_id);
            prop_assert_eq!(listing.quarter, parsed.quarter);
            prop_assert_eq!(listing.year, parsed.year);
            prop_assert_eq!(listing.title, parsed.title);
            prop_assert_eq!(listing.url, parsed.url);
        }
    }
}
