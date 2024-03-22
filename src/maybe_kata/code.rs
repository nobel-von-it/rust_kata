pub mod kyu8 {
    pub fn maps(values: &Vec<i32>) -> Vec<i32> {
        values.iter().map(|x| x * 2).collect()
    }
}
pub mod kyu5 {
    pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
        let arr2 = arr.to_vec();
        let mut res_arr = vec![];
        let mut zeros = 0;
        for i in arr2 {
            if i != 0 {
                res_arr.push(i)
            } else {
                zeros += 1
            }
        }
        for _i in 0..zeros {
            res_arr.push(0)
        }
        res_arr
    }
    pub fn move_zeros_smart(arr: &[u8]) -> Vec<u8> {
        let mut res = Vec::with_capacity(arr.len());
        res.extend(arr.iter().filter(|&&x| x != 0));
        res.resize(arr.len(), 0);
        res
    }
}
pub mod kyu4 {}
pub mod kyu6 {
    use std::collections::{BTreeMap, HashMap, HashSet};

    pub fn is_pangram(s: &str) -> bool {
        s.to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<HashSet<char>>()
            .len()
            == 26
    }
    pub fn decode_morse(encoded: &str) -> String {
        let _morse_code: HashMap<&str, String> = HashMap::new();
        let mut result = String::new();
        for word in encoded.split("  ") {
            for ch in word.split_whitespace() {
                match _morse_code.get(ch) {
                    Some(c) => result.push_str(c),
                    None => {}
                }
            }
            result.push(' ');
        }
        result.trim().to_string()
    }
}
pub mod kyu7 {
    use itertools::Itertools;

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
}
