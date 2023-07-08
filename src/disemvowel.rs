use itertools::Itertools;

pub fn disemvowel(s: &str) -> String {
    s.chars().filter(|e| !"aeiouAEIOU".find(*e).is_some()).join("")
}