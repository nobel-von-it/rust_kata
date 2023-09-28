pub fn hello(name: &str) -> String {
    let mut res = String::from("Hello, ");
    if name.len() > 1 {
        res.push_str(&name[..1].to_ascii_uppercase());
        res.push_str(&name[1..].to_ascii_lowercase());
        res.push_str("!");
        res
    } else {
        "Hello, World!".to_string()
    }
}