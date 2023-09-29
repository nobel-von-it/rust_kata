pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {return vec![]}
    let mut vec = vec![0;2];
    for i in input {
        if i > 0 {
            vec[0] += 1
        } else {
            vec[1] += i
        }
    }
    vec
}