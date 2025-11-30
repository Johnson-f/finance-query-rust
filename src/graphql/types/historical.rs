use async_graphql::*;
use std::collections::HashMap;
use finance_query_core::models::historical::{
    HistoricalData as HistoricalDataModel,
    HistoricalResponse as HistoricalResponseModel,
    TimeRange as TimeRangeModel,
    Interval as IntervalModel,
    IndicatorType as IndicatorTypeModel,
};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TimeRange {
    #[graphql(name = "1d")]
    Day,
    #[graphql(name = "5d")]
    FiveDays,
    #[graphql(name = "1mo")]
    OneMonth,
    #[graphql(name = "3mo")]
    ThreeMonths,
    #[graphql(name = "6mo")]
    SixMonths,
    #[graphql(name = "1y")]
    Year,
    #[graphql(name = "2y")]
    TwoYears,
    #[graphql(name = "5y")]
    FiveYears,
    #[graphql(name = "10y")]
    TenYears,
    #[graphql(name = "ytd")]
    Ytd,
    #[graphql(name = "max")]
    Max,
}

impl From<TimeRangeModel> for TimeRange {
    fn from(range: TimeRangeModel) -> Self {
        match range {
            TimeRangeModel::Day => TimeRange::Day,
            TimeRangeModel::FiveDays => TimeRange::FiveDays,
            TimeRangeModel::OneMonth => TimeRange::OneMonth,
            TimeRangeModel::ThreeMonths => TimeRange::ThreeMonths,
            TimeRangeModel::SixMonths => TimeRange::SixMonths,
            TimeRangeModel::Year => TimeRange::Year,
            TimeRangeModel::TwoYears => TimeRange::TwoYears,
            TimeRangeModel::FiveYears => TimeRange::FiveYears,
            TimeRangeModel::TenYears => TimeRange::TenYears,
            TimeRangeModel::Ytd => TimeRange::Ytd,
            TimeRangeModel::Max => TimeRange::Max,
        }
    }
}

impl From<TimeRange> for TimeRangeModel {
    fn from(range: TimeRange) -> Self {
        match range {
            TimeRange::Day => TimeRangeModel::Day,
            TimeRange::FiveDays => TimeRangeModel::FiveDays,
            TimeRange::OneMonth => TimeRangeModel::OneMonth,
            TimeRange::ThreeMonths => TimeRangeModel::ThreeMonths,
            TimeRange::SixMonths => TimeRangeModel::SixMonths,
            TimeRange::Year => TimeRangeModel::Year,
            TimeRange::TwoYears => TimeRangeModel::TwoYears,
            TimeRange::FiveYears => TimeRangeModel::FiveYears,
            TimeRange::TenYears => TimeRangeModel::TenYears,
            TimeRange::Ytd => TimeRangeModel::Ytd,
            TimeRange::Max => TimeRangeModel::Max,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Interval {
    #[graphql(name = "1m")]
    OneMinute,
    #[graphql(name = "3m")]
    ThreeMinutes,
    #[graphql(name = "5m")]
    FiveMinutes,
    #[graphql(name = "10m")]
    TenMinutes,
    #[graphql(name = "15m")]
    FifteenMinutes,
    #[graphql(name = "20m")]
    TwentyMinutes,
    #[graphql(name = "30m")]
    ThirtyMinutes,
    #[graphql(name = "65m")]
    SixtyFiveMinutes,
    #[graphql(name = "95m")]
    NinetyFiveMinutes,
    #[graphql(name = "1h")]
    OneHour,
    #[graphql(name = "1d")]
    Daily,
    #[graphql(name = "1wk")]
    Weekly,
    #[graphql(name = "1mo")]
    Monthly,
}

impl From<IntervalModel> for Interval {
    fn from(interval: IntervalModel) -> Self {
        match interval {
            IntervalModel::OneMinute => Interval::OneMinute,
            IntervalModel::ThreeMinutes => Interval::ThreeMinutes,
            IntervalModel::FiveMinutes => Interval::FiveMinutes,
            IntervalModel::TenMinutes => Interval::TenMinutes,
            IntervalModel::FifteenMinutes => Interval::FifteenMinutes,
            IntervalModel::TwentyMinutes => Interval::TwentyMinutes,
            IntervalModel::ThirtyMinutes => Interval::ThirtyMinutes,
            IntervalModel::SixtyFiveMinutes => Interval::SixtyFiveMinutes,
            IntervalModel::NinetyFiveMinutes => Interval::NinetyFiveMinutes,
            IntervalModel::OneHour => Interval::OneHour,
            IntervalModel::Daily => Interval::Daily,
            IntervalModel::Weekly => Interval::Weekly,
            IntervalModel::Monthly => Interval::Monthly,
        }
    }
}

impl From<Interval> for IntervalModel {
    fn from(interval: Interval) -> Self {
        match interval {
            Interval::OneMinute => IntervalModel::OneMinute,
            Interval::ThreeMinutes => IntervalModel::ThreeMinutes,
            Interval::FiveMinutes => IntervalModel::FiveMinutes,
            Interval::TenMinutes => IntervalModel::TenMinutes,
            Interval::FifteenMinutes => IntervalModel::FifteenMinutes,
            Interval::TwentyMinutes => IntervalModel::TwentyMinutes,
            Interval::ThirtyMinutes => IntervalModel::ThirtyMinutes,
            Interval::SixtyFiveMinutes => IntervalModel::SixtyFiveMinutes,
            Interval::NinetyFiveMinutes => IntervalModel::NinetyFiveMinutes,
            Interval::OneHour => IntervalModel::OneHour,
            Interval::Daily => IntervalModel::Daily,
            Interval::Weekly => IntervalModel::Weekly,
            Interval::Monthly => IntervalModel::Monthly,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum IndicatorType {
    #[graphql(name = "sma")]
    SMA,
    #[graphql(name = "ema")]
    EMA,
}

impl From<IndicatorTypeModel> for IndicatorType {
    fn from(indicator: IndicatorTypeModel) -> Self {
        match indicator {
            IndicatorTypeModel::SMA => IndicatorType::SMA,
            IndicatorTypeModel::EMA => IndicatorType::EMA,
        }
    }
}

impl From<IndicatorType> for IndicatorTypeModel {
    fn from(indicator: IndicatorType) -> Self {
        match indicator {
            IndicatorType::SMA => IndicatorTypeModel::SMA,
            IndicatorType::EMA => IndicatorTypeModel::EMA,
        }
    }
}

#[derive(SimpleObject, Clone, serde::Serialize)]
pub struct HistoricalData {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub adj_close: Option<f64>,
    pub sma: Option<HashMap<String, f64>>,
    pub ema: Option<HashMap<String, f64>>,
}

impl From<HistoricalDataModel> for HistoricalData {
    fn from(data: HistoricalDataModel) -> Self {
        HistoricalData {
            open: data.open,
            high: data.high,
            low: data.low,
            close: data.close,
            volume: data.volume,
            adj_close: data.adj_close,
            sma: data.sma,
            ema: data.ema,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct HistoricalResponse {
    pub data: HashMap<String, HistoricalData>,
}

impl From<HistoricalResponseModel> for HistoricalResponse {
    fn from(response: HistoricalResponseModel) -> Self {
        HistoricalResponse {
            data: response.data.into_iter()
                .map(|(k, v)| (k, HistoricalData::from(v)))
                .collect(),
        }
    }
}