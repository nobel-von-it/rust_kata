use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    i32,
};

pub fn spin_words(words: &str) -> String {
    words
        .split(' ')
        .map(|e| {
            if e.len() > 4 {
                e.chars().rev().join("")
            } else {
                e.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
        println!("{}^{} = {}", a, v, a ^ v);
        a ^ v
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
    text.to_lowercase()
        .chars()
        .counts()
        .values()
        .filter(|&&i| i > 1)
        .count() as u32
}
pub fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    };
    let mut x = 0;
    let mut y = 0;
    for i in walk {
        match i {
            'n' => y += 1,
            's' => y -= 1,
            'w' => x -= 1,
            'e' => x += 1,
            _ => {}
        }
    }
    x == 0 && y == 0
}
pub fn alphabet_position(text: &str) -> String {
    let alph = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut result = vec![];
    for i in text.to_lowercase().chars() {
        if let Some(el) = alph.find(i) {
            result.push((el + 1).to_string())
        }
    }
    result.join(" ")
}
pub fn persistence(num: u64) -> u64 {
    let mut num = num;
    let mut count = 0;
    while num > 9 {
        num = num
            .to_string()
            .chars()
            .map(|c| c.to_string().parse::<u64>().unwrap())
            .product();
        count += 1;
    }
    count
}
