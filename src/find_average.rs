pub fn find_average(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0.0
    }
    let sum: f64 = slice.iter().sum();
    sum / slice.len() as f64
}
pub fn find_average2(arr: &[f64]) -> f64 {
    match arr.len() {
        0 => 0.,
        n => arr.iter().sum::<f64>() / n as f64
    }
}