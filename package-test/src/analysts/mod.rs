//! Analyst data module for fetching recommendations, price targets, and estimates

mod analysts;

pub use analysts::{
    // Functions
    get_recommendations,
    get_upgrades_downgrades,
    get_price_targets,
    get_earnings_history,
    get_earnings_estimates,
    get_revenue_estimates,
    get_eps_trend,
    get_eps_revisions,
    get_growth_estimates,
    get_all_analyst_data,
    get_analysts_raw,
    // Types
    AnalysisType,
    RecommendationData,
    UpgradeDowngrade,
    PriceTarget,
    EarningsEstimate,
    RevenueEstimate,
    EarningsHistoryItem,
    EpsTrend,
    EpsRevisions,
    GrowthEstimate,
    AnalystData,
};
