pub fn fake_bin2(s: &str) -> String {
    s.chars().map(|c| if c < '5' {'0'} else {'1'}).collect()
}
pub fn fake_bin(s: &str) -> String {
    let mut r = String::new();
    for i in s.split("") {
        if !i.is_empty() {
            if i > "4" {
                r += "1"
            } else {
                r += "0"
            }
        }
    }
    r
}