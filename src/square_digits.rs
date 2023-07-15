use std::str::FromStr;
use itertools::Itertools;

pub fn square_digits(n: u64) -> u64 {
    u64::from_str(&*n.to_string()
        .chars()
        .map(|e| u32::pow(e.to_digit(10).unwrap(), 2))
        .join(""))
        .unwrap()
}