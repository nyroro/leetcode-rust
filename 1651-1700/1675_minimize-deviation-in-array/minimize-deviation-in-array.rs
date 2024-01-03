
use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut set: BTreeSet<i32> = nums.iter().map(|&x| if x % 2 == 0 { x } else { x * 2 }).collect();
        let mut min_deviation = *set.iter().rev().next().unwrap() - *set.iter().next().unwrap();
        
        while *set.iter().rev().next().unwrap() % 2 == 0 {
            let max_val = *set.iter().rev().next().unwrap();
            set.remove(&max_val);
            set.insert(max_val / 2);
            min_deviation = min_deviation.min(*set.iter().rev().next().unwrap() - *set.iter().next().unwrap());
        }
        
        min_deviation

    }
}
