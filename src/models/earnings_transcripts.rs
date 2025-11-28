use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

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