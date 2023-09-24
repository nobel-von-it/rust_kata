pub fn abbrev_name(name: &str) -> String {
    let v: Vec<&str> = name.split(" ").collect();

    let first = v[0].chars().next().unwrap();
    let second = v[1].chars().next().unwrap();

    format!("{}.{}", first.to_ascii_uppercase(), second.to_ascii_uppercase())
}