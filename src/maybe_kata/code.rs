pub mod kyu8 {
    pub fn square_sum(vec: Vec<i32>) -> i32 {
        vec.iter().map(|x| x.pow(2)).sum()
    }
}
pub mod kyu5 {}
pub mod kyu4 {}
pub mod kyu6 {
    pub fn alphabet_position(text: &str) -> String {
        let alph = String::from("abcdefghijklmnopqrstuvwxyz");
        let mut result = vec![];
        for i in text.to_lowercase().chars() {
            match alph.find(i) {
                Some(el) => result.push((el + 1).to_string()),
                None => {}
            }
        }
        result.join(" ")
    }
    pub fn persistence(num: u64) -> u64 {
        let mut num = num;
        let mut count = 0;
        while num > 9 {
            num = num
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .fold(1, |acc, x| acc * x);
            count += 1;
        }
        count
    }
}
pub mod kyu7 {
    pub fn row_sum_odd_numbers(n: i64) -> i64 {
        n.pow(3)
    }
    pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
        let mut result = vec![];
        for i in data {
            if i.0 >= 55 && i.1 >= 7 {
                result.push("Senior".to_string())
            } else {
                result.push("Open".to_string())
            }
        }
        result
    }
}

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
