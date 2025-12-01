use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum IndicatorType {
    SMA,
    EMA,
}

impl IndicatorType {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sma" => Some(IndicatorType::SMA),
            "ema" => Some(IndicatorType::EMA),
            _ => None,
        }
    }
    
    pub fn parse_list(s: &str) -> HashSet<Self> {
        s.split(',')
            .map(|s| s.trim())
            .filter_map(IndicatorType::parse)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    #[serde(rename = "1d")]
    Day,
    #[serde(rename = "5d")]
    FiveDays,
    #[serde(rename = "1mo")]
    OneMonth,
    #[serde(rename = "3mo")]
    ThreeMonths,
    #[serde(rename = "6mo")]
    SixMonths,
    #[serde(rename = "1y")]
    Year,
    #[serde(rename = "2y")]
    TwoYears,
    #[serde(rename = "5y")]
    FiveYears,
    #[serde(rename = "10y")]
    TenYears,
    #[serde(rename = "ytd")]
    Ytd,
    #[serde(rename = "max")]
    Max,
}

impl TimeRange {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeRange::Day => "1d",
            TimeRange::FiveDays => "5d",
            TimeRange::OneMonth => "1mo",
            TimeRange::ThreeMonths => "3mo",
            TimeRange::SixMonths => "6mo",
            TimeRange::Year => "1y",
            TimeRange::TwoYears => "2y",
            TimeRange::FiveYears => "5y",
            TimeRange::TenYears => "10y",
            TimeRange::Ytd => "ytd",
            TimeRange::Max => "max",
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Interval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "10m")]
    TenMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "20m")]
    TwentyMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "65m")]
    SixtyFiveMinutes,
    #[serde(rename = "95m")]
    NinetyFiveMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "1d")]
    Daily,
    #[serde(rename = "1wk")]
    Weekly,
    #[serde(rename = "1mo")]
    Monthly,
}

impl Interval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Interval::OneMinute => "1m",
            Interval::ThreeMinutes => "3m",
            Interval::FiveMinutes => "5m",
            Interval::TenMinutes => "10m",
            Interval::FifteenMinutes => "15m",
            Interval::TwentyMinutes => "20m",
            Interval::ThirtyMinutes => "30m",
            Interval::SixtyFiveMinutes => "65m",
            Interval::NinetyFiveMinutes => "95m",
            Interval::OneHour => "1h",
            Interval::Daily => "1d",
            Interval::Weekly => "1wk",
            Interval::Monthly => "1mo",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HistoricalData {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adj_close: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sma: Option<std::collections::HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ema: Option<std::collections::HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HistoricalResponse {
    pub data: std::collections::HashMap<String, HistoricalData>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Helper function to compare f64 values with tolerance for JSON round-trip
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    fn approx_eq_opt(a: Option<f64>, b: Option<f64>) -> bool {
        match (a, b) {
            (Some(x), Some(y)) => approx_eq(x, y),
            (None, None) => true,
            _ => false,
        }
    }

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn historical_data_roundtrip(
            open in 0.01f64..10000.0f64,
            high in 0.01f64..10000.0f64,
            low in 0.01f64..10000.0f64,
            close in 0.01f64..10000.0f64,
            volume in 0i64..1_000_000_000i64,
            adj_close in proptest::option::of(0.01f64..10000.0f64),
        ) {
            let data = HistoricalData {
                open,
                high,
                low,
                close,
                volume,
                adj_close,
                sma: None,
                ema: None,
            };

            let json = serde_json::to_string(&data).unwrap();
            let parsed: HistoricalData = serde_json::from_str(&json).unwrap();

            // Use approximate comparison for f64 due to JSON serialization precision limits
            prop_assert!(approx_eq(data.open, parsed.open), "open mismatch");
            prop_assert!(approx_eq(data.high, parsed.high), "high mismatch");
            prop_assert!(approx_eq(data.low, parsed.low), "low mismatch");
            prop_assert!(approx_eq(data.close, parsed.close), "close mismatch");
            prop_assert_eq!(data.volume, parsed.volume);
            prop_assert!(approx_eq_opt(data.adj_close, parsed.adj_close), "adj_close mismatch");
        }

        #[test]
        fn time_range_roundtrip(range in prop_oneof![
            Just(TimeRange::Day),
            Just(TimeRange::FiveDays),
            Just(TimeRange::OneMonth),
            Just(TimeRange::ThreeMonths),
            Just(TimeRange::SixMonths),
            Just(TimeRange::Year),
            Just(TimeRange::TwoYears),
            Just(TimeRange::FiveYears),
            Just(TimeRange::TenYears),
            Just(TimeRange::Ytd),
            Just(TimeRange::Max),
        ]) {
            let json = serde_json::to_string(&range).unwrap();
            let parsed: TimeRange = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(range.as_str(), parsed.as_str());
        }

        #[test]
        fn interval_roundtrip(interval in prop_oneof![
            Just(Interval::OneMinute),
            Just(Interval::ThreeMinutes),
            Just(Interval::FiveMinutes),
            Just(Interval::TenMinutes),
            Just(Interval::FifteenMinutes),
            Just(Interval::TwentyMinutes),
            Just(Interval::ThirtyMinutes),
            Just(Interval::SixtyFiveMinutes),
            Just(Interval::NinetyFiveMinutes),
            Just(Interval::OneHour),
            Just(Interval::Daily),
            Just(Interval::Weekly),
            Just(Interval::Monthly),
        ]) {
            let json = serde_json::to_string(&interval).unwrap();
            let parsed: Interval = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(interval.as_str(), parsed.as_str());
        }
    }
}
