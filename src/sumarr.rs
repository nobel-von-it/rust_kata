pub fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() < 2 { return None; }
    let mut array = arr.to_vec();
    let mut res: i8 = 0;

    array.sort();
    array.reverse();


    for i in 0..arr.len() - 1 {
        res += (array[i] - array[i + 1])
    }

    Some(res)
}

//or best practices
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

pub fn sum_of_differences2(arr: &[i8]) -> Option<i8> {
    match arr.iter().minmax() {
        MinMax(x, y) => Some(y - x),
        _ => None
    }
}