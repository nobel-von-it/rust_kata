use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    vec![]
}
pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match hash.get(&(target - *num)) {
            Some(&j) => return vec![i as i32, j],
            None => hash.insert(*num, i as i32),
        };
    }
    todo!()
}