use itertools::Itertools;

pub fn spin_words(words: &str) -> String {
    words.split(" ").map(|e| {
        if e.len() > 4 {
            e.chars().rev().join("")
        } else {
            e.to_string()
        }
    }).collect::<Vec<_>>().join(" ")
}