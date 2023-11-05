pub mod kyu8 {

}
pub mod kyu6 {
    use std::collections::{BTreeSet, HashMap, HashSet};
    use itertools::Itertools;

    pub fn find_odd_set(arr: &[i32]) -> i32 {
        let mut set = BTreeSet::new();
        for i in arr {
            if !set.insert(*i) {
                set.remove(i);
            }
        }
        set.pop_first().unwrap()
    }
    pub fn find_odd_xor(arr: &[i32]) -> i32 {
        //this is very fast solution
        arr.iter().fold(0i32, |a, v| {
            println!("{}^{} = {}", a, v, a^v);
            a^v
        })
    }
    pub fn count_bits(n: i64) -> u32 {
        format!("{:b}", n).matches('1').count() as u32
    }
    pub fn count_duplicates(text: &str) -> u32 {
        let mut set = HashSet::new();
        let mut res = 0;
        for i in text.to_lowercase().split("") {
            if !set.insert(i) {
                res += 1
            }
        }
        res
    }
    pub fn count_duplicates_hashmap(text: &str) -> u32 {
        let mut map: HashMap<char, u16> = HashMap::new();
        let mut res: u32 = 0;
        for i in text.trim().to_lowercase().chars() {
            let v = map.entry(i).or_insert(0);
            *v += 1;
        }
        for (_, v) in map {
            if v > 1 {
                res += 1
            }
        }
        res
    }
    pub fn count_duplicates_one_line(text: &str) -> u32 {
        text.to_lowercase().chars().counts().values().filter(|&&i| i > 1).count() as u32
    }
}
pub mod kyu7 {

    pub fn high_and_low(nums: &str) -> String {
        let mut min = 10000;
        let mut max = -10000;
        for n in nums.split(" ") {
            match n.parse::<i32>() {
                Ok(v) => {
                    if min > v {
                        min = v
                    }
                    if max < v {
                        max = v
                    }
                }
                Err(_) => panic!("parse error")
            }
        }
        format!("{} {}", max, min)
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