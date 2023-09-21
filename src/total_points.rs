use std::cmp::Ordering;

pub fn points(g: &[String]) -> u32 {
    let mut r: u32 = 0;
    r
}
pub fn points2(g: &[String]) -> u32 {
    g.iter().map(|e| {
        let (l, r) = e.split_once(':').unwrap();
        match l.cmp(r) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 3,
        }
    }).sum()
}