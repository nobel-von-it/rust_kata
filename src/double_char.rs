fn double_char(s: &str) -> String {
    let mut res = String::new();
    for i in s.chars() {
        res.push(i);
        res.push(i);
    }
    res
}
// or
fn double_char2(s: &str) -> String {
    s.chars().map(|c| format!("{}{}", c, c)).collect()
}