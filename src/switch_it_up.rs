pub fn switch_it_up(n: usize) -> &'static str {
    if n < 10 && n >= 0 {
        let v = vec!["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Zero"];
        v[n-1]
    } else {
        "One"
    }
}