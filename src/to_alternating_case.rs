
pub fn to_alternating_case(s: &str) -> String {
    let mut res = String::new();
    for i in s.split(""){
        if i == i.to_lowercase() {
            res.push_str(&i.to_uppercase())
        } else {
            res.push_str(&i.to_lowercase())
        }
    }
    res
}