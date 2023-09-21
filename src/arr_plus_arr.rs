use std::cmp::max;

pub fn slice_plus_slice(x: &[i32], y: &[i32]) -> i32 {
    let xl = x.len();
    let yl = y.len();
    let max_len = max(xl, yl);
    let mut res = 0;

    for i in 0..max_len {
        if i < yl {
            res += y[i]
        }
        if i < xl {
            res += x[i]
        }
    }
    res
}