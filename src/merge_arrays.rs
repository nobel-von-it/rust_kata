

pub fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let l1 = arr1.len();
    let l2 = arr2.len();
    for i in arr1 {
        if !v.contains(i) {
            v.push(*i)
        }
    }
    for i in arr2 {
        if !v.contains(i) {
            v.push(*i)
        }
    }
    v.sort();
    v
}
pub fn merge_arrs(a1: &[i32], a2: &[i32]) -> Vec<i32> {
    let mut v = [a1, a2].concat();
    v.sort();
    v.dedup();
    v
}