pub fn max_profit(prices: &[f64]) -> f64 {
    let mut min_price = f64::INFINITY;
    let mut best_profit = 0f64;

    for &price in prices.iter() {
        best_profit = best_profit.max(price - min_price);
        min_price = min_price.min(price);
    }

    best_profit
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[5.0, 4.5, 2.5, 1.0], 0.0)]
    #[case(&[310.0, 315.0, 275.0, 295.0, 260.0, 270.0, 290.0, 230.0, 255.0, 250.0], 30.0)]
    #[case(&[15.0, 5.0, 26.0, 13.0, 24.0, 6.0, 19.0], 21.0)]
    #[case(&[10.0, 38.0, 7.0, 36.0, 2.0, 25.0, 27.0], 29.0)]
    fn max_stock_profit(#[case] input: &[f64], #[case] expected: f64) {
        assert_eq!(expected, max_profit(input));
    }
}
