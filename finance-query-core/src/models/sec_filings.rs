use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Represents an SEC filing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecFiling {
    pub date: DateTime<Utc>,
    pub filing_type: String,
    pub title: String,
    pub url: String,
    pub exhibits: Vec<SecExhibit>,
}

/// Represents an exhibit within an SEC filing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecExhibit {
    pub exhibit_type: String,
    pub url: String,
}

/// Response containing SEC filings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecFilingsResponse {
    pub symbol: String,
    pub filings: Vec<SecFiling>,
}

impl SecFilingsResponse {
    pub(crate) fn from_yahoo_response(
        symbol: String,
        response: YahooSecFilingsResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let result = response.quote_summary.result.first().ok_or_else(|| {
            crate::client::YahooError::ParseError("No SEC filings data in response".to_string())
        })?;

        let filings = result
            .sec_filings
            .filings
            .iter()
            .filter_map(|f| {
                // Parse date - can be string "YYYY-MM-DD" or timestamp
                let date = parse_sec_date(&f.date)?;

                let exhibits = f
                    .exhibits
                    .as_ref()
                    .map(|exs| {
                        exs.iter()
                            .map(|e| SecExhibit {
                                exhibit_type: e.exhibit_type.clone().unwrap_or_default(),
                                url: e.url.clone().unwrap_or_default(),
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                Some(SecFiling {
                    date,
                    filing_type: f.filing_type.clone().unwrap_or_default(),
                    title: f.title.clone().unwrap_or_default(),
                    url: f.edgar_url.clone().unwrap_or_default(),
                    exhibits,
                })
            })
            .collect();

        Ok(Self { symbol, filings })
    }
}

/// Parse SEC filing date which can be either a string or timestamp
fn parse_sec_date(date_value: &SecDateValue) -> Option<DateTime<Utc>> {
    match date_value {
        SecDateValue::String(s) => {
            // Parse "YYYY-MM-DD" format
            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .ok()
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .and_then(|dt| dt.and_local_timezone(Utc).single())
        }
        SecDateValue::Timestamp(ts) => Utc.timestamp_opt(*ts, 0).single(),
        SecDateValue::Object { raw, .. } => raw.and_then(|ts| Utc.timestamp_opt(ts, 0).single()),
    }
}

// Internal Yahoo response structures
#[derive(Debug, Deserialize)]
pub(crate) struct YahooSecFilingsResponse {
    #[serde(rename = "quoteSummary")]
    pub quote_summary: SecQuoteSummaryData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SecQuoteSummaryData {
    pub result: Vec<SecQuoteSummaryResult>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SecQuoteSummaryResult {
    #[serde(rename = "secFilings")]
    pub sec_filings: SecFilingsData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SecFilingsData {
    pub filings: Vec<YahooSecFiling>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooSecFiling {
    pub date: SecDateValue,
    #[serde(rename = "type")]
    pub filing_type: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "edgarUrl")]
    pub edgar_url: Option<String>,
    pub exhibits: Option<Vec<YahooExhibit>>,
}

/// SEC date can be a string, timestamp, or object with raw field
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum SecDateValue {
    String(String),
    Timestamp(i64),
    Object {
        raw: Option<i64>,
        #[allow(dead_code)]
        fmt: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooExhibit {
    #[serde(rename = "type")]
    pub exhibit_type: Option<String>,
    pub url: Option<String>,
}
