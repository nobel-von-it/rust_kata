pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false
    }
    let mut x = x;
    let mut len = ("1".to_owned() + &*"0".repeat(x.to_string().len() - 1)).parse::<i32>().unwrap();
    while len > 1 {
        let first = x % 10;
        let end = x / len % 10;
        if first != end {
            return false
        }
        x /= 10;
        len /= 100;
    }
    true
}