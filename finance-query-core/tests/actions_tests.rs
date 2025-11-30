use finance_query_core::*;
use std::sync::Arc;

async fn setup_client() -> YahooFinanceClient {
    let fetch_client = Arc::new(FetchClient::new(None).unwrap());
    let cookie_jar = fetch_client.cookie_jar().clone();
    let auth_manager = Arc::new(YahooAuthManager::new(None, cookie_jar));
    auth_manager.refresh().await.unwrap();
    YahooFinanceClient::new(auth_manager, fetch_client)
}

#[tokio::test]
#[ignore] // Ignore by default since it requires network access
async fn test_get_dividends() {
    let client = setup_client().await;
    let dividends = client.get_dividends("AAPL", "1y").await.unwrap();
    assert!(!dividends.is_empty(), "AAPL should have dividends");

    // Check dividend structure
    let div = &dividends[0];
    assert!(div.amount > 0.0);
}

#[tokio::test]
#[ignore] // Ignore by default since it requires network access
async fn test_get_splits() {
    let client = setup_client().await;
    let splits = client.get_splits("AAPL", "max").await.unwrap();
    // AAPL has had splits in its history
    assert!(!splits.is_empty());
}

#[tokio::test]
#[ignore] // Ignore by default since it requires network access
async fn test_get_actions() {
    let client = setup_client().await;
    let actions = client.get_actions("AAPL", "5y").await.unwrap();

    assert!(!actions.is_empty());
    assert!(!actions.dividends.is_empty());
    assert!(actions.total_dividends() > 0.0);
}

#[tokio::test]
#[ignore] // Ignore by default since it requires network access
async fn test_capital_gains_etf() {
    let client = setup_client().await;
    let actions = client.get_actions("SPY", "5y").await.unwrap();
    // SPY is an ETF and may have capital gains
    // Just verify it doesn't error
    assert!(actions.capital_gains.len() >= 0);
}
