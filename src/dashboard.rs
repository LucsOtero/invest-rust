pub fn calculate_total(investments: Vec<(f64, f64)>) -> f64 {
    investments.iter().map(|(q, p)| q * p).sum()
}
