pub mod kyu8 {}
pub mod kyu5 {
    pub fn beeramid(bonus: i32, price: f32) -> usize {
        let mut bank: i32 = (bonus as f32 / price) as i32;
        if bank <= 0 {
            return 0;
        }

        let mut count = 0usize;
        while bank > 0 {
            count += 1;
            bank -= count.pow(2) as i32;
        }
        if bank < 0 {
            count - 1
        } else {
            count
        }
    }
    pub fn beeramid_one_line(bonus: i32, price: f32) -> usize {
        let count = (bonus as f32 / price) as i32;
        ((1i32)..)
            .scan(0, |acc, x| {
                *acc += x.pow(2);
                Some(*acc)
            })
            .take_while(|&x| x <= count)
            .count()
    }
}
pub mod kyu4 {
    pub fn format_duration(seconds: u64) -> String {
        const YEAR: u64 = 31536000;
        const DAY: u64 = 86400;
        const HOUR: u64 = 3600;
        const MINUTE: u64 = 60;

        let mut seconds = seconds;
        let mut data = String::new();
        let year = seconds / YEAR;
        if year > 0 {
            seconds -= YEAR * year;
            let year_el = format!("{year} year{}", if year > 1 { "s " } else { " " });
            data.push_str(year_el.as_str());
        }
        let day = seconds / DAY;
        if day > 0 {
            seconds -= DAY * day;
            let day_el = format!("{day} day{}", if day > 1 { "s " } else { " " });
            data.push_str(day_el.as_str())
        }
        let hour = seconds / HOUR;
        if hour > 0 {
            seconds -= HOUR * hour;
            let hour_el = format!("{hour} hour{}", if hour > 1 { "s " } else { " " });
            data.push_str(hour_el.as_str())
        }
        let minute = seconds / MINUTE;
        if minute > 0 {
            seconds -= MINUTE * minute;
            let minute_el = format!("{minute} minute{}", if minute > 1 { "s " } else { " " });
            data.push_str(minute_el.as_str())
        }
        if seconds > 0 {
            let seconds_el = format!("{seconds} second{}", if seconds > 1 { "s" } else { "" });
            data.push_str(seconds_el.as_str())
        }
        println!("{}", data);
        data
    }
}
pub mod kyu6 {
    pub fn is_valid_walk(walk: &[char]) -> bool {
        if walk.len() != 10 {
            return false;
        };
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
    use crate::maybe_kata::code;
    use std::time::Instant;

    #[test]
    fn test() {
        let now = Instant::now();
        // assert_eq!(code::kyu6::find_odd_xor(&[1,2,2,3,3,3,4,3,3,3,2,2,1]), 4);
        // assert_eq!(code::kyu7::high_and_low("1 2 3"), "3 1");
        // assert_eq!(code::kyu6::count_bits(1234), 5);
        // assert_eq!(code::kyu6::count_duplicates_one_line("abcaaaa"), 1);
        assert_eq!(code::kyu5::beeramid(5000, 3.0), 16);
        assert_eq!(code::kyu5::beeramid(4, 4.0), 1);
        assert_eq!(code::kyu5::beeramid(9, 2.0), 1);
        assert_eq!(
            code::kyu4::format_duration(3661),
            "1 hour 1 minute 1 second".to_string()
        );
        println!("{:?}", now.elapsed())
    }
}
