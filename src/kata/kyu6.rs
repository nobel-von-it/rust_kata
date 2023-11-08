use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};


pub fn spin_words(words: &str) -> String {
    words.split(" ").map(|e| {
        if e.len() > 4 {
            e.chars().rev().join("")
        } else {
            e.to_string()
        }
    }).collect::<Vec<_>>().join(" ")
}

pub fn find_odd_set(arr: &[i32]) -> i32 {
    let mut set = BTreeSet::new();
    for i in arr {
        if !set.insert(*i) {
            set.remove(i);
        }
    }
    set.pop_first().unwrap()
}
pub fn find_odd_xor(arr: &[i32]) -> i32 {
    //this is very fast solution
    arr.iter().fold(0i32, |a, v| {
        println!("{}^{} = {}", a, v, a^v);
        a^v
    })
}
pub fn count_bits(n: i64) -> u32 {
    format!("{:b}", n).matches('1').count() as u32
}
pub fn count_duplicates(text: &str) -> u32 {
    let mut set = HashSet::new();
    let mut res = 0;
    for i in text.to_lowercase().split("") {
        if !set.insert(i) {
            res += 1
        }
    }
    res
}
pub fn count_duplicates_hashmap(text: &str) -> u32 {
    let mut map: HashMap<char, u16> = HashMap::new();
    let mut res: u32 = 0;
    for i in text.trim().to_lowercase().chars() {
        let v = map.entry(i).or_insert(0);
        *v += 1;
    }
    for (_, v) in map {
        if v > 1 {
            res += 1
        }
    }
    res
}
pub fn count_duplicates_one_line(text: &str) -> u32 {
    text.to_lowercase().chars().counts().values().filter(|&&i| i > 1).count() as u32
}