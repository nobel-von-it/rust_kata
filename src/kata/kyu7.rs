use itertools::Itertools;
use std::str::FromStr;

pub fn neutralise(s1: &str, s2: &str) -> String {
    assert_eq!(s1.len(), s2.len());
    let v1: Vec<&str> = s1.split("").collect();
    let v2: Vec<&str> = s2.split("").collect();
    let mut str = String::new();
    for i in 0..s1.len() + 1 {
        if v1[i] != v2[i] {
            str.push_str("0")
        } else {
            str.push_str(v1[i])
        }
    }
    str
}

pub fn square_digits(n: u64) -> u64 {
    u64::from_str(
        &*n.to_string()
            .chars()
            .map(|e| u32::pow(e.to_digit(10).unwrap(), 2))
            .join(""),
    )
    .unwrap()
}
pub fn high_and_low(nums: &str) -> String {
    let mut min = 10000;
    let mut max = -10000;
    for n in nums.split(" ") {
        match n.parse::<i32>() {
            Ok(v) => {
                if min > v {
                    min = v
                }
                if max < v {
                    max = v
                }
            }
            Err(_) => panic!("parse error"),
        }
    }
    format!("{} {}", max, min)
}
pub fn find_short(s: &str) -> u32 {
    let mut min: u32 = 10000;
    for word in s.split_whitespace() {
        min = min.min(word.len() as u32)
    }
    min
}
pub fn row_sum_odd_numbers(n: i64) -> i64 {
    n.pow(3)
}
pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut result = vec![];
    for i in data {
        if i.0 >= 55 && i.1 > 7 {
            result.push("Senior".to_string())
        } else {
            result.push("Open".to_string())
        }
    }
    result
}
