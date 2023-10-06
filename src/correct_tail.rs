
pub fn correct_tail(body: &str, tail: char) -> bool {
    body.chars().collect::<Vec<char>>()[body.len()-1] == tail
}

pub fn correct_tail_aaaaaaaaaaaa(b: &str, t: char) -> bool {
    b.ends_with(t)
}