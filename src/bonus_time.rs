pub fn bonus_time(s: u64, bonus: bool) -> String {
    let mut str = String::from("¥");
    if bonus {
       str.push_str(&(s * 10).to_string())
    } else {
        str.push_str(&s.to_string())
    }
    str
}
// using format!

pub fn bonus_time2(s: u64, b: bool) -> String {
    format!("¥{}", if b {s * 10} else {s})
}

