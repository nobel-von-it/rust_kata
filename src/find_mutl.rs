pub fn find_multiples(n: u32, l: u32) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];
    for i in 1..=l {
        if i % n == 0 {
            v.push(i)
        }
    }
    v
}