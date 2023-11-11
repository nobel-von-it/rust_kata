pub mod kyu8 {}
pub mod kyu5 {}
pub mod kyu4 {}
pub mod kyu6 {}
pub mod kyu7 {}

pub mod test {
    use crate::maybe_kata::code;
    use std::time::Instant;

    #[test]
    fn test() {
        let now = Instant::now();
        // assert_eq!(code::kyu6::find_odd_xor(&[1,2,2,3,3,3,4,3,3,3,2,2,1]), 4);
        // assert_eq!(code::kyu7::high_and_low("1 2 3"), "3 1");
        // assert_eq!(code::kyu6::count_bits(1234), 5);
        // assert_eq!(code::kyu6::count_duplicates_one_line("abcaaaa"), 1);
        // assert_eq!(code::kyu5::beeramid(5000, 3.0), 16);
        // assert_eq!(code::kyu5::beeramid(4, 4.0), 1);
        // assert_eq!(code::kyu5::beeramid(9, 2.0), 1);
        // assert_eq!(
        //     code::kyu4::format_duration(3661),
        //     "1 hour, 1 minute and 1 second"
        // );
        println!("{:?}", now.elapsed())
    }
}
