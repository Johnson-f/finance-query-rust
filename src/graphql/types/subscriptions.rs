use async_graphql::*;
use chrono::{DateTime, Utc};

#[derive(SimpleObject, Clone)]
pub struct ProfileUpdate {
    pub quote: Option<super::quote::Quote>,
    pub similar: Option<Vec<super::quote::SimpleQuote>>,
    pub sector_performance: Option<super::sectors::MarketSector>,
    pub news: Option<Vec<super::news::News>>,
}

#[derive(SimpleObject, Clone)]
pub struct MoversUpdate {
    pub actives: Option<Vec<super::movers::MarketMover>>,
    pub gainers: Option<Vec<super::movers::MarketMover>>,
    pub losers: Option<Vec<super::movers::MarketMover>>,
}

#[derive(SimpleObject, Clone)]
pub struct MarketHours {
    pub status: String,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(SimpleObject, Clone)]
pub struct MovingAverageUpdate {
    pub symbol: String,
    pub indicator_type: String,
    pub period: i32,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}