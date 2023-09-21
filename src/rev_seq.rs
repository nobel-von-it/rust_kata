pub fn reverse_seq(n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];
    for i in (1..=n).rev() {
        v.push(i)
    }
    v
}
pub fn rev_seq(n: u32) -> Vec<u32> {
    (1..=n).rev().collect()
}