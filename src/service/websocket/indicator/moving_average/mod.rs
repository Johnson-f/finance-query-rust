use super::price_buffer::PricePoint;

pub mod sma;
pub mod ema;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum MovingAverageType {
    SMA,
    EMA,
}

/// Calculate moving average based on type (single value for latest period)
pub fn calculate_ma(
    prices: &[PricePoint],
    ma_type: MovingAverageType,
    period: usize,
) -> Option<f64> {
    if prices.len() < period {
        return None;
    }

    let price_values: Vec<f64> = prices.iter().map(|p| p.price).collect();
    
    match ma_type {
        MovingAverageType::SMA => sma::calculate(&price_values, period),
        MovingAverageType::EMA => ema::calculate(&price_values, period),
    }
}

/// Calculate moving average for all historical points (rolling window)
/// Returns a vector of (timestamp, ma_value) pairs
pub fn calculate_ma_series(
    prices: &[PricePoint],
    ma_type: MovingAverageType,
    period: usize,
) -> Vec<(i64, f64)> {
    if prices.len() < period {
        return Vec::new();
    }

    let price_values: Vec<f64> = prices.iter().map(|p| p.price).collect();
    let mut results = Vec::new();
    
    match ma_type {
        MovingAverageType::SMA => {
            // For SMA: calculate rolling average starting from period-th element
            for i in period..=price_values.len() {
                let window = &price_values[i - period..i];
                let sum: f64 = window.iter().sum();
                let ma_value = sum / period as f64;
                results.push((prices[i - 1].timestamp, ma_value));
            }
        }
        MovingAverageType::EMA => {
            // For EMA: calculate iteratively for each point after period
            let multiplier = 2.0 / (period as f64 + 1.0);
            
            // Start with SMA of first period values
            let first_window = &price_values[0..period];
            let initial_sma: f64 = first_window.iter().sum::<f64>() / period as f64;
            let mut ema = initial_sma;
            
            // First EMA value
            results.push((prices[period - 1].timestamp, ema));
            
            // Calculate EMA for remaining points
            for i in period..price_values.len() {
                ema = (price_values[i] - ema) * multiplier + ema;
                results.push((prices[i].timestamp, ema));
            }
        }
    }
    
    results
}

