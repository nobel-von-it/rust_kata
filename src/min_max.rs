pub fn minimum(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}
pub fn maximum(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}