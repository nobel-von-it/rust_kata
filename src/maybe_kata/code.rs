pub mod kyu8 {

}
pub mod kyu6 {
    pub fn is_valid_walk(walk: &[char]) -> bool {
        if walk.len() != 10 {return false};
        let mut x = 0;
        let mut y = 0;
        for i in walk {
            match i {
                'n' => y += 1,
                's' => y -= 1,
                'w' => x -= 1,
                'e' => x += 1,
                _ => {}
            }
        }
        x == 0 && y == 0
    }
}
pub mod kyu7 {
    pub fn find_short(s: &str) -> u32 {
        let mut min: u32 = 10000;
        for word in s.split_whitespace() {
            min = min.min(word.len() as u32)
        }
        min
    }
}
pub mod test {
    use std::time::Instant;
    use crate::maybe_kata::code;

    #[test]
    fn test() {
        let now = Instant::now();
        assert_eq!(code::kyu6::find_odd_xor(&[1,2,2,3,3,3,4,3,3,3,2,2,1]), 4);
        // assert_eq!(code::kyu7::high_and_low("1 2 3"), "3 1");
        // assert_eq!(code::kyu6::count_bits(1234), 5);
        // assert_eq!(code::kyu6::count_duplicates_one_line("abcaaaa"), 1);
        println!("{:?}", now.elapsed())
    }
}