use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub fn neutralise(s1: &str, s2: &str) -> String {
    assert_eq!(s1.len(), s2.len());
    let v1: Vec<&str> = s1.split("").collect();
    let v2: Vec<&str> = s2.split("").collect();
    let mut str = String::new();
    for i in 0..s1.len() + 1 {
        if v1[i] != v2[i] {
            str.push('0')
        } else {
            str.push_str(v1[i])
        }
    }
    str
}

pub fn square_digits(n: u64) -> u64 {
    u64::from_str(
        &n.to_string()
            .chars()
            .map(|e| u32::pow(e.to_digit(10).unwrap(), 2))
            .join(""),
    )
    .unwrap()
}
pub fn high_and_low(nums: &str) -> String {
    let mut min = 10000;
    let mut max = -10000;
    for n in nums.split(' ') {
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
pub fn word_pattern(word: &str) -> String {
    let mut res = String::new();
    let mut cryptomap = HashMap::new();
    let mut counter = 0;

    for c in word.to_lowercase().chars() {
        match cryptomap.insert(c, counter) {
            None => {
                counter += 1;
            }
            Some(v) => {
                cryptomap.insert(c, v).unwrap();
            }
        };
        match cryptomap.get(&c) {
            None => {}
            Some(v) => res.push_str(&format!("{v}.")),
        }
    }
    res.pop();

    res
}

pub fn wall_paper(l: f64, w: f64, h: f64) -> String {
    const NUMBERS: [&str; 21] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    if l == 0.0 || w == 0.0 || h == 0.0 {
        return NUMBERS[0].to_string();
    }
    let p = 2.0 * l * h + 2.0 * w * h;
    let pp = p + p / 100.0 * 15.0;

    NUMBERS[(pp / 5.2).ceil() as usize].to_string()
}
pub fn get_sum(a: i64, b: i64) -> i64 {
    let min = a.min(b);
    let max = a.max(b);
    let mut result: i64 = 0;
    for i in min..=max {
        result += i
    }
    result
}
pub fn validate_pin(pin: &str) -> bool {
    match pin.len() {
        4 => pin.matches(char::is_numeric).count() == 4,
        6 => pin.matches(char::is_numeric).count() == 6,
        _ => false,
    }
}
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
pub fn descending_order(x: u64) -> u64 {
    let mut ch: Vec<String> = x.to_string().chars().map(|c| c.to_string()).collect();
    ch.sort();
    ch.reverse();
    ch.join("").parse::<u64>().unwrap()
}
