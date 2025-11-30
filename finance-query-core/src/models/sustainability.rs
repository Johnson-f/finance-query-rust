use serde::{Deserialize, Serialize};

/// ESG (Environmental, Social, Governance) sustainability scores
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SustainabilityScores {
    pub symbol: String,
    /// Total ESG score
    pub total_esg: Option<f64>,
    /// Environmental score
    pub environment_score: Option<f64>,
    /// Social score
    pub social_score: Option<f64>,
    /// Governance score
    pub governance_score: Option<f64>,
    /// Controversy level (0-5, higher is worse)
    pub controversy_level: Option<u8>,
    /// ESG percentile rank
    pub percentile: Option<f64>,
    /// Peer group
    pub peer_group: Option<String>,
    /// ESG performance relative to peers
    pub peer_esg_score_performance: Option<String>,
    /// Related controversy topics
    pub related_controversy: Option<Vec<String>>,
}

impl SustainabilityScores {
    pub(crate) fn from_yahoo_response(
        symbol: String,
        response: YahooEsgResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let result = response
            .quote_summary
            .result
            .first()
            .ok_or_else(|| {
                crate::client::YahooError::ParseError("No ESG data in response".to_string())
            })?;

        let esg = &result.esg_scores;

        Ok(Self {
            symbol,
            total_esg: esg.total_esg.as_ref().and_then(|v| v.raw),
            environment_score: esg.environment_score.as_ref().and_then(|v| v.raw),
            social_score: esg.social_score.as_ref().and_then(|v| v.raw),
            governance_score: esg.governance_score.as_ref().and_then(|v| v.raw),
            controversy_level: esg.highest_controversy.and_then(|v| u8::try_from(v).ok()),
            percentile: esg.percentile.as_ref().and_then(|v| v.raw),
            peer_group: esg.peer_group.clone(),
            peer_esg_score_performance: esg
                .peer_esg_score_performance
                .as_ref()
                .and_then(|v| v.raw.clone()),
            related_controversy: esg.related_controversy.clone(),
        })
    }

    /// Check if ESG data is available
    pub fn has_data(&self) -> bool {
        self.total_esg.is_some()
    }

    /// Get a simple ESG rating (A, B, C, D, F based on total score)
    pub fn rating(&self) -> Option<&'static str> {
        self.total_esg.map(|score| {
            if score >= 70.0 {
                "A"
            } else if score >= 50.0 {
                "B"
            } else if score >= 30.0 {
                "C"
            } else if score >= 20.0 {
                "D"
            } else {
                "F"
            }
        })
    }
}

// Internal Yahoo response structures
#[derive(Debug, Deserialize)]
pub(crate) struct YahooEsgResponse {
    #[serde(rename = "quoteSummary")]
    pub quote_summary: EsgQuoteSummaryData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EsgQuoteSummaryData {
    pub result: Vec<EsgQuoteSummaryResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EsgQuoteSummaryResult {
    #[serde(rename = "esgScores")]
    pub esg_scores: EsgScoresData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EsgScoresData {
    #[serde(rename = "totalEsg")]
    pub total_esg: Option<YahooNumericValue>,
    #[serde(rename = "environmentScore")]
    pub environment_score: Option<YahooNumericValue>,
    #[serde(rename = "socialScore")]
    pub social_score: Option<YahooNumericValue>,
    #[serde(rename = "governanceScore")]
    pub governance_score: Option<YahooNumericValue>,
    #[serde(rename = "highestControversy")]
    pub highest_controversy: Option<i32>,
    pub percentile: Option<YahooNumericValue>,
    #[serde(rename = "peerGroup")]
    pub peer_group: Option<String>,
    #[serde(rename = "peerEsgScorePerformance")]
    pub peer_esg_score_performance: Option<YahooStringValue>,
    #[serde(rename = "relatedControversy")]
    pub related_controversy: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooNumericValue {
    pub raw: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooStringValue {
    pub raw: Option<String>,
}
