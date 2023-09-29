fn positive_sum(slice: &[i32]) -> i32 {
    // slice.iter().filter(|e| e.is_positive()).sum();
    let mut sum = 0;
    for i in slice {
        if i > &0 {
            sum += *i
        }
    }
    sum
}