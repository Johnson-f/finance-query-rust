use serde::{Deserialize, Serialize};

/// Represents an industry with its companies and performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Industry {
    pub key: String,
    pub name: String,
    pub sector_key: Option<String>,
    pub sector_name: Option<String>,
    pub description: Option<String>,
    pub top_performing_companies: Vec<IndustryCompany>,
    pub top_growth_companies: Vec<IndustryCompany>,
}

/// A company within an industry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryCompany {
    pub symbol: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_return: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_estimate: Option<f64>,
}

impl Industry {
    pub(crate) fn from_yahoo_response(
        response: YahooIndustryResponse,
    ) -> Result<Self, crate::client::YahooError> {
        let data = response.data;

        let top_performing = data
            .top_performing_companies
            .unwrap_or_default()
            .into_iter()
            .map(|c| IndustryCompany {
                symbol: c.symbol.unwrap_or_default(),
                name: c.name.unwrap_or_default(),
                ytd_return: c.ytd_return.and_then(|v| v.raw),
                last_price: c.last_price.and_then(|v| v.raw),
                target_price: c.target_price.and_then(|v| v.raw),
                growth_estimate: None,
            })
            .collect();

        let top_growth = data
            .top_growth_companies
            .unwrap_or_default()
            .into_iter()
            .map(|c| IndustryCompany {
                symbol: c.symbol.unwrap_or_default(),
                name: c.name.unwrap_or_default(),
                ytd_return: c.ytd_return.and_then(|v| v.raw),
                last_price: None,
                target_price: None,
                growth_estimate: c.growth_estimate.and_then(|v| v.raw),
            })
            .collect();

        Ok(Self {
            key: data.key.unwrap_or_default(),
            name: data.name.unwrap_or_default(),
            sector_key: data.sector_key,
            sector_name: data.sector_name,
            description: data.description,
            top_performing_companies: top_performing,
            top_growth_companies: top_growth,
        })
    }
}

// Internal Yahoo response structures
#[derive(Debug, Deserialize)]
pub(crate) struct YahooIndustryResponse {
    pub data: IndustryData,
}

#[derive(Debug, Deserialize)]
pub(crate) struct IndustryData {
    pub key: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "sectorKey")]
    pub sector_key: Option<String>,
    #[serde(rename = "sectorName")]
    pub sector_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "topPerformingCompanies")]
    pub top_performing_companies: Option<Vec<YahooIndustryCompany>>,
    #[serde(rename = "topGrowthCompanies")]
    pub top_growth_companies: Option<Vec<YahooIndustryCompany>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooIndustryCompany {
    pub symbol: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "ytdReturn")]
    pub ytd_return: Option<YahooNumericValue>,
    #[serde(rename = "lastPrice")]
    pub last_price: Option<YahooNumericValue>,
    #[serde(rename = "targetPrice")]
    pub target_price: Option<YahooNumericValue>,
    #[serde(rename = "growthEstimate")]
    pub growth_estimate: Option<YahooNumericValue>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct YahooNumericValue {
    pub raw: Option<f64>,
}
