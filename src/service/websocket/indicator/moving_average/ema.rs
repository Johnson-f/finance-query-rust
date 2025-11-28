/// Calculate Exponential Moving Average (EMA)
/// 
/// # Arguments
/// * `prices` - Slice of price values
/// * `period` - Number of periods for EMA calculation
/// 
/// # Returns
/// * `Some(f64)` - The EMA value if enough data points exist
/// * `None` - If insufficient data points
pub fn calculate(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }
    
    let multiplier = 2.0 / (period as f64 + 1.0);
    
    // Start with SMA of first period values
    let mut ema = prices[0];
    
    // Calculate EMA iteratively
    for &price in prices.iter().skip(1) {
        ema = (price - ema) * multiplier + ema;
    }
    
    Some(ema)
}

