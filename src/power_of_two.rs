fn powers_of_two(n: u8) -> Vec<u128> {
    // let mut v = vec![]
    (0..=n as u128).map(|e| 2u128.pow(e as u32)).collect()
}
