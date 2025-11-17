pub mod quotes;
pub mod historical;
pub mod search;
pub mod news;
pub mod financials;
pub mod earnings_transcript;
pub mod health;
pub mod similar;

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
            .route("/earnings/{symbol}/transcript/{event_id}", web::get().to(earnings_transcript::get_earnings_transcript_handler)),
    )
    .route("/ping", web::get().to(health::ping_handler))
    .route("/health", web::get().to(health::health_handler));
}