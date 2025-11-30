use async_graphql::*;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use finance_query_core::models::earnings_transcripts::{
    EarningsCallListing as EarningsCallListingModel,
    EarningsCallsList as EarningsCallsListModel,
    TranscriptSpeaker as TranscriptSpeakerModel,
    TranscriptParagraph as TranscriptParagraphModel,
    EarningsTranscript as EarningsTranscriptModel,
};

#[derive(SimpleObject, Clone)]
pub struct EarningsCallListing {
    pub event_id: String,
    pub quarter: Option<String>,
    pub year: Option<i32>,
    pub title: String,
    pub url: String,
}

impl From<EarningsCallListingModel> for EarningsCallListing {
    fn from(listing: EarningsCallListingModel) -> Self {
        EarningsCallListing {
            event_id: listing.event_id,
            quarter: listing.quarter,
            year: listing.year,
            title: listing.title,
            url: listing.url,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct EarningsCallsList {
    pub symbol: String,
    pub earnings_calls: Vec<EarningsCallListing>,
    pub total: usize,
}

impl From<EarningsCallsListModel> for EarningsCallsList {
    fn from(list: EarningsCallsListModel) -> Self {
        EarningsCallsList {
            symbol: list.symbol,
            earnings_calls: list.earnings_calls.into_iter().map(EarningsCallListing::from).collect(),
            total: list.total,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct TranscriptSpeaker {
    pub name: String,
    pub role: Option<String>,
    pub company: Option<String>,
}

impl From<TranscriptSpeakerModel> for TranscriptSpeaker {
    fn from(speaker: TranscriptSpeakerModel) -> Self {
        TranscriptSpeaker {
            name: speaker.name,
            role: speaker.role,
            company: speaker.company,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct TranscriptParagraph {
    pub speaker: String,
    pub text: String,
}

impl From<TranscriptParagraphModel> for TranscriptParagraph {
    fn from(para: TranscriptParagraphModel) -> Self {
        TranscriptParagraph {
            speaker: para.speaker,
            text: para.text,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct EarningsTranscript {
    pub symbol: String,
    pub quarter: String,
    pub year: i32,
    pub date: DateTime<Utc>,
    pub title: String,
    pub speakers: Vec<TranscriptSpeaker>,
    pub paragraphs: Vec<TranscriptParagraph>,
    pub metadata: HashMap<String, async_graphql::Json<serde_json::Value>>,
}

impl From<EarningsTranscriptModel> for EarningsTranscript {
    fn from(transcript: EarningsTranscriptModel) -> Self {
        EarningsTranscript {
            symbol: transcript.symbol,
            quarter: transcript.quarter,
            year: transcript.year,
            date: transcript.date,
            title: transcript.title,
            speakers: transcript.speakers.into_iter().map(TranscriptSpeaker::from).collect(),
            paragraphs: transcript.paragraphs.into_iter().map(TranscriptParagraph::from).collect(),
            metadata: transcript.metadata.into_iter()
                .map(|(k, v)| (k, async_graphql::Json(v)))
                .collect(),
        }
    }
}