pub fn invert(values: &[i32]) -> Vec<i32> {
    let v: Vec<i32> = values.iter().map(|e| -e).collect();
    v
}