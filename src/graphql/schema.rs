use async_graphql::*;
use actix_web::web;
use crate::AppState;
use crate::graphql::types::*;
use crate::models::historical::{TimeRange as TimeRangeModel, Interval as IntervalModel, IndicatorType as IndicatorTypeModel};
use crate::models::holders::HolderType;
use crate::models::analysts::AnalysisType;
use crate::models::financials::{StatementType as StatementTypeModel, Frequency as FrequencyModel};
use crate::service::historical::calculate_indicators;
use crate::service::market::MarketSchedule;
use crate::service::websocket::indicator::moving_average::{MovingAverageType, calculate_ma};
use std::collections::HashSet;
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use futures_util::Stream;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use chrono::Utc;
use tracing::error;
use async_stream::stream;

pub struct AppContext {
    pub app_state: web::Data<AppState>,
}

pub struct Query;

#[Object]
impl Query {
    // Quote endpoints
    async fn quotes(
        &self,
        ctx: &Context<'_>,
        symbols: Vec<String>,
    ) -> Result<Vec<Quote>> {
        let context = ctx.data::<AppContext>()?;
        let symbol_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
        let quotes = crate::service::get_quotes(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            &symbol_refs,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(quotes.into_iter().map(Quote::from).collect())
    }

    async fn simple_quotes(
        &self,
        ctx: &Context<'_>,
        symbols: Vec<String>,
    ) -> Result<Vec<SimpleQuote>> {
        let context = ctx.data::<AppContext>()?;
        let symbol_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
        let quotes = crate::service::get_simple_quotes(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            &symbol_refs,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(quotes.into_iter().map(SimpleQuote::from).collect())
    }

    async fn detailed_quotes(
        &self,
        ctx: &Context<'_>,
        symbols: Vec<String>,
    ) -> Result<Vec<DetailedQuote>> {
        let context = ctx.data::<AppContext>()?;
        let symbol_refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
        let quotes = crate::service::get_quotes(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            &symbol_refs,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(quotes.into_iter()
            .map(|q| crate::models::quote::DetailedQuote::from(q))
            .map(DetailedQuote::from)
            .collect())
    }

    async fn similar(
        &self,
        ctx: &Context<'_>,
        symbol: String,
        limit: Option<usize>,
    ) -> Result<Vec<SimpleQuote>> {
        let context = ctx.data::<AppContext>()?;
        let limit = limit.unwrap_or(10).clamp(1, 20);
        let quotes = crate::service::get_similar_quotes(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            &symbol,
            limit,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        // get_similar_quotes returns Vec<SimpleQuote>
        Ok(quotes.into_iter().map(SimpleQuote::from).collect())
    }

    // Historical endpoint
    async fn historical(
        &self,
        ctx: &Context<'_>,
        symbol: String,
        range: String,
        interval: String,
        indicators: Option<Vec<String>>,
        period: Option<String>,
    ) -> Result<HistoricalResponse> {
        let context = ctx.data::<AppContext>()?;
        
        // Parse time range
        let time_range = parse_time_range(&range)
            .map_err(|_| Error::new("Invalid time range"))?;
        
        // Parse interval
        let interval_model = parse_interval(&interval)
            .map_err(|_| Error::new("Invalid interval"))?;
        
        // Validate interval-range compatibility
        validate_interval_range_compatibility(&interval_model, &time_range)?;
        
        // Get historical data
        let mut historical = crate::service::get_historical(
            &context.app_state.yahoo_client,
            &symbol,
            time_range,
            interval_model,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        
        // Calculate indicators if requested
        if let Some(indicators_str) = indicators {
            let requested_indicators: HashSet<IndicatorTypeModel> = indicators_str
                .iter()
                .filter_map(|s| IndicatorTypeModel::from_str(s))
                .collect();
            
            if requested_indicators.is_empty() {
                return Err(Error::new("Invalid indicator type. Supported: sma, ema"));
            }
            
            let periods_str = period.as_ref().map(|s| s.as_str()).unwrap_or("20");
            let periods = parse_periods(periods_str)?;
            
            if periods.is_empty() {
                return Err(Error::new("At least one period must be specified"));
            }
            
            historical = calculate_indicators(historical, &periods, &requested_indicators);
        }
        
        Ok(HistoricalResponse::from(historical))
    }

    // Search endpoint
    async fn search(
        &self,
        ctx: &Context<'_>,
        query: String,
        hits: Option<usize>,
    ) -> Result<SearchResponse> {
        let context = ctx.data::<AppContext>()?;
        let hits = hits.unwrap_or(6);
        let results = crate::service::search(
            &context.app_state.yahoo_client,
            &query,
            hits,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(SearchResponse::from(results))
    }

    // News endpoints
    async fn news(
        &self,
        ctx: &Context<'_>,
        symbol: Option<String>,
    ) -> Result<Vec<News>> {
        let context = ctx.data::<AppContext>()?;
        let news_list = if let Some(symbol) = symbol {
            crate::service::scrape_news_for_quote(
                &context.app_state.fetch_client,
                &symbol,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?
        } else {
            crate::service::scrape_general_news(
                &context.app_state.fetch_client,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?
        };
        Ok(news_list.into_iter().map(News::from).collect())
    }

    async fn news_by_symbol(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<Vec<News>> {
        let context = ctx.data::<AppContext>()?;
        let news_list = crate::service::scrape_news_for_quote(
            &context.app_state.fetch_client,
            &symbol,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(news_list.into_iter().map(News::from).collect())
    }

    // Financials endpoint
    async fn financials(
        &self,
        ctx: &Context<'_>,
        symbol: String,
        statement: Option<String>,
        frequency: Option<String>,
    ) -> Result<FinancialStatement> {
        let context = ctx.data::<AppContext>()?;
        let statement_type = statement
            .as_ref()
            .map(|s| parse_statement_type(s))
            .transpose()?
            .unwrap_or(StatementTypeModel::IncomeStatement);
        let freq = frequency
            .as_ref()
            .map(|s| parse_frequency(s))
            .transpose()?
            .unwrap_or(FrequencyModel::Annual);
        let stmt = crate::service::get_financial_statement(
            &context.app_state.yahoo_client,
            &symbol,
            statement_type,
            freq,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(FinancialStatement::from(stmt))
    }

    // Earnings endpoints
    async fn earnings_calls(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<EarningsCallsList> {
        let context = ctx.data::<AppContext>()?;
        let calls = crate::service::get_earnings_calls_list(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            &symbol,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(EarningsCallsList::from(calls))
    }

    async fn earnings_transcript(
        &self,
        ctx: &Context<'_>,
        symbol: String,
        quarter: Option<String>,
        year: Option<i32>,
    ) -> Result<EarningsTranscript> {
        let context = ctx.data::<AppContext>()?;
        let transcript = crate::service::get_earnings_transcript(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            &symbol,
            quarter,
            year,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(EarningsTranscript::from(transcript))
    }

    // Movers endpoints
    async fn actives(&self, ctx: &Context<'_>) -> Result<Vec<MarketMover>> {
        let context = ctx.data::<AppContext>()?;
        use crate::models::movers::MoverCount;
        let movers = crate::service::get_actives(
            &context.app_state.yahoo_client,
            MoverCount::Fifty,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(movers.into_iter().map(MarketMover::from).collect())
    }

    async fn gainers(&self, ctx: &Context<'_>) -> Result<Vec<MarketMover>> {
        let context = ctx.data::<AppContext>()?;
        use crate::models::movers::MoverCount;
        let movers = crate::service::get_gainers(
            &context.app_state.yahoo_client,
            MoverCount::Fifty,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(movers.into_iter().map(MarketMover::from).collect())
    }

    async fn losers(&self, ctx: &Context<'_>) -> Result<Vec<MarketMover>> {
        let context = ctx.data::<AppContext>()?;
        use crate::models::movers::MoverCount;
        let movers = crate::service::get_losers(
            &context.app_state.yahoo_client,
            MoverCount::Fifty,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(movers.into_iter().map(MarketMover::from).collect())
    }

    // Indices endpoint
    async fn indices(&self, ctx: &Context<'_>) -> Result<Vec<MarketIndex>> {
        let context = ctx.data::<AppContext>()?;
        let indices = crate::service::get_indices(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            None,
            None,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(indices.into_iter().map(MarketIndex::from).collect())
    }

    // Holders endpoints
    async fn major_holders(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<MajorHoldersResponse> {
        let context = ctx.data::<AppContext>()?;
        let holders_data = crate::service::holders::get_holders_data(
            &context.app_state.yahoo_client,
            &symbol,
            HolderType::Major,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(MajorHoldersResponse {
            symbol: holders_data.symbol,
            breakdown: MajorHoldersBreakdown::from(
                holders_data.major_breakdown
                    .ok_or_else(|| Error::new("No major breakdown data"))?
            ),
        })
    }

    async fn institutional_holders(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<InstitutionalHoldersResponse> {
        let context = ctx.data::<AppContext>()?;
        let holders_data = crate::service::holders::get_holders_data(
            &context.app_state.yahoo_client,
            &symbol,
            HolderType::Institutional,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(InstitutionalHoldersResponse {
            symbol: holders_data.symbol,
            holders: holders_data.institutional_holders
                .ok_or_else(|| Error::new("No institutional holders data"))?
                .into_iter()
                .map(InstitutionalHolder::from)
                .collect(),
        })
    }

    async fn mutual_fund_holders(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<MutualFundHoldersResponse> {
        let context = ctx.data::<AppContext>()?;
        let holders_data = crate::service::holders::get_holders_data(
            &context.app_state.yahoo_client,
            &symbol,
            HolderType::MutualFund,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(MutualFundHoldersResponse {
            symbol: holders_data.symbol,
            holders: holders_data.mutualfund_holders
                .ok_or_else(|| Error::new("No mutual fund holders data"))?
                .into_iter()
                .map(MutualFundHolder::from)
                .collect(),
        })
    }

    async fn insider_transactions(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<InsiderTransactionsResponse> {
        let context = ctx.data::<AppContext>()?;
        let holders_data = crate::service::holders::get_holders_data(
            &context.app_state.yahoo_client,
            &symbol,
            HolderType::InsiderTransactions,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(InsiderTransactionsResponse {
            symbol: holders_data.symbol,
            transactions: holders_data.insider_transactions
                .ok_or_else(|| Error::new("No insider transactions data"))?
                .into_iter()
                .map(InsiderTransaction::from)
                .collect(),
        })
    }

    async fn insider_purchases(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<InsiderPurchasesResponse> {
        let context = ctx.data::<AppContext>()?;
        let holders_data = crate::service::holders::get_holders_data(
            &context.app_state.yahoo_client,
            &symbol,
            HolderType::InsiderPurchases,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(InsiderPurchasesResponse {
            symbol: holders_data.symbol,
            summary: InsiderPurchase::from(
                holders_data.insider_purchases
                    .ok_or_else(|| Error::new("No insider purchases data"))?
            ),
        })
    }

    async fn insider_roster(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<InsiderRosterResponse> {
        let context = ctx.data::<AppContext>()?;
        let holders_data = crate::service::holders::get_holders_data(
            &context.app_state.yahoo_client,
            &symbol,
            HolderType::InsiderRoster,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(InsiderRosterResponse {
            symbol: holders_data.symbol,
            roster: holders_data.insider_roster
                .ok_or_else(|| Error::new("No insider roster data"))?
                .into_iter()
                .map(InsiderRosterMember::from)
                .collect(),
        })
    }

    // Analysts endpoints
    async fn recommendations(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<RecommendationsResponse> {
        let context = ctx.data::<AppContext>()?;
        let data = crate::service::analysts::get_analysis_data(
            &context.app_state.yahoo_client,
            &symbol,
            AnalysisType::Recommendations,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        let recommendations: Vec<crate::models::analysts::RecommendationData> = 
            serde_json::from_value(data.get("recommendations")
                .ok_or_else(|| Error::new("No recommendations data"))?
                .clone())
            .map_err(|e| Error::new(format!("Failed to parse recommendations: {}", e)))?;
        Ok(RecommendationsResponse {
            symbol: data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or(&symbol)
                .to_string(),
            recommendations: recommendations.into_iter().map(RecommendationData::from).collect(),
        })
    }

    async fn upgrades_downgrades(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<UpgradesDowngradesResponse> {
        let context = ctx.data::<AppContext>()?;
        let data = crate::service::analysts::get_analysis_data(
            &context.app_state.yahoo_client,
            &symbol,
            AnalysisType::UpgradesDowngrades,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        let upgrades_downgrades: Vec<crate::models::analysts::UpgradeDowngrade> = 
            serde_json::from_value(data.get("upgrades_downgrades")
                .ok_or_else(|| Error::new("No upgrades_downgrades data"))?
                .clone())
            .map_err(|e| Error::new(format!("Failed to parse upgrades_downgrades: {}", e)))?;
        Ok(UpgradesDowngradesResponse {
            symbol: data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or(&symbol)
                .to_string(),
            upgrades_downgrades: upgrades_downgrades.into_iter().map(UpgradeDowngrade::from).collect(),
        })
    }

    async fn price_targets(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<PriceTargetsResponse> {
        let context = ctx.data::<AppContext>()?;
        let data = crate::service::analysts::get_analysis_data(
            &context.app_state.yahoo_client,
            &symbol,
            AnalysisType::PriceTargets,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        let price_targets: crate::models::analysts::PriceTarget = 
            serde_json::from_value(data.get("price_targets")
                .ok_or_else(|| Error::new("No price_targets data"))?
                .clone())
            .map_err(|e| Error::new(format!("Failed to parse price_targets: {}", e)))?;
        Ok(PriceTargetsResponse {
            symbol: data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or(&symbol)
                .to_string(),
            price_targets: PriceTarget::from(price_targets),
        })
    }

    async fn earnings_estimate(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<EarningsEstimateResponse> {
        let context = ctx.data::<AppContext>()?;
        let data = crate::service::analysts::get_analysis_data(
            &context.app_state.yahoo_client,
            &symbol,
            AnalysisType::EarningsEstimate,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        let earnings_estimate: crate::models::analysts::EarningsEstimate = 
            serde_json::from_value(data.get("earnings_estimate")
                .ok_or_else(|| Error::new("No earnings_estimate data"))?
                .clone())
            .map_err(|e| Error::new(format!("Failed to parse earnings_estimate: {}", e)))?;
        Ok(EarningsEstimateResponse {
            symbol: data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or(&symbol)
                .to_string(),
            earnings_estimate: EarningsEstimate::from(earnings_estimate),
        })
    }

    async fn revenue_estimate(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<RevenueEstimateResponse> {
        let context = ctx.data::<AppContext>()?;
        let data = crate::service::analysts::get_analysis_data(
            &context.app_state.yahoo_client,
            &symbol,
            AnalysisType::RevenueEstimate,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        let revenue_estimate: crate::models::analysts::RevenueEstimate = 
            serde_json::from_value(data.get("revenue_estimate")
                .ok_or_else(|| Error::new("No revenue_estimate data"))?
                .clone())
            .map_err(|e| Error::new(format!("Failed to parse revenue_estimate: {}", e)))?;
        Ok(RevenueEstimateResponse {
            symbol: data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or(&symbol)
                .to_string(),
            revenue_estimate: RevenueEstimate::from(revenue_estimate),
        })
    }

    async fn earnings_history(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<EarningsHistoryResponse> {
        let context = ctx.data::<AppContext>()?;
        let data = crate::service::analysts::get_analysis_data(
            &context.app_state.yahoo_client,
            &symbol,
            AnalysisType::EarningsHistory,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        let earnings_history: Vec<crate::models::analysts::EarningsHistoryItem> = 
            serde_json::from_value(data.get("earnings_history")
                .ok_or_else(|| Error::new("No earnings_history data"))?
                .clone())
            .map_err(|e| Error::new(format!("Failed to parse earnings_history: {}", e)))?;
        Ok(EarningsHistoryResponse {
            symbol: data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or(&symbol)
                .to_string(),
            earnings_history: earnings_history.into_iter().map(EarningsHistoryItem::from).collect(),
        })
    }

    // Sectors endpoints
    async fn sectors(&self, ctx: &Context<'_>) -> Result<Vec<MarketSector>> {
        let context = ctx.data::<AppContext>()?;
        let sectors = crate::service::get_sectors(
            &context.app_state.fetch_client,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(sectors.into_iter().map(MarketSector::from).collect())
    }

    async fn sector_for_symbol(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> Result<MarketSector> {
        let context = ctx.data::<AppContext>()?;
        let sector = crate::service::get_sector_for_symbol(
            &context.app_state.yahoo_client,
            &context.app_state.fetch_client,
            &symbol,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(MarketSector::from(sector))
    }

    async fn sector_details(
        &self,
        ctx: &Context<'_>,
        sector: String,
    ) -> Result<MarketSectorDetails> {
        let context = ctx.data::<AppContext>()?;
        use crate::models::sectors::Sector;
        use std::str::FromStr;
        let sector_enum = Sector::from_str(&sector)
            .map_err(|e| Error::new(format!("Invalid sector: {}", e)))?;
        let details = crate::service::get_sector_details(
            &context.app_state.fetch_client,
            sector_enum,
        )
        .await
        .map_err(|e| Error::new(e.to_string()))?;
        Ok(MarketSectorDetails::from(details))
    }

    // Health endpoints
    async fn ping(&self) -> Result<String> {
        Ok("healthy".to_string())
    }

    async fn health(&self) -> Result<HealthResponse> {
        use chrono::Utc;
        Ok(HealthResponse {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            services: crate::graphql::types::health::HealthServices {
                status: "all_operational".to_string(),
            },
        })
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    /// Subscribe to profile updates for a symbol (quote, similar, sector, news)
    async fn profile_updates(
        &self,
        ctx: &Context<'_>,
        symbol: String,
    ) -> impl Stream<Item = Result<ProfileUpdate>> {
        let context = ctx.data::<AppContext>().ok();
        let symbol_upper = symbol.to_uppercase();
        
        let interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        interval_stream.then(move |_| {
            let context_clone = context.clone();
            let symbol_clone = symbol_upper.clone();
            
            async move {
                if let Some(ctx) = &context_clone {
                    let yahoo = ctx.app_state.yahoo_client.clone();
                    let fetch = ctx.app_state.fetch_client.clone();
                    let sym = symbol_clone.clone();
                    
                    let symbols = vec![sym.as_str()];
                    let quotes_task = crate::service::get_quotes(&yahoo, &fetch, &symbols);
                    let similar_task = crate::service::get_similar_quotes(&yahoo, &fetch, &sym, 10);
                    let sector_task = crate::service::get_sector_for_symbol(&yahoo, &fetch, &sym);
                    let news_task = crate::service::scrape_news_for_quote(&fetch, &sym);
                    
                    let (quotes_result, similar_result, sector_result, news_result) = tokio::join!(
                        quotes_task,
                        similar_task,
                        sector_task,
                        news_task
                    );
                    
                    let quote = quotes_result.ok()
                        .and_then(|q| q.first().cloned())
                        .map(Quote::from);
                    let similar = similar_result.ok().map(|s| s.into_iter().map(SimpleQuote::from).collect());
                    let sector = sector_result.ok().map(MarketSector::from);
                    let news = news_result.ok().map(|n| n.into_iter().map(News::from).collect());
                    
                    Ok(ProfileUpdate {
                        quote,
                        similar,
                        sector_performance: sector,
                        news,
                    })
                } else {
                    Err(Error::new("Context not available"))
                }
            }
        })
    }

    /// Subscribe to quote updates for multiple symbols
    async fn quote_updates(
        &self,
        ctx: &Context<'_>,
        symbols: Vec<String>,
    ) -> impl Stream<Item = Result<SimpleQuote>> {
        let context = ctx.data::<AppContext>().ok();
        let symbol_refs: Vec<String> = symbols.iter().map(|s| s.to_uppercase()).collect();
        
        let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        stream! {
            while let Some(_) = interval_stream.next().await {
                if let Some(ctx) = &context {
                    let yahoo = ctx.app_state.yahoo_client.clone();
                    let fetch = ctx.app_state.fetch_client.clone();
                    let symbols_clone: Vec<&str> = symbol_refs.iter().map(|s| s.as_str()).collect();
                    
                    match crate::service::get_simple_quotes(&yahoo, &fetch, &symbols_clone).await {
                        Ok(quotes) => {
                            for quote in quotes {
                                yield Ok(SimpleQuote::from(quote));
                            }
                        }
                        Err(e) => {
                            error!("Failed to fetch quotes: {}", e);
                            yield Err(Error::new(format!("Failed to fetch quotes: {}", e)));
                        }
                    }
                } else {
                    yield Err(Error::new("Context not available"));
                    break;
                }
            }
        }
    }

    /// Subscribe to market indices updates
    async fn indices_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MarketIndex>> {
        let context = ctx.data::<AppContext>().ok();
        
        let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        stream! {
            while let Some(_) = interval_stream.next().await {
                if let Some(ctx) = &context {
                    let yahoo = ctx.app_state.yahoo_client.clone();
                    let fetch = ctx.app_state.fetch_client.clone();
                    
                    use crate::models::indices::Index;
                    let indices_to_fetch = vec![Index::Dji, Index::Ixic, Index::Gspc];
                    
                    match crate::service::get_indices(&yahoo, &fetch, Some(indices_to_fetch), None).await {
                        Ok(indices) => {
                            for index in indices {
                                yield Ok(MarketIndex::from(index));
                            }
                        }
                        Err(e) => {
                            error!("Failed to fetch indices: {}", e);
                            yield Err(Error::new(format!("Failed to fetch indices: {}", e)));
                        }
                    }
                } else {
                    yield Err(Error::new("Context not available"));
                    break;
                }
            }
        }
    }

    /// Subscribe to general market news updates
    async fn news_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<News>> {
        let context = ctx.data::<AppContext>().ok();
        
        let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        stream! {
            while let Some(_) = interval_stream.next().await {
                if let Some(ctx) = &context {
                    let fetch = ctx.app_state.fetch_client.clone();
                    
                    match crate::service::scrape_general_news(&fetch).await {
                        Ok(news_list) => {
                            for news_item in news_list {
                                yield Ok(News::from(news_item));
                            }
                        }
                        Err(e) => {
                            error!("Failed to fetch news: {}", e);
                            yield Err(Error::new(format!("Failed to fetch news: {}", e)));
                        }
                    }
                } else {
                    yield Err(Error::new("Context not available"));
                    break;
                }
            }
        }
    }

    /// Subscribe to sector performance updates
    async fn sectors_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MarketSector>> {
        let context = ctx.data::<AppContext>().ok();
        
        let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        stream! {
            while let Some(_) = interval_stream.next().await {
                if let Some(ctx) = &context {
                    let fetch = ctx.app_state.fetch_client.clone();
                    
                    match crate::service::get_sectors(&fetch).await {
                        Ok(sectors) => {
                            for sector in sectors {
                                yield Ok(MarketSector::from(sector));
                            }
                        }
                        Err(e) => {
                            error!("Failed to fetch sectors: {}", e);
                            yield Err(Error::new(format!("Failed to fetch sectors: {}", e)));
                        }
                    }
                } else {
                    yield Err(Error::new("Context not available"));
                    break;
                }
            }
        }
    }

    /// Subscribe to market movers updates (actives, gainers, losers)
    async fn movers_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MoversUpdate>> {
        let context = ctx.data::<AppContext>().ok();
        
        let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        stream! {
            while let Some(_) = interval_stream.next().await {
                if let Some(ctx) = &context {
                    let yahoo = ctx.app_state.yahoo_client.clone();
                    
                    use crate::models::movers::MoverCount;
                    let actives_task = crate::service::get_actives(&yahoo, MoverCount::Fifty);
                    let gainers_task = crate::service::get_gainers(&yahoo, MoverCount::Fifty);
                    let losers_task = crate::service::get_losers(&yahoo, MoverCount::Fifty);
                    
                    let (actives_result, gainers_result, losers_result) = tokio::join!(
                        actives_task,
                        gainers_task,
                        losers_task
                    );
                    
                    // Filter to US-only stocks (symbols without dots or with US exchange suffixes)
                    let filter_us = |movers: Vec<crate::models::movers::MarketMover>| -> Vec<MarketMover> {
                        movers.into_iter()
                            .filter(|m| {
                                let symbol = &m.symbol;
                                !symbol.contains('.') || 
                                symbol.ends_with(".OB") || 
                                symbol.ends_with(".PK") ||
                                symbol.ends_with(".OTC")
                            })
                            .map(MarketMover::from)
                            .collect()
                    };
                    
                    let actives = actives_result.ok().map(filter_us);
                    let gainers = gainers_result.ok().map(filter_us);
                    let losers = losers_result.ok().map(filter_us);
                    
                    yield Ok(MoversUpdate {
                        actives,
                        gainers,
                        losers,
                    });
                } else {
                    yield Err(Error::new("Context not available"));
                    break;
                }
            }
        }
    }

    /// Subscribe to market hours/status updates
    async fn market_hours_updates(
        &self,
        _ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<MarketHours>> {
        let market_schedule = Arc::new(MarketSchedule::new());
        
        let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        stream! {
            while let Some(_) = interval_stream.next().await {
                let (status, reason) = market_schedule.get_market_status();
                yield Ok(MarketHours {
                    status: status.as_str().to_string(),
                    reason,
                    timestamp: Utc::now(),
                });
            }
        }
    }

    /// Subscribe to moving average updates for a symbol
    async fn moving_average_updates(
        &self,
        ctx: &Context<'_>,
        symbol: String,
        indicator_type: String,
        period: i32,
    ) -> impl Stream<Item = Result<MovingAverageUpdate>> {
        let context = ctx.data::<AppContext>().ok();
        let symbol_upper = symbol.to_uppercase();
        let period_usize = period as usize;
        let ma_type_result = match indicator_type.to_lowercase().as_str() {
            "sma" => Ok(MovingAverageType::SMA),
            "ema" => Ok(MovingAverageType::EMA),
            _ => Err(Error::new("Invalid indicator type. Must be 'sma' or 'ema'")),
        };
        
        let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(5)));
        
        stream! {
            let ma_type = match ma_type_result {
                Ok(ma) => ma,
                Err(e) => {
                    yield Err(e);
                    return;
                }
            };
            
            while let Some(_) = interval_stream.next().await {
                if let Some(ctx) = &context {
                    let yahoo = ctx.app_state.yahoo_client.clone();
                    let fetch = ctx.app_state.fetch_client.clone();
                    let price_buffer = ctx.app_state.price_buffer_manager.clone();
                    let sym = symbol_upper.clone();
                    
                    // Get current price
                    let symbol_refs = vec![sym.as_str()];
                    let price_result = crate::service::get_simple_quotes(&yahoo, &fetch, &symbol_refs).await;
                    
                    if let Ok(quotes) = price_result {
                        if let Some(quote) = quotes.first() {
                            // Parse price
                            if let Ok(price) = quote.price.parse::<f64>() {
                                // Add to price buffer (real-time, not daily/weekly)
                                price_buffer.add_price(&sym, price, false, false).await;
                                
                                // Get price history from buffer
                                let prices = price_buffer.get_prices(&sym).await;
                                
                                if prices.len() >= period_usize {
                                    // Calculate moving average
                                    if let Some(ma_value) = calculate_ma(&prices, ma_type, period_usize) {
                                        yield Ok(MovingAverageUpdate {
                                            symbol: sym.clone(),
                                            indicator_type: indicator_type.clone(),
                                            period,
                                            value: ma_value,
                                            timestamp: Utc::now(),
                                        });
                                    } else {
                                        yield Err(Error::new("Failed to calculate moving average"));
                                    }
                                } else {
                                    yield Err(Error::new(format!(
                                        "Not enough data points. Need {}, have {}",
                                        period_usize,
                                        prices.len()
                                    )));
                                }
                            } else {
                                yield Err(Error::new("Failed to parse price"));
                            }
                        } else {
                            yield Err(Error::new("Quote not found"));
                        }
                    } else {
                        yield Err(Error::new("Failed to fetch quote"));
                    }
                } else {
                    yield Err(Error::new("Context not available"));
                    break;
                }
            }
        }
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, Subscription>;

// Helper functions for parsing
fn parse_time_range(s: &str) -> Result<TimeRangeModel, ()> {
    match s {
        "1d" => Ok(TimeRangeModel::Day),
        "5d" => Ok(TimeRangeModel::FiveDays),
        "1mo" => Ok(TimeRangeModel::OneMonth),
        "3mo" => Ok(TimeRangeModel::ThreeMonths),
        "6mo" => Ok(TimeRangeModel::SixMonths),
        "1y" => Ok(TimeRangeModel::Year),
        "2y" => Ok(TimeRangeModel::TwoYears),
        "5y" => Ok(TimeRangeModel::FiveYears),
        "10y" => Ok(TimeRangeModel::TenYears),
        "ytd" => Ok(TimeRangeModel::Ytd),
        "max" => Ok(TimeRangeModel::Max),
        _ => Err(()),
    }
}

fn parse_interval(s: &str) -> Result<IntervalModel, ()> {
    match s {
        "1m" => Ok(IntervalModel::OneMinute),
        "3m" => Ok(IntervalModel::ThreeMinutes),
        "5m" => Ok(IntervalModel::FiveMinutes),
        "10m" => Ok(IntervalModel::TenMinutes),
        "15m" => Ok(IntervalModel::FifteenMinutes),
        "20m" => Ok(IntervalModel::TwentyMinutes),
        "30m" => Ok(IntervalModel::ThirtyMinutes),
        "65m" => Ok(IntervalModel::SixtyFiveMinutes),
        "95m" => Ok(IntervalModel::NinetyFiveMinutes),
        "1h" => Ok(IntervalModel::OneHour),
        "1d" => Ok(IntervalModel::Daily),
        "1wk" => Ok(IntervalModel::Weekly),
        "1mo" => Ok(IntervalModel::Monthly),
        _ => Err(()),
    }
}

fn validate_interval_range_compatibility(
    interval: &IntervalModel,
    time_range: &TimeRangeModel,
) -> Result<(), Error> {
    let restricted_minute_intervals = matches!(
        interval,
        IntervalModel::OneMinute
            | IntervalModel::ThreeMinutes
            | IntervalModel::FiveMinutes
            | IntervalModel::TenMinutes
            | IntervalModel::FifteenMinutes
            | IntervalModel::TwentyMinutes
            | IntervalModel::ThirtyMinutes
            | IntervalModel::SixtyFiveMinutes
    );

    if restricted_minute_intervals {
        let allowed_ranges = matches!(time_range, TimeRangeModel::Day | TimeRangeModel::FiveDays);
        
        if !allowed_ranges {
            return Err(Error::new(format!(
                "The interval '{}' can only be used with ranges '1d' or '5d'. Please use one of these ranges or choose a different interval.",
                interval.as_str()
            )));
        }
    }

    Ok(())
}

fn parse_periods(periods_str: &str) -> Result<Vec<usize>, Error> {
    let periods: Result<Vec<usize>, _> = periods_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<usize>()
                .map_err(|_| Error::new(format!("Invalid period value: '{}'. Periods must be positive integers.", s)))
        })
        .collect();
    
    let periods = periods?;
    
    for period in &periods {
        if *period == 0 {
            return Err(Error::new("Period must be greater than 0"));
        }
    }
    
    if periods.is_empty() {
        return Err(Error::new("At least one period must be specified"));
    }
    
    Ok(periods)
}

fn parse_statement_type(s: &str) -> Result<StatementTypeModel, Error> {
    match s {
        "income" => Ok(StatementTypeModel::IncomeStatement),
        "balance" => Ok(StatementTypeModel::BalanceSheet),
        "cashflow" => Ok(StatementTypeModel::CashFlow),
        _ => Err(Error::new(format!("Invalid statement type: {}", s))),
    }
}

fn parse_frequency(s: &str) -> Result<FrequencyModel, Error> {
    match s {
        "annual" => Ok(FrequencyModel::Annual),
        "quarterly" => Ok(FrequencyModel::Quarterly),
        _ => Err(Error::new(format!("Invalid frequency: {}", s))),
    }
}