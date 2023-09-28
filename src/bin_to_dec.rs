pub fn bin_to_decimal(i: &str) -> i32 {
    let res = i32::from_str_radix(i, 2).unwrap();
    res
}