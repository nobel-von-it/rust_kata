pub fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    let mut prev = arr[0];
    for i in arr {
        let check = i - prev;
        if check > 1 {
            return Some(*i);
        }
        prev = *i;
    }
    None
}