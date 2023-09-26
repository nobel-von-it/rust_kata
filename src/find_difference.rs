pub fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let va = a.into_iter().fold(1, |acc, i| acc * i);
    let vb = b.into_iter().fold(1, |acc, i| acc * i);
    (va - vb).abs()
}