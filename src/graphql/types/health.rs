use async_graphql::*;
use chrono::{DateTime, Utc};

#[derive(SimpleObject, Clone)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub services: HealthServices,
}

#[derive(SimpleObject, Clone)]
pub struct HealthServices {
    pub status: String,
}