pub mod quotes;
pub mod historical;
pub mod search;
pub mod news;
pub mod financials;
pub mod earnings_transcript;
pub mod health;
pub mod similar;
pub mod movers;
pub mod indices;
pub mod holders;
pub mod analysts;
pub mod sectors;
pub mod websocket;

use actix_web::web;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/quotes", web::get().to(quotes::get_quotes_handler))
            .route("/simple-quotes", web::get().to(quotes::get_simple_quotes_handler))
            .route("/detailed-quotes", web::get().to(quotes::get_detailed_quotes_handler))
            .route("/similar", web::get().to(similar::get_similar_quotes_handler))
            .route("/historical/{symbol}", web::get().to(historical::get_historical_handler))
            .route("/search", web::get().to(search::search_handler))
            .route("/news", web::get().to(news::get_news_handler))
            .route("/financials/{symbol}", web::get().to(financials::get_financials_handler))
            .route("/earnings/{symbol}/calls", web::get().to(earnings_transcript::get_earnings_calls_handler))
            .route("/earnings/{symbol}/transcript", web::get().to(earnings_transcript::get_earnings_transcript_handler))
            .route("/actives", web::get().to(movers::get_actives_handler))
            .route("/gainers", web::get().to(movers::get_gainers_handler))
            .route("/losers", web::get().to(movers::get_losers_handler))
            .route("/indices", web::get().to(indices::get_indices_handler))
            .route("/holders/{symbol}/major", web::get().to(holders::get_major_holders_handler))
            .route("/holders/{symbol}/institutional", web::get().to(holders::get_institutional_holders_handler))
            .route("/holders/{symbol}/mutualfund", web::get().to(holders::get_mutualfund_holders_handler))
            .route("/holders/{symbol}/insider-transactions", web::get().to(holders::get_insider_transactions_handler))
            .route("/holders/{symbol}/insider-purchases", web::get().to(holders::get_insider_purchases_handler))
            .route("/holders/{symbol}/insider-roster", web::get().to(holders::get_insider_roster_handler))
            .route("/analysis/{symbol}/recommendations", web::get().to(analysts::get_recommendations_handler))
            .route("/analysis/{symbol}/upgrades-downgrades", web::get().to(analysts::get_upgrades_downgrades_handler))
            .route("/analysis/{symbol}/price-targets", web::get().to(analysts::get_price_targets_handler))
            .route("/analysis/{symbol}/earnings-estimate", web::get().to(analysts::get_earnings_estimate_handler))
            .route("/analysis/{symbol}/revenue-estimate", web::get().to(analysts::get_revenue_estimate_handler))
            .route("/analysis/{symbol}/earnings-history", web::get().to(analysts::get_earnings_history_handler))
            .route("/sectors", web::get().to(sectors::get_sectors_handler))
            .route("/sectors/symbol/{symbol}", web::get().to(sectors::get_sector_for_symbol_handler))
            .route("/sectors/details/{sector}", web::get().to(sectors::get_sector_details_handler))
            .route("/ws/profile/{symbol}", web::get().to(websocket::profile_handler))
            .route("/ws/quotes", web::get().to(websocket::quotes_handler))
            .route("/ws/indices", web::get().to(websocket::indices_handler))
            .route("/ws/news", web::get().to(websocket::news_handler))
            .route("/ws/sectors", web::get().to(websocket::sectors_handler))
            .route("/ws/movers", web::get().to(websocket::movers_handler))
            .route("/ws/hours", web::get().to(websocket::hours_handler)),
    )
    .route("/ping", web::get().to(health::ping_handler))
    .route("/health", web::get().to(health::health_handler));
}