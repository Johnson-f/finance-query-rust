/// Calculate Simple Moving Average (SMA)
///
/// # Arguments
/// * `prices` - Slice of price values
/// * `period` - Number of periods to average
///
/// # Returns
/// * `Some(f64)` - The SMA value if enough data points exist
/// * `None` - If insufficient data points
pub fn calculate(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }

    let sum: f64 = prices[prices.len() - period..].iter().sum();
    Some(sum / period as f64)
}
