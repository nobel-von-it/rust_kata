pub fn string_to_array(s: &str) -> Vec<String> {
    s.split(" ").map(|e| e.to_string()).collect()
    // s.split_whitespace().map(str::to_string).collect()
}