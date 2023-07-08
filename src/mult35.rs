pub fn solution(num: i32) -> i32 {
    (0..=num-1).filter(|i| i % 3 == 0 || i % 5 == 0).reduce(|a, i| a + i).unwrap()
}