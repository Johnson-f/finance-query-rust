pub mod quotes;
pub mod historical;
pub mod search;
pub mod financials;
pub mod news;
pub mod earnings_transcript;
pub mod logo;

pub use quotes::{get_quotes, get_simple_quotes, get_similar_quotes};
pub use historical::get_historical;
pub use search::search;
pub use financials::get_financial_statement;
pub use news::{scrape_news_for_quote, scrape_general_news};
pub use earnings_transcript::{get_earnings_calls_list, get_earnings_transcript};
#[allow(unused_imports)] // Will be used by other endpoints
pub use logo::get_logo;