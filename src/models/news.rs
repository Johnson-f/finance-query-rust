use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct News {
    pub title: String,
    pub link: String,
    pub source: String,
    pub img: String,
    pub time: String,
}

