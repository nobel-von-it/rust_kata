pub fn digital_root(n: i64) -> i64 {
    let mut res = n;
    while res > 9 {
        res = res.to_string().chars().fold(0, |a: i64, b| a + i64::from(b.to_digit(10).unwrap()));
    }
    res
}

// wtf clever best practices mega mind ++

pub fn digital_root_pp(n: i64) -> i64 {
    (n - 1) % 9 + 1
}