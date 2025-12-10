use crate::service::quotes;
use finance_query_core::client::{FetchClient, YahooFinanceClient, error::YahooError};
use finance_query_core::models::indices::{Index, MarketIndex, Region, get_index_regions};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Get Yahoo Finance symbol for an index
fn get_yahoo_index_symbol(index: Index) -> String {
    // Special case mapping from Index enum to actual Yahoo Finance symbols
    match index {
        Index::MoexMe => "MOEX.ME".to_string(),
        Index::DxYNyb => "DX-Y.NYB".to_string(),
        Index::UsdStrd => "^125904-USD-STRD".to_string(),
        Index::MsciWorld => "^990100-USD-STRD".to_string(),
        Index::Shanghai => "000001.SS".to_string(),
        Index::Szse => "399001.SZ".to_string(),
        Index::Psi => "PSI20.LS".to_string(),
        Index::Bux => "^BUX.BD".to_string(),
        Index::Bist100 => "XU100.IS".to_string(),
        Index::Ta35 => "TA35.TA".to_string(),
        Index::Tasi => "^TASI.SR".to_string(),
        Index::Set => "^SET.BK".to_string(),
        Index::Psei => "PSEI.PS".to_string(),
        Index::Imoex => "IMOEX.ME".to_string(),
        Index::Rtsi => "RTSI.ME".to_string(),
        Index::ChinaA50 => "XIN9.FGI".to_string(),
        Index::Wig20 => "WIG20.WA".to_string(),
        Index::Ftsemib => "FTSEMIB.MI".to_string(),
        Index::Ftsejse => "^J580.JO".to_string(),
        Index::Afr40 => "^JA0R.JO".to_string(),
        Index::Sa40 => "^J200.JO".to_string(),
        Index::Raf40 => "^J260.JO".to_string(),
        Index::Alt15 => "^J233.JO".to_string(),
        Index::Tamayuz => "^TAMAYUZ.CA".to_string(),
        Index::Ivbx => "^IVBX".to_string(),
        Index::Ibrx50 => "^IBX50".to_string(),
        // Default format: ^INDEX_NAME
        Index::Gspc => "^GSPC".to_string(),
        Index::Dji => "^DJI".to_string(),
        Index::Ixic => "^IXIC".to_string(),
        Index::Nya => "^NYA".to_string(),
        Index::Xax => "^XAX".to_string(),
        Index::Rut => "^RUT".to_string(),
        Index::Vix => "^VIX".to_string(),
        Index::Gsptse => "^GSPTSE".to_string(),
        Index::Bvsp => "^BVSP".to_string(),
        Index::Mxx => "^MXX".to_string(),
        Index::Ipsa => "^IPSA".to_string(),
        Index::Merv => "^MERV".to_string(),
        Index::Ftse => "^FTSE".to_string(),
        Index::Gdaxi => "^GDAXI".to_string(),
        Index::Fchi => "^FCHI".to_string(),
        Index::Stoxx50e => "^STOXX50E".to_string(),
        Index::N100 => "^N100".to_string(),
        Index::Bfx => "^BFX".to_string(),
        Index::Aex => "^AEX".to_string(),
        Index::Ibex => "^IBEX".to_string(),
        Index::Ssmi => "^SSMI".to_string(),
        Index::Atx => "^ATX".to_string(),
        Index::Omxs30 => "^OMXS30".to_string(),
        Index::Omxc25 => "^OMXC25".to_string(),
        Index::Hsi => "^HSI".to_string(),
        Index::Sti => "^STI".to_string(),
        Index::Bsesn => "^BSESN".to_string(),
        Index::Jkse => "^JKSE".to_string(),
        Index::Klse => "^KLSE".to_string(),
        Index::Ks11 => "^KS11".to_string(),
        Index::Twii => "^TWII".to_string(),
        Index::N225 => "^N225".to_string(),
        Index::Nsei => "^NSEI".to_string(),
        Index::Cnx200 => "^CNX200".to_string(),
        Index::Djsh => "^DJSH".to_string(),
        Index::Indiavix => "^INDIAVIX".to_string(),
        Index::Case30 => "^CASE30".to_string(),
        Index::Jn0uJo => "^JN0U.JO".to_string(),
        Index::Ta125Ta => "^TA125.TA".to_string(),
        Index::Axjo => "^AXJO".to_string(),
        Index::Aord => "^AORD".to_string(),
        Index::Nz50 => "^NZ50".to_string(),
        Index::Xdb => "^XDB".to_string(),
        Index::Xde => "^XDE".to_string(),
        Index::Xdn => "^XDN".to_string(),
        Index::Xda => "^XDA".to_string(),
        Index::Buk100p => "^BUK100P".to_string(),
    }
}

/// Get formatted index name
fn get_formatted_index_name(index: Index, default_name: String) -> String {
    match index {
        Index::Gdaxi => "DAX Performance Index".to_string(),
        Index::Stoxx50e => "EURO STOXX 50".to_string(),
        Index::Nz50 => "S&P/NZX 50 Index".to_string(),
        Index::Set => "Thailand SET Index".to_string(),
        Index::Jn0uJo => "FTSE JSE Top 40- USD Net TRI".to_string(),
        Index::Sa40 => "South Africa Top 40".to_string(),
        _ => default_name,
    }
}

/// Helper function to extract formatted value from Yahoo API response
fn get_fmt(data: &Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| {
        if let Some(obj) = v.as_object() {
            obj.get("fmt").or_else(|| obj.get("raw")).and_then(|val| {
                val.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| val.as_f64().map(|f| f.to_string()))
            })
        } else if let Some(num) = v.as_f64() {
            Some(num.to_string())
        } else {
            v.as_str().map(|str_val| str_val.to_string())
        }
    })
}

/// Helper function to format return values with plus sign for positives
fn format_return(value: Option<&Value>) -> Option<String> {
    value.and_then(|v| {
        let fmt = if let Some(obj) = v.as_object() {
            obj.get("fmt")
                .and_then(|f| f.as_str().map(|s| s.to_string()))
        } else {
            v.as_str().map(|str_val| str_val.to_string())
        };

        fmt.map(|f| {
            if !f.starts_with('-') && f != "0.00%" {
                format!("+{}", f)
            } else {
                f
            }
        })
    })
}

/// Parse Yahoo Finance API response into MarketIndex
async fn parse_yahoo_index(summary_data: Value, index: Index) -> Result<MarketIndex, YahooError> {
    let summary_result = summary_data
        .get("quoteSummary")
        .and_then(|qs| qs.get("result"))
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .ok_or_else(|| YahooError::ParseError("No quoteSummary result found".to_string()))?;

    let price_data = summary_result.get("price").unwrap_or(&Value::Null);
    let performance_data = summary_result
        .get("quoteUnadjustedPerformanceOverview")
        .and_then(|q| q.get("performanceOverview"))
        .unwrap_or(&Value::Null);

    // Get name
    let default_name = price_data
        .get("longName")
        .or_else(|| price_data.get("shortName"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| index.as_str().to_string());
    let formatted_name = get_formatted_index_name(index, default_name);

    // Get value (round to 2 decimal places)
    let value = price_data
        .get("regularMarketPrice")
        .and_then(|p| {
            if let Some(obj) = p.as_object() {
                obj.get("raw").and_then(|r| r.as_f64())
            } else {
                p.as_f64()
            }
        })
        .map(|v| (v * 100.0).round() / 100.0)
        .unwrap_or(0.0);

    Ok(MarketIndex {
        name: formatted_name,
        value,
        change: get_fmt(price_data, "regularMarketChange").unwrap_or_else(|| "0.0".to_string()),
        percent_change: get_fmt(price_data, "regularMarketChangePercent")
            .unwrap_or_else(|| "0.0%".to_string()),
        five_days_return: format_return(performance_data.get("fiveDaysReturn")),
        one_month_return: format_return(performance_data.get("oneMonthReturn")),
        three_month_return: format_return(performance_data.get("threeMonthReturn")),
        six_month_return: format_return(performance_data.get("sixMonthReturn")),
        ytd_return: format_return(performance_data.get("ytdReturnPct")),
        year_return: format_return(performance_data.get("oneYearTotalReturn")),
        three_year_return: format_return(performance_data.get("threeYearTotalReturn")),
        five_year_return: format_return(performance_data.get("fiveYearTotalReturn")),
        ten_year_return: format_return(performance_data.get("tenYearTotalReturn")),
        max_return: format_return(performance_data.get("maxReturn")),
    })
}

/// Fetch a single index
async fn fetch_index(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    index: Index,
) -> Result<MarketIndex, YahooError> {
    let symbol = get_yahoo_index_symbol(index);
    info!("Fetching index {} (symbol: {})", index.as_str(), symbol);

    // Try the full quote-summary endpoint first
    match yahoo_client.get_quote(&symbol).await {
        Ok(data) => {
            debug!("Successfully fetched quote-summary for {}", symbol);
            return parse_yahoo_index(data, index).await;
        }
        Err(e) => {
            warn!(
                "Failed to fetch quote-summary for {}: {}, trying fallback",
                symbol, e
            );
        }
    }

    // If that fails, try the simple quotes endpoint
    match yahoo_client.get_simple_quotes(&[&symbol]).await {
        Ok(data) => {
            if let Some(quote_response) = data.get("quoteResponse")
                && let Some(results) = quote_response.get("result").and_then(|r| r.as_array())
                && let Some(result) = results.first()
            {
                debug!("Successfully fetched simple quote for {}", symbol);
                // Convert simple quote format to quote-summary format for parsing
                let mock_summary = serde_json::json!({
                    "quoteSummary": {
                        "result": [{
                            "price": result,
                            "quoteUnadjustedPerformanceOverview": {
                                "performanceOverview": {}
                            }
                        }]
                    }
                });
                return parse_yahoo_index(mock_summary, index).await;
            }
        }
        Err(e) => {
            warn!(
                "Failed to fetch simple quotes for {}: {}, trying final fallback",
                symbol, e
            );
        }
    }

    // As last resort, try get_quotes which might use scraping
    match quotes::get_quotes(yahoo_client, fetch_client, &[&symbol]).await {
        Ok(quotes_data) => {
            if let Some(quote) = quotes_data.first() {
                debug!("Successfully fetched quote via scraping for {}", symbol);
                return Ok(MarketIndex {
                    name: get_formatted_index_name(index, quote.name.clone()),
                    value: quote.price.parse::<f64>().unwrap_or(0.0),
                    change: quote.change.clone(),
                    percent_change: quote.percent_change.clone(),
                    five_days_return: quote.five_days_return.clone(),
                    one_month_return: quote.one_month_return.clone(),
                    three_month_return: quote.three_month_return.clone(),
                    six_month_return: quote.six_month_return.clone(),
                    ytd_return: quote.ytd_return.clone(),
                    year_return: quote.year_return.clone(),
                    three_year_return: quote.three_year_return.clone(),
                    five_year_return: quote.five_year_return.clone(),
                    ten_year_return: quote.ten_year_return.clone(),
                    max_return: quote.max_return.clone(),
                });
            }
        }
        Err(e) => {
            warn!("Failed to fetch quote via scraping for {}: {}", symbol, e);
        }
    }

    // If all else fails, create a minimal MarketIndex
    warn!(
        "All fetch methods failed for {}, returning minimal index",
        symbol
    );
    Ok(MarketIndex {
        name: index.as_str().to_string(),
        value: 0.0,
        change: "".to_string(),
        percent_change: "".to_string(),
        five_days_return: None,
        one_month_return: None,
        three_month_return: None,
        six_month_return: None,
        ytd_return: None,
        year_return: None,
        three_year_return: None,
        five_year_return: None,
        ten_year_return: None,
        max_return: None,
    })
}

/// Get indices, optionally filtered by specific indices or region
pub async fn get_indices(
    yahoo_client: &YahooFinanceClient,
    fetch_client: &Arc<FetchClient>,
    indices: Option<Vec<Index>>,
    region: Option<Region>,
) -> Result<Vec<MarketIndex>, YahooError> {
    let index_regions = get_index_regions();
    let mut selected_indices: HashSet<Index> = HashSet::new();

    // Add explicitly requested indices
    if let Some(requested_indices) = indices {
        selected_indices.extend(requested_indices);
    }

    // Add indices from selected region
    if let Some(selected_region) = region {
        for (idx, idx_region) in &index_regions {
            if *idx_region == selected_region
                || (*idx_region == Region::UnitedStates && selected_region == Region::NorthAmerica)
            {
                selected_indices.insert(*idx);
            }
        }
    }

    // If no indices selected, get all
    if selected_indices.is_empty() {
        selected_indices = Index::all().into_iter().collect();
    }

    // Convert back to ordered list
    let ordered_indices: Vec<Index> = Index::all()
        .into_iter()
        .filter(|idx| selected_indices.contains(idx))
        .collect();

    info!("Fetching {} indices", ordered_indices.len());

    // Fetch indices sequentially (can be optimized to concurrent later if needed)
    let mut results = Vec::new();
    for index in ordered_indices {
        match fetch_index(yahoo_client, fetch_client, index).await {
            Ok(index_data) => {
                results.push(index_data);
            }
            Err(e) => {
                error!("Failed to fetch index {}: {}", index.as_str(), e);
                // Continue with other indices - create minimal entry
                results.push(MarketIndex {
                    name: index.as_str().to_string(),
                    value: 0.0,
                    change: "".to_string(),
                    percent_change: "".to_string(),
                    five_days_return: None,
                    one_month_return: None,
                    three_month_return: None,
                    six_month_return: None,
                    ytd_return: None,
                    year_return: None,
                    three_year_return: None,
                    five_year_return: None,
                    ten_year_return: None,
                    max_return: None,
                });
            }
        }
    }

    info!("Successfully fetched {} indices", results.len());
    Ok(results)
}
